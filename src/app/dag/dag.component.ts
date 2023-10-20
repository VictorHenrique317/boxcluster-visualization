import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/models/datapoint';
import { invoke } from '@tauri-apps/api';
import { CanvasService } from '../services/canva.service';
import { DagViewService } from '../services/dag-view.service';
import { Subscription } from 'rxjs';
import { Color } from 'src/models/color';

// https://angular.io/guide/template-syntax

@Component({
  selector: 'app-dag',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './dag.component.html',
  styleUrls: ['./dag.component.scss']
})
export class DagComponent implements AfterViewInit{
  @ViewChild('dagWindow') dagWindow: ElementRef<HTMLBodyElement>;
  private datapoints_subscription: Subscription;
  private subscribed_datapoints: Array<DataPoint>;

  @ViewChild('canvas') canvas: ElementRef<HTMLCanvasElement>;
  private context: CanvasRenderingContext2D;
  private totalDx = 0;
  private totalDy = 0;
  private maximum_dislocation_multiplier = 0.5;
  private maximum_dx: number;
  private maximum_dy: number;

  private scale: number = 1.0;
  private scaleMultiplier: number = 0.8;
  private minimum_scale: number = 0.8;

  // Variables to keep track of the mouse position and left-button status 
  private isDragging = false;
  private previousMousePosition = {
    x: 0,
    y: 0
  };

  constructor(private canvas_service: CanvasService, private dagview_service: DagViewService){
    this.datapoints_subscription = this.dagview_service.datapoints$.subscribe(value => {
      this.subscribed_datapoints = value;
      this.drawDataPoints();
    });
  }
  
  ngAfterViewInit(){
    this.context = this.canvas.nativeElement.getContext("2d");
    this.canvas_service.fixCanvasRendering(this.dagWindow, this.canvas);

    this.dagview_service.updateDataPoints();
  }

  ngOnDestroy(){
    this.datapoints_subscription.unsubscribe();
  }

  private scaleToFitCanvas(x: number, y:number, radius: number){
    // x goes from -1 to 1, and y goes from -1 to 1
    let scaled_x = (x + 1) * this.canvas.nativeElement.width / 2;
    let scaled_y = (y + 1) * this.canvas.nativeElement.height / 2;
    let scaled_radius = radius;

    return {x: scaled_x, y: scaled_y, radius: scaled_radius};
  }

  private drawDataPoints(){
    this.context.save();

    this.context.translate(this.totalDx, this.totalDy);
    this.context.scale(this.scale, this.scale);
    this.canvas_service.drawGrid(this.canvas, this.maximum_dx*4, this.maximum_dy*4);
    
    for (let i = 0; i < this.subscribed_datapoints.length; i++){
      let datapoint = this.subscribed_datapoints[i];
      let scaled_datapoint = this.scaleToFitCanvas(datapoint.x, datapoint.y, datapoint.size);
      let color: Color = {r: datapoint.r, g: datapoint.g, b: datapoint.b};
      this.canvas_service.drawCircle(this.canvas, scaled_datapoint.x, scaled_datapoint.y, scaled_datapoint.radius, color);
    }
  
    this.context.restore();
  } 

  public onResize(event) {
    this.canvas_service.onResize(this.canvas, this.dagWindow);
  }

  public mouseDownHandler(e) {
    this.isDragging = true;
    this.previousMousePosition = {
      x: e.clientX,
      y: e.clientY
    };
  }
  
  public mouseMoveHandler(e) {
    if (this.isDragging == true) {
      const dx = e.clientX - this.previousMousePosition.x;
      const dy = e.clientY - this.previousMousePosition.y;
  
      // Update the total translation
      let temp_total_dx = (this.totalDx + dx);
      let temp_total_dy = (this.totalDy + dy);
  
      // Update maximum dx and dy based on the current scale
      this.maximum_dx = this.canvas.nativeElement.width * this.scale * this.maximum_dislocation_multiplier;
      this.maximum_dy = this.canvas.nativeElement.height * this.scale * this.maximum_dislocation_multiplier;
      
      // console.log(this.scale);
      if ((temp_total_dx / this.scale) > this.maximum_dx / this.scale) {return;} // Left side block
      if (temp_total_dx / this.scale < -this.maximum_dx) {return;} // Right side block
      if (temp_total_dy / this.scale < -this.maximum_dy) {;return;} // Bottom side block
      if ((temp_total_dy / this.scale) > this.maximum_dy / this.scale) {return;} // Top side block
  
      this.totalDx = temp_total_dx;
      this.totalDy = temp_total_dy;
  
      // console.log(this.totalDx, this.totalDy);
  
      // Update the mouse position
      this.previousMousePosition = {
          x: e.clientX,
          y: e.clientY
        };
    
        this.canvas_service.clearCanvas(this.canvas);
        this.drawDataPoints();
    }
  }
  
  public mouseUpHandler() {
    this.isDragging = false;
  }
  
  public wheelHandler(event: WheelEvent) {
    event.preventDefault();
  
    // Calculate the mouse position in canvas coordinates
    const rect = this.canvas.nativeElement.getBoundingClientRect();
    const mouseX = (event.clientX - rect.left - this.totalDx) / this.scale;
    const mouseY = (event.clientY - rect.top - this.totalDy) / this.scale;
  
    // Update the scale
    const oldScale = this.scale;
    if (event.deltaY < 0) { // Zoom in
      this.scale /= this.scaleMultiplier;

    } else { // Zoom out
      let temp_scale = this.scale * this.scaleMultiplier;
      if (temp_scale < this.minimum_scale){ return;} // Minimum amount of zoom
      this.scale = temp_scale;
    }
  
    // Update the translation to center the zoom effect at the mouse position
    this.totalDx -= mouseX * (this.scale - oldScale);
    this.totalDy -= mouseY * (this.scale - oldScale);
  
    // Redraw the canvas
    this.canvas_service.clearCanvas(this.canvas);
    this.drawDataPoints();

    // console.log(this.scale);
  }

}
