import { ElementRef, Injectable } from '@angular/core';
import { Color } from 'src/models/color';

@Injectable({
  providedIn: 'root'
})
export class SvgService {

  constructor() { }

  // public fixCanvasRendering(parent_window: ElementRef, canvas: ElementRef<HTMLCanvasElement>) {
  //   canvas.nativeElement.width = parent_window.nativeElement.clientWidth;
  //   canvas.nativeElement.height = parent_window.nativeElement.clientHeight;
  // }

  // public clearCanvas(canvas: ElementRef<HTMLCanvasElement>){
  //   canvas.nativeElement.getContext("2d").setTransform(1, 0, 0, 1, 0, 0);
  //   canvas.nativeElement.getContext("2d").clearRect(0, 0, canvas.nativeElement.width, canvas.nativeElement.height);
  // }

  // public onResize(svg: ElementRef<SVGElement>, parent_window: ElementRef<HTMLBodyElement>) {
  //   // this.clearCanvas(canvas);
  //   svg.nativeElement.setAttribute('width', parent_window.nativeElement.clientWidth.toString());
  //   svg.nativeElement.setAttribute('height', parent_window.nativeElement.clientHeight.toString());
  // }

  // public drawGrid(canvas: ElementRef<HTMLCanvasElement>, maximum_dx: number, maximum_dy: number, grid_size:number) {
  //   // Draw vertical lines
  //   for (let x = -maximum_dx; x <= maximum_dx*2; x += grid_size) {
  //     canvas.nativeElement.getContext("2d").moveTo(x, -maximum_dy);
  //     canvas.nativeElement.getContext("2d").lineTo(x, maximum_dy*2);
  //   }
  
  //   // Draw horizontal lines
  //   for (let y = -maximum_dy; y <= maximum_dy*2; y += grid_size) {
  //     canvas.nativeElement.getContext("2d").moveTo(-maximum_dx, y);
  //     canvas.nativeElement.getContext("2d").lineTo(maximum_dx*2, y);
  //   }
  
  //   // Set the line color and stroke the lines
  //   canvas.nativeElement.getContext("2d").strokeStyle = 'lightgrey';
  //   canvas.nativeElement.getContext("2d").stroke();
  // }

  // public drawCircle(canvas: ElementRef<HTMLCanvasElement>, x:number, y:number, radius:number, color:Color){
  //   canvas.nativeElement.getContext("2d").beginPath(); // Start a new path
  //   canvas.nativeElement.getContext("2d").arc(x,y, radius, 0, Math.PI*2, false);
  //   canvas.nativeElement.getContext("2d").fillStyle = `rgb(${color.r}, ${color.g}, ${color.b})`;
  //   canvas.nativeElement.getContext("2d").fill();
  // }


}
