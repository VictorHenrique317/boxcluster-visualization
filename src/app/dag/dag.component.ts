import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { Coordinate } from 'src/models/coordinate';
import { invoke } from '@tauri-apps/api';

// https://angular.io/guide/template-syntax

@Component({
  selector: 'app-dag',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './dag.component.html',
  styleUrls: ['./dag.component.scss']
})
export class DagComponent implements AfterViewInit{
  @ViewChild('canvas') canvas: ElementRef<HTMLCanvasElement>;
  private context: CanvasRenderingContext2D;
  private totalDx = 0;
  private totalDy = 0;
  private maximum_dx = 0;
  private maximum_dy = 0;

  private scale: number = 1.0;
  private scaleMultiplier: number = 0.8;
  private minimum_scale: number = 0.5;

  @ViewChild('dagWindow') dagWindow: ElementRef<HTMLBodyElement>;

  public coordinates: Array<Coordinate>;

  // Variables to keep track of the mouse position and left-button status 
  private isDragging = false;
  private previousMousePosition = {
    x: 0,
    y: 0
  };

  constructor(){}
  
  ngAfterViewInit(){
    this.fixCanvasRendering();
    this.maximum_dx = this.canvas.nativeElement.width * 2;
    this.maximum_dy = this.canvas.nativeElement.height * 2;

    let coord1: Coordinate = {x: 0, y: 0, radius: 10};
    let coord2: Coordinate = {x: -1, y: -1, radius: 10};
    let coord3: Coordinate = {x: -1, y: 1, radius: 10};
    let coord4: Coordinate = {x: 1, y: -1, radius: 10};
    let coord5: Coordinate = {x: 1, y: 1, radius: 10};
    this.coordinates = [coord1, coord2, coord3, coord4, coord5];

    // this.getCoordinates();
    this.drawCoordinates();
  }

  private fixCanvasRendering() {
    const originalWidth = this.canvas.nativeElement.width;
    const originalHeight = this.canvas.nativeElement.height;
  
    this.canvas.nativeElement.width = this.dagWindow.nativeElement.clientWidth;
    this.canvas.nativeElement.height = this.dagWindow.nativeElement.clientHeight;
  
    this.context = this.canvas.nativeElement.getContext("2d");
    let ratio = Math.min(
      this.canvas.nativeElement.clientWidth / originalWidth,
      this.canvas.nativeElement.clientHeight / originalHeight
    );
    
    this.context.scale(1, 1);
  }

  private scaleToFitCanvas(x: number, y:number, radius: number){
    // x goes from -1 to 1, and y goes from -1 to 1
    let scaled_x = (x + 1) * this.canvas.nativeElement.width / 2;
    let scaled_y = (y + 1) * this.canvas.nativeElement.height / 2;
    let scaled_radius = radius;

    return {x: scaled_x, y: scaled_y, radius: scaled_radius};
  }
  
  private drawCircle(x:number, y:number, radius:number){
    let scaled = this.scaleToFitCanvas(x, y, radius);
    this.context.beginPath(); // Start a new path
    this.context.arc(scaled.x,scaled.y, scaled.radius, 0, Math.PI*2, false);
    this.context.fill();
  }

  private drawCoordinates(){
    this.context.save();

    this.context.translate(this.totalDx, this.totalDy);
    this.context.scale(this.scale, this.scale);
    this.drawGrid();
    
    for (let i = 0; i < this.coordinates.length; i++){
      let coordinate = this.coordinates[i];
      this.drawCircle(coordinate.x, coordinate.y, coordinate.radius);
    }
  
    this.context.restore();
  } 

  private drawGrid() {
    const gridSize = 50;
  
    // Draw vertical lines
    for (let x = -this.maximum_dx*4; x <= this.maximum_dx*8; x += gridSize) {
      this.context.moveTo(x, -this.maximum_dy);
      this.context.lineTo(x, this.maximum_dy*2);
    }
  
    // Draw horizontal lines
    for (let y = -this.maximum_dy*4; y <= this.maximum_dy*8; y += gridSize) {
      this.context.moveTo(-this.maximum_dx, y);
      this.context.lineTo(this.maximum_dx*2, y);
    }
  
    // Set the line color and stroke the lines
    this.context.strokeStyle = 'lightgrey';
    this.context.stroke();
  }

  private clearCanvas(){
    this.context.setTransform(1, 0, 0, 1, 0, 0);
    this.context.clearRect(0, 0, this.canvas.nativeElement.width, this.canvas.nativeElement.height);
  }

  public onResize(event) {
    this.clearCanvas();
    this.canvas.nativeElement.width = this.dagWindow.nativeElement.clientWidth;
    this.canvas.nativeElement.height = this.dagWindow.nativeElement.clientHeight;
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
      let maximum_dx = this.canvas.nativeElement.width * this.scale * 2;
      let maximum_dy = this.canvas.nativeElement.height * this.scale * 2;
      
      console.log(this.scale);
      if ((temp_total_dx / this.scale) > maximum_dx / this.scale) {return;} // Left side block
      if (temp_total_dx / this.scale < -maximum_dx) {return;} // Right side block
      if (temp_total_dy / this.scale < -maximum_dy) {;return;} // Bottom side block
      if ((temp_total_dy / this.scale) > maximum_dy / this.scale) {return;} // Top side block
  
      this.totalDx = temp_total_dx;
      this.totalDy = temp_total_dy;
  
      // console.log(this.totalDx, this.totalDy);
  
      // Update the mouse position
      this.previousMousePosition = {
          x: e.clientX,
          y: e.clientY
        };
    
        this.clearCanvas();
        this.drawCoordinates();
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
    this.clearCanvas();
    this.drawCoordinates();

    console.log(this.scale);
  }

  public getCoordinates(){
    invoke("getCoordinates").then((result: Array<Coordinate>) =>{
      this.coordinates = result;
      console.log(this.coordinates);
    });
  }

}
