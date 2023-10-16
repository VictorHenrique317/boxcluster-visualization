import { Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { CanvasService } from '../services/canva.service';

@Component({
  selector: 'app-rss-view',
  standalone: true,
  imports: [
    CommonModule,
    MatSliderModule
  ],
  templateUrl: './rss-view.component.html',
  styleUrls: ['./rss-view.component.scss']
})
export class RssViewComponent {
  @ViewChild('canvas') canvas: ElementRef<HTMLCanvasElement>;
  @ViewChild('rssWindow') rssWindow: ElementRef<HTMLBodyElement>;

  private context: CanvasRenderingContext2D;

  constructor(private canvas_service: CanvasService){}

  ngAfterViewInit(){
    this.canvas_service.fixCanvasRendering(this.rssWindow, this.canvas);
  }

}
