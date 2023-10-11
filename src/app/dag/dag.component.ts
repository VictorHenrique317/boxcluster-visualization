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
  @ViewChild('dagWindow') dagWindow: ElementRef<HTMLBodyElement>;
  private context: CanvasRenderingContext2D;

  public coordinates: Array<Coordinate>;

  constructor(){}

  ngAfterViewInit(){
    this.drawTestSquare();

    // this.drawCircle(0, 690, 10);
    // this.getCoordinates();
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
    
    // Scale the context by the calculated ratio
    this.context.scale(1, 1);
  }

  private drawTestSquare(){
    this.fixCanvasRendering();
    this.drawCircle(0, 0, 10);
    this.drawCircle(-1, -1, 10);
    this.drawCircle(-1, 1, 10);
    this.drawCircle(1, -1, 10);
    this.drawCircle(1, 1, 10);
  }

  public onResize(event) {
    // Clear the canvas
    this.context.clearRect(0, 0, this.canvas.nativeElement.width, this.canvas.nativeElement.height);
    this.canvas.nativeElement.width = this.dagWindow.nativeElement.clientWidth;
    this.canvas.nativeElement.height = this.dagWindow.nativeElement.clientHeight;
    
    console.log(this.dagWindow.nativeElement.clientWidth);
    console.log(this.dagWindow.nativeElement.clientHeight);
    console.log("------")
    this.drawTestSquare();
  }

  private scaleToFitCanva(x: number, y:number, radius: number){
    // x goes from -1 to 1, and y goes from -1 to 1
    let scaled_x = (x + 1) * this.canvas.nativeElement.width / 2;
    let scaled_y = (y + 1) * this.canvas.nativeElement.height / 2;
    let scaled_radius = radius;

    return {x: scaled_x, y: scaled_y, radius: scaled_radius};
  }
  
  private drawCircle(x:number, y:number, radius:number){
    let scaled = this.scaleToFitCanva(x, y, radius);
    this.context.arc(scaled.x,scaled.y, scaled.radius, 0, Math.PI*2, false);
    this.context.fill();
  }

  public getCoordinates(){
    invoke("getCoordinates").then((result: Array<Coordinate>) =>{
      this.coordinates = result;
      console.log(this.coordinates);
    });
  }

}
