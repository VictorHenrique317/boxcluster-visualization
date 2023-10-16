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

}
