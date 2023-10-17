import { ElementRef, Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class CanvasService {

  constructor() { }

  public fixCanvasRendering(parent_window: ElementRef, canvas: ElementRef<HTMLCanvasElement>) {
    canvas.nativeElement.width = parent_window.nativeElement.clientWidth;
    canvas.nativeElement.height = parent_window.nativeElement.clientHeight;
  }

  public clearCanvas(canvas: ElementRef<HTMLCanvasElement>){
    canvas.nativeElement.getContext("2d").setTransform(1, 0, 0, 1, 0, 0);
    canvas.nativeElement.getContext("2d").clearRect(0, 0, canvas.nativeElement.width, canvas.nativeElement.height);
  }

  public onResize(canvas: ElementRef<HTMLCanvasElement>, parent_window: ElementRef<HTMLBodyElement>) {
    this.clearCanvas(canvas);
    canvas.nativeElement.width = parent_window.nativeElement.clientWidth;
    canvas.nativeElement.height = parent_window.nativeElement.clientHeight;
  }

  public drawGrid(canvas: ElementRef<HTMLCanvasElement>, maximum_dx: number, maximum_dy: number) {
    const gridSize = 50;
  
    // Draw vertical lines
    for (let x = -maximum_dx; x <= maximum_dx*2; x += gridSize) {
      canvas.nativeElement.getContext("2d").moveTo(x, -maximum_dy);
      canvas.nativeElement.getContext("2d").lineTo(x, maximum_dy*2);
    }
  
    // Draw horizontal lines
    for (let y = -maximum_dy; y <= maximum_dy*2; y += gridSize) {
      canvas.nativeElement.getContext("2d").moveTo(-maximum_dx, y);
      canvas.nativeElement.getContext("2d").lineTo(maximum_dx*2, y);
    }
  
    // Set the line color and stroke the lines
    canvas.nativeElement.getContext("2d").strokeStyle = 'lightgrey';
    canvas.nativeElement.getContext("2d").stroke();
  }

}
