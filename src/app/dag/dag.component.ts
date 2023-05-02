import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';

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

  constructor(){}

  ngAfterViewInit(){
    this.fixCanvasRendering();
    this.drawCircle(946, 0, 10);
    // this.drawCircle(0, 723, 10);
    this.drawCircle(0, 690, 10);
  }

  private fixCanvasRendering(){
    let windowWidth = window.innerWidth;
    let windowHeight = window.innerHeight;

    // console.log(this.dagWindow.nativeElement.clientWidth); // 
    // console.log(this.dagWindow.nativeElement.clientHeight);

    // console.log(windowWidth);
    // console.log(windowHeight);
    const originalHeight = this.canvas.nativeElement.height;
    const originalWidth = this.canvas.nativeElement.width;

    // const dimensions = this.getObjectFitSize(
    //   true,
    //   this.canvas.nativeElement.clientWidth,
    //   this.canvas.nativeElement.clientHeight,
    //   this.canvas.nativeElement.width,
    //   this.canvas.nativeElement.height
    // );

    // this.canvas.nativeElement.width = dimensions.width;
    // this.canvas.nativeElement.height = dimensions.height;
    this.canvas.nativeElement.width = this.dagWindow.nativeElement.clientWidth;
    this.canvas.nativeElement.height = this.dagWindow.nativeElement.clientHeight;

    this.context = this.canvas.nativeElement.getContext("2d");
    let ratio = Math.min(
      this.canvas.nativeElement.clientWidth / originalWidth,
      this.canvas.nativeElement.clientHeight / originalHeight
    );
    // this.context.scale(ratio, ratio);
    this.context.scale(1, 1);
  }

  private drawCircle(x:number, y:number, radius:number){
    this.context.arc(x,y, radius, 0, Math.PI*2, false);
    this.context.fill();
  }

  private getObjectFitSize(contains /* true = contain, false = cover */, containerWidth, containerHeight, width, height){
    var doRatio = width / height;
    var cRatio = containerWidth / containerHeight;
    var targetWidth = 0;
    var targetHeight = 0;
    var test = contains ? (doRatio > cRatio) : (doRatio < cRatio);

    if (test) {
        targetWidth = containerWidth;
        targetHeight = targetWidth / doRatio;
    } else {
        targetHeight = containerHeight;
        targetWidth = targetHeight * doRatio;
    }

    return {
        width: targetWidth,
        height: targetHeight,
        x: (containerWidth - targetWidth) / 2,
        y: (containerHeight - targetHeight) / 2
    };
}

}
