import { Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { CanvasService } from '../services/canva.service';
import { FormsModule } from '@angular/forms';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCardModule} from '@angular/material/card';
import { Coordinate } from 'src/models/coordinate';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-rss-view',
  standalone: true,
  imports: [
    CommonModule,
    MatSliderModule,
    FormsModule
  ],
  templateUrl: './rss-view.component.html',
  styleUrls: ['./rss-view.component.scss']
})
export class RssViewComponent {
  @ViewChild('canvas') canvas: ElementRef<HTMLCanvasElement>;
  @ViewChild('rssWindow') rssWindow: ElementRef<HTMLBodyElement>;

  max = 100;
  pattern_number = 1;

  private context: CanvasRenderingContext2D;
  public rss_evolution: Array<number>;

  constructor(private canvas_service: CanvasService){}

  ngAfterViewInit(){
    this.context = this.canvas.nativeElement.getContext("2d");
    this.canvas_service.fixCanvasRendering(this.rssWindow, this.canvas);
    this.canvas_service.drawGrid(this.canvas, this.canvas.nativeElement.width*4, this.canvas.nativeElement.height);

    this.getRssEvolution();
  } 

  public getRssEvolution(){
    this.rss_evolution = [44356, 44256, 44156, 43956, 4389, 4370, 4365];
    // invoke("getFullRssEvolution").then((result: Array<number>) =>{
    //   this.rss_evolution = result;
    //   console.log(this.rss_evolution);
    // });
  }

}
