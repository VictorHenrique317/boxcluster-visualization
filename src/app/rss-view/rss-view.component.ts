import { Component, ElementRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { SvgService } from '../services/svg.service';
import { FormsModule } from '@angular/forms';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCardModule} from '@angular/material/card';
import { DataPoint } from 'src/models/datapoint';
import { invoke } from '@tauri-apps/api';
import { ChangeDetectorRef } from '@angular/core';
import { DagViewService } from '../services/dag-view.service';
import { Color } from 'src/models/color';
import { Svg } from 'src/models/svg';

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
  @ViewChild('visualization_div') canvas: ElementRef<HTMLDivElement>;
  private svg: Svg;
  pattern_number = 1;

  @ViewChild('rssWindow') rssWindow: ElementRef<HTMLBodyElement>;
  
  private max_y: number;
  private y_range: number;

  public rss_evolution: Array<number> = [];

  constructor(private canvas_service: SvgService, private dagview_service: DagViewService){}

  ngAfterViewInit() {
    let width = 930;
    let height = 463;

    invoke("getFullRssEvolution").then((result: Array<number>) =>{
        this.rss_evolution = result;
        this.svg = new Svg(this.canvas, width, height, this.rss_evolution, this.scalingFunction, true, false);
        this.svg.resize(width, height, 0);
        
      }).catch((error: any) => {
        console.log(error);
      });

    // this.rss_evolution = [
    //   55563.5,
    //   55548.7,
    //   55534.2,
    //   55519.6,
    //   55505.7,
    //   55492.6,
    //   55479.7,
    //   55467,
    //   55454.9,
    //   55443.1,
    //   55432.9,
    //   55423,
    //   55413.4,
    //   55403.8,
    //   55394.2,
    //   55384.9,
    //   55375.5,
    //   55366.3,
    //   55357,
    //   55347.8]

    
      
    // this.svg = new Svg(this.canvas, width, height, this.rss_evolution, this.scalingFunction, true, false);
    // this.svg.resize(width, height, 0);
  }

  // private createDatapoints(rss_evolution: Array<number>): Array<DataPoint>{
  //   let datapoints: DataPoint[] = [];

  //   for (let i = 0; i < rss_evolution.length; i++){
  //     let rss = rss_evolution[i];
  //     let x = i/rss_evolution.length;
  //     let y = rss;
  //     let radius = 10;
  //     let datapoint = new DataPoint(i, radius, 0, x, y, 0, 0, 0);
  //     datapoints[i] = datapoint;
  //   }
  // }

  // updateMax() {
  //   this.max = this.rss_evolution.length;
  //   // this.cdr.detectChanges();
  // }

  private scalingFunction(datapoints: Array<any>): Array<any>{
    let min_rss = Math.min(...datapoints);
    let max_rss = Math.max(...datapoints);
    let max_y = max_rss;
    let y_range = max_rss - min_rss;
    let length = datapoints.length;
    
    let screen_coverage = 0.7;
    let scaled_datapoints: Array<DataPoint> = datapoints;
    for (let i = 0; i < datapoints.length; i++){
      let rss = datapoints[i];
      
      let x = (i/length) * 2 - 1; // scale x to be between -1 and 1
      x *= ((1-screen_coverage) + 1)

      let y = (rss - min_rss) / y_range; // Scale y to be between 0 and 1
      
      let radius = 10;
      let datapoint = new DataPoint(i, radius, 0, x, y, 0, 0, 0);
      scaled_datapoints[i] = datapoint;
    }

    return scaled_datapoints;
  }
  
  // private scaleToFitCanvas(x: number, y:number, radius: number){
  //   // Define padding (as a percentage of canvas size)
  //   let paddingX = 0.02 * this.canvas.nativeElement.width;
  //   let paddingY = 0.05 * this.canvas.nativeElement.height;
  
  //   // Adjust canvas size to account for padding
  //   let adjustedWidth = this.canvas.nativeElement.width - 2 * paddingX;
  //   let adjustedHeight = this.canvas.nativeElement.height - 2 * paddingY;
  
  //   // Scale x and y to fit within the adjusted canvas size, and add padding
  //   let scaled_x = x * adjustedWidth + paddingX;
  //   let scaled_y = (1 - y) * adjustedHeight + paddingY; // Flip y axis so that higher values are at the top
  //   let scaled_radius = radius;
  
  //   return {x: scaled_x, y: scaled_y, radius: scaled_radius};
  // }

  private drawRssEvolution(){
    // for (let i = 0; i < this.coordinates.length; i++){
    //   let coordinate = this.coordinates[i];
    //   let scaled_coordinate = this.scaleToFitCanvas(coordinate[0], coordinate[1], 10);
    //   let color: Color = {r: 0, g: 0, b: 0};
    //   this.canvas_service.drawCircle(this.canvas, scaled_coordinate.x, scaled_coordinate.y, scaled_coordinate.radius, color);
    // }
  }

  onSliderChange(event: any) {
    // this.svg.getD3Svg().select("#truncate_line").remove();
    // // Clear the last drawn line by restoring the saved state
    // this.context.clearRect(0, 0, this.canvas.nativeElement.width, this.canvas.nativeElement.height);
    // this.context.restore();
  
    // // Save the current state of the canvas before drawing the new line
    // this.context.save();
  
    // // This function will be called when the slider is stopped being dragged
    // let coordinate = this.coordinates[this.pattern_number -1];
    // let {x, y} = this.scaleToFitCanvas(coordinate[0], coordinate[1], 10);
    
    // this.canvas_service.drawGrid(this.canvas, this.canvas.nativeElement.width*4, this.canvas.nativeElement.height, 50);
    // this.drawRssEvolution();

    // // Draw a vertical line at the x position of the i-th point
    // this.context.strokeStyle = 'red';
    // this.context.lineWidth = 5;
    // this.context.beginPath();
    // this.context.moveTo(x, 0);
    // this.context.lineTo(x, this.canvas.nativeElement.height);
    // this.context.stroke();

    // this.dagview_service.truncateDataPoints(this.pattern_number);
  }

  public onResize(event) {
    let width = this.rssWindow.nativeElement.clientWidth;
    let height = this.rssWindow.nativeElement.clientHeight;

    this.svg.resize(width, height, 0);
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }
}
