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
import { invoke } from '@tauri-apps/api';
import { ChangeDetectorRef } from '@angular/core';
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
export class RssViewComponent {
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;

  @ViewChild('visualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;
  pattern_number;

  @Output() onTruncation: EventEmitter<any> = new EventEmitter();

  protected enabled: boolean = false;

  private max_y: number;
  private y_range: number;

  public rss_evolution: Array<number> = [];
  private datapoints: Array<DataPoint>;

  constructor(private route: ActivatedRoute, private canvas_service: SvgService, private cdr: ChangeDetectorRef){}

  ngOnInit(){
    console.log("Initializing rss view component");
    
    if(!environment.dev_mode){
      invoke("getFullRssEvolution").then((result: Array<number>) =>{
        this.rss_evolution = result;
        this.datapoints = this.wrapIntoDatapoints(this.rss_evolution);
        this.pattern_number = this.rss_evolution.length;
        
      }).catch((error: any) => {
        console.log(error);
      });
    }

    if(environment.dev_mode){
      this.rss_evolution = [55563.5, 55548.7, 55534.2, 55519.6, 55505.7, 55492.6, 55479.7, 55467,
        55454.9, 55443.1, 55432.9, 55423, 55413.4, 55403.8, 55394.2, 55384.9, 55375.5, 55366.3, 55357, 55347.8]
        
      this.datapoints = this.wrapIntoDatapoints(this.rss_evolution);
      this.pattern_number = this.rss_evolution.length;
    }
  }

  ngAfterViewInit() { }

  protected toggleVisualization(){
    console.log("tottle");
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg = new Svg(this.visualization_div, width, height, this.datapoints, this.scalingFunction, 10, true, false);
    this.svg.resize(width, height, 0);

    this.onSliderChange(null);

    this.enabled = !this.enabled;
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
      
      let radius = 10;
      let datapoint = new DataPoint(i, rss, radius, 0, x, y, 0, 0, 0, 0);
      scaled_datapoints[i] = datapoint;
    }

    return scaled_datapoints;
  }
  
  onSliderChange(event: any) {
    let x = this.datapoints[this.pattern_number - 1].x;
    this.svg.drawVerticalLine(x);

    this.onTruncation.emit(this.pattern_number);
  }

  public onResize(event) {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg.resize(width, height, 0);
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }
}
