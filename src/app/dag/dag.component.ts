import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/models/datapoint';
import { invoke } from '@tauri-apps/api';
import { SvgService } from '../services/svg.service';
import { DagViewService } from '../services/dag-view.service';
import { Subscription } from 'rxjs';
import { Color } from 'src/models/color';
import * as d3 from 'd3';

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
  
  @ViewChild('vizualization_div') vizualization_div: ElementRef<HTMLDivElement>;
  private d3_svg: any;
  private width: number;
  private height: number;
  private x: any;
  private y: any;
  private xAxis: any;
  private yAxis: any;

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

  constructor(private svg_service: SvgService, private dagview_service: DagViewService){
    this.datapoints_subscription = this.dagview_service.datapoints$.subscribe(value => {
      this.subscribed_datapoints = value;
      // this.drawDataPoints();
    });
  }
  
  ngAfterViewInit(){
    // this.width = 1280;
    this.width = 800;
    // this.height = 720;
    this.height = 800;
  
    this.d3_svg = d3.select(this.vizualization_div.nativeElement)
      .append('svg')
        .attr('width', this.width)
        .attr('height', this.height);

    let x = d3.scaleLinear()
      .domain([0, 1])
      .range([0, this.width]);

    let x_axis = d3.axisBottom(x);

    this.d3_svg.append('g')
      .attr('transform', `translate(0,${0})`)
      .call(x_axis);

    let y = d3.scaleLinear()
      .domain([0, 1])
      .range([this.height, 0]);

    let y_axis = d3.axisLeft(y);

    this.d3_svg.append('g')
      .attr('transform', `translate(0,${0})`)
      .call(y_axis);

    this.x = x;
    this.y = y;
    this.xAxis = x_axis;
    this.yAxis = y_axis;

    this.updateChart();
  }

  updateChart() {
    // console.log(this.svg_element.nativeElement.clientHeight);
  }

  public onResize(event) {
    this.width = this.dagWindow.nativeElement.clientWidth;
    this.height = this.dagWindow.nativeElement.clientHeight;

    this.updateChart();
  }

  ngOnDestroy(){
    this.datapoints_subscription.unsubscribe();
  }

  // private scaleToFitCanvas(x: number, y:number, radius: number){
  //   // x goes from -1 to 1, and y goes from -1 to 1
  //   let scaled_x = (x + 1) * this.canvas.nativeElement.width / 2;
  //   let scaled_y = (y + 1) * this.canvas.nativeElement.height / 2;
  //   let scaled_radius = radius;

  //   return {x: scaled_x, y: scaled_y, radius: scaled_radius};
  // }

//   private drawDataPoints(){
//     this.context.save();

//     this.context.translate(this.totalDx, this.totalDy);
//     this.context.scale(this.scale, this.scale);
//     this.canvas_service.drawGrid(this.canvas, this.maximum_dx*4, this.maximum_dy*4, 10);
    
//     for (let i = 0; i < this.subscribed_datapoints.length; i++){
//       let datapoint = this.subscribed_datapoints[i];
//       let scaled_datapoint = this.scaleToFitCanvas(datapoint.x, datapoint.y, datapoint.size);
//       let color: Color = {r: datapoint.r, g: datapoint.g, b: datapoint.b};
//       this.canvas_service.drawCircle(this.canvas, scaled_datapoint.x, scaled_datapoint.y, scaled_datapoint.radius/2, color);
//     }
  
//     this.context.restore();
//   } 



//   public mouseDownHandler(e) {
//     this.isDragging = true;
//     this.previousMousePosition = {
//       x: e.clientX,
//       y: e.clientY
//     };
//   }
  
//   public mouseMoveHandler(e) {
//     if (this.isDragging == true) {
//       const dx = e.clientX - this.previousMousePosition.x;
//       const dy = e.clientY - this.previousMousePosition.y;
  
//       // Update the total translation
//       let temp_total_dx = (this.totalDx + dx);
//       let temp_total_dy = (this.totalDy + dy);
  
//       // Update maximum dx and dy based on the current scale
//       this.maximum_dx = this.canvas.nativeElement.width * this.scale * this.maximum_dislocation_multiplier;
//       this.maximum_dy = this.canvas.nativeElement.height * this.scale * this.maximum_dislocation_multiplier;
      
//       // console.log(this.scale);
//       if ((temp_total_dx / this.scale) > this.maximum_dx / this.scale) {return;} // Left side block
//       if (temp_total_dx / this.scale < -this.maximum_dx) {return;} // Right side block
//       if (temp_total_dy / this.scale < -this.maximum_dy) {;return;} // Bottom side block
//       if ((temp_total_dy / this.scale) > this.maximum_dy / this.scale) {return;} // Top side block
  
//       this.totalDx = temp_total_dx;
//       this.totalDy = temp_total_dy;
  
//       // console.log(this.totalDx, this.totalDy);
  
//       // Update the mouse position
//       this.previousMousePosition = {
//           x: e.clientX,
//           y: e.clientY
//         };
    
//         this.canvas_service.clearCanvas(this.canvas);
//         this.drawDataPoints();
//     }
//   }
  
//   public mouseUpHandler() {
//     this.isDragging = false;
//   }
  
//   public wheelHandler(event: WheelEvent) {
//     event.preventDefault();
  
//     // Calculate the mouse position in canvas coordinates
//     const rect = this.canvas.nativeElement.getBoundingClientRect();
//     const mouseX = (event.clientX - rect.left - this.totalDx) / this.scale;
//     const mouseY = (event.clientY - rect.top - this.totalDy) / this.scale;
  
//     // Update the scale
//     const oldScale = this.scale;
//     if (event.deltaY < 0) { // Zoom in
//       this.scale /= this.scaleMultiplier;

//     } else { // Zoom out
//       let temp_scale = this.scale * this.scaleMultiplier;
//       if (temp_scale < this.minimum_scale){ return;} // Minimum amount of zoom
//       this.scale = temp_scale;
//     }
  
//     // Update the translation to center the zoom effect at the mouse position
//     this.totalDx -= mouseX * (this.scale - oldScale);
//     this.totalDy -= mouseY * (this.scale - oldScale);
  
//     // Redraw the canvas
//     this.canvas_service.clearCanvas(this.canvas);
//     this.drawDataPoints();

//     // console.log(this.scale);
//   }

}
