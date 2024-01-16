import { resolveResource } from '@tauri-apps/api/path'
import * as d3 from 'd3';
import { Component, ElementRef, EventEmitter, Output, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSliderModule} from '@angular/material/slider';
import { SvgService } from 'src/app/services/svg/svg.service';
import { FormsModule } from '@angular/forms';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatCardModule} from '@angular/material/card';
import { DataPoint } from 'src/app/models/datapoint';
import { fs, invoke } from '@tauri-apps/api';
import { ChangeDetectorRef } from '@angular/core';
import { AfterViewInit } from '@angular/core'
import { Color } from 'src/app/models/color';
import { Svg } from 'src/app/models/svg';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs';
import { environment } from 'src/environments/environment';

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
export class RssViewComponent implements AfterViewInit{
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;

  @ViewChild('visualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;
  pattern_number;

  protected sliderDisabled: boolean = false;
  @Output() onTruncation: EventEmitter<any> = new EventEmitter();
  @Output() initialized: EventEmitter<any> = new EventEmitter();

  private max_y: number;
  private y_range: number;

  public rss_evolution: Array<number> = [];
  private datapoints: Array<DataPoint>;

  constructor(private route: ActivatedRoute, private canvas_service: SvgService, private cdr: ChangeDetectorRef){}
  
  async ngAfterViewInit() {
    console.log("Initializing rss view component");
    console.log("Invoking getFullRssEvolution");
    
    let rss_evolution;
    if(!environment.dev_mode){
      rss_evolution = await invoke("getFullRssEvolution").catch((error: any) => {
        console.log(error);
      });

    } else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/rss_evolution.json'));
      rss_evolution = JSON.parse(rawdata);
    }

    console.log("Received rss_evolution:");
    console.log(rss_evolution);
    this.rss_evolution = rss_evolution;
    this.pattern_number = this.rss_evolution.length;

    this.datapoints = this.wrapIntoDatapoints(this.rss_evolution);
    this.initializeSvg();
  }

  private initializeSvg() {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg = new Svg(this.visualization_div, width, height, this.datapoints, this.scalingFunction, 10, true, false);
    this.svg.resize(width, height, 0);
    this.connectDatapoints();

    this.initialized.emit();
   }

  private wrapIntoDatapoints(rss_evolution: Array<number>): Array<DataPoint>{
    let datapoints: DataPoint[] = [];

    for (let i = 0; i < rss_evolution.length; i++){
      let x = undefined;
      let y = undefined;
      let datapoint = new DataPoint(i, rss_evolution[i], 10, 0, x, y, 0, 0, 0, 0);
      datapoints[i] = datapoint;
    }

    return datapoints;
  }

  private scalingFunction(datapoints: Array<DataPoint>): Array<any>{
    let min_rss = Math.min(...datapoints.map(datapoint => Math.abs(datapoint.value)));
    let max_rss = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.value)));

    let max_y = max_rss;
    let y_range = max_rss - min_rss;
    let length = datapoints.length;

    let lateral_screen_coverage = 1;
    let vertical_screen_coverage = 0.9;
    let scaled_datapoints: Array<DataPoint> = datapoints;
    for (let i = 0; i < datapoints.length; i++){
      let rss = datapoints[i].value;
      
      let x = ((i + 0.5)/length) * 2 - 1; // scale x to be between -1 and 1
      x /= ((1-lateral_screen_coverage) + 1)
      
      let y = (rss - min_rss) / y_range; // Scale y to be between 0 and 1
      y = y * 2 - 1; // Scale y to be between -1 and 1
      y /= ((1-vertical_screen_coverage) + 1)
      
      let radius = 3;
      let datapoint = new DataPoint(i, rss, radius, 0, x, y, 0, 0, 0, 1);
      scaled_datapoints[i] = datapoint;
    }

    return scaled_datapoints;
  }

  private connectDatapoints(){
    console.log("Connecting datapoints");
    let scaled_datapoints = this.svg.getScaledDatapoints();
    if(scaled_datapoints.length < 2){ return; }

    let line = d3.line<DataPoint>()
      .x(d => this.svg.getXScale()(d.x))
      .y(d => this.svg.getYScale()(d.y));

    for(let i = 0; i < scaled_datapoints.length - 1; i++) {
      let point1 = scaled_datapoints[i];
      let point2 = scaled_datapoints[i+1];

      this.svg.getPlot().append('path')
        .attr('d', line([point1, point2]))
        .attr('stroke', 'black')
        .attr('stroke-width', 2)
        .attr('fill', 'none');
    }
  }

  public enableSlider(){ this.sliderDisabled = false; }
  public disableSlider(){ this.sliderDisabled = true; }
  
  protected onSliderChange(event: any) {
    this.onTruncation.emit(this.pattern_number);
  }

  protected onSliderDrag(event: any) {
    let x = this.datapoints[this.pattern_number - 1].x;
    this.svg.drawVerticalLine(x);
  }

  public onResize(event) {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg.resize(width, height, 0);
    this.connectDatapoints();
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }
}
