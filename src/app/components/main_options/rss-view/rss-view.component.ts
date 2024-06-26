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
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs';
import { environment } from 'src/environments/environment';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { ApiService } from 'src/app/services/api/api.service';

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
  private svg: any;
  private plot: any;

  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;
  
  protected sliderDisabled: boolean = false;
  @Output() onTruncation: EventEmitter<any> = new EventEmitter();
  @Output() initialized: EventEmitter<any> = new EventEmitter();

  public rss_evolution: Array<number> = [];
  private datapoints: Array<DataPoint>;
  private scaled_datapoints: Array<DataPoint>;
  protected pattern_number;

  constructor(private route: ActivatedRoute, private dialog_service: DialogService, private api_service: ApiService){}
  
  async ngAfterViewInit() {
    this.rss_evolution = await this.api_service.getFullRssEvolution();
    let subpatterns_identifiers: number[] = await this.api_service.getAllSubpatternsIdentifiers();
    
    this.pattern_number = this.rss_evolution.length;
    this.datapoints = this.wrapIntoDatapoints(this.rss_evolution, subpatterns_identifiers);
    
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.svg = this.createSvg();
    this.resizeSvg(width, height, 0);
    this.drawDataPoints();

    this.connectDatapoints();
    this.initialized.emit();
  }

  private wrapIntoDatapoints(rss_evolution: Array<number>, subpatterns_identifiers: number[]): Array<DataPoint>{
    let datapoints: DataPoint[] = [];
    let subpatterns_identifiers_set: Set<number> = new Set(subpatterns_identifiers);

    let gray_shade = 160;
    for (let i = 0; i < rss_evolution.length; i++){
      let identifier = i; // i because the first index is the null model rss
      let x = undefined;
      let y = undefined;

      let r = 0;
      let g = 0;
      let b = 0;
      let a = 1;
      if(subpatterns_identifiers_set.has(identifier)){
        r = gray_shade;
        g = gray_shade;
        b = gray_shade;
      }

      let datapoint = new DataPoint(i, 10, 10, 0, 0, x, y, r, g, b, a);
      datapoints[i] = datapoint;
    }

    return datapoints;
  }

  private scalingFunction(datapoints: Array<DataPoint>): Array<any>{
    let min_rss = Math.min(...this.rss_evolution.map(rss => Math.abs(rss)));
    let max_rss = Math.max(...this.rss_evolution.map(rss => Math.abs(rss)));

    let max_y = max_rss;
    let y_range = max_rss - min_rss;
    let length = datapoints.length;

    let lateral_screen_coverage = 1;
    let vertical_screen_coverage = 0.9;
    let scaled_datapoints: Array<DataPoint> = datapoints;
    for (let i = 0; i < datapoints.length; i++){
      let datapoint: DataPoint = datapoints[i];
      let rss = this.rss_evolution[i];
      
      let x = ((i + 0.5)/length) * 2 - 1; // scale x to be between -1 and 1
      x /= ((1-lateral_screen_coverage) + 1)
      
      let y = (rss - min_rss) / y_range; // Scale y to be between 0 and 1
      y = y * 2 - 1; // Scale y to be between -1 and 1
      y /= ((1-vertical_screen_coverage) + 1)
      
      let radius = 3;
      let scaled_datapoint = new DataPoint(i, radius, 10, 0, 0, x, y, datapoint.r, datapoint.g, datapoint.b, datapoint.a);
      scaled_datapoints[i] = scaled_datapoint;
    }

    return scaled_datapoints;
  }

  private drawDataPoints() {
    if(this.plot == undefined){ return; }
  
    this.scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('circle')
        .data(this.scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
  
    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', d => d.size)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer'); // Set cursor to pointer
  }

  private connectDatapoints(){
    console.log("Connecting datapoints");
    if(this.scaled_datapoints.length < 2){ return; }

    let line = d3.line<DataPoint>()
      .x(d => this.x_scale(d.x))
      .y(d => this.y_scale(d.y));

    for(let i = 0; i < this.scaled_datapoints.length - 1; i++) {
      let point1 = this.scaled_datapoints[i];
      let point2 = this.scaled_datapoints[i+1];

      this.plot.append('path')
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
    this.drawVerticalLine(x);
  }

  public onResize(event) {
    let width = this.visualization_div.nativeElement.clientWidth;
    let height = this.visualization_div.nativeElement.clientHeight;

    this.resizeSvg(width, height);
    this.drawDataPoints();
    this.connectDatapoints();
  }
  
  public getPatternNumber(): number{  
    return this.pattern_number;
  }

  public setPatternNumber(pattern_number: number){
    this.pattern_number = pattern_number;
  }

  // ========================= SVG FUNCTIONS ========================= //

  private createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
    .append('svg')
      .attr('width', this.svg_width)
      .attr('height',this.svg_height);

    return svg;
  }

  public resizeSvg(width: number, height: number, y_correction=0){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale;

    x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (width/1)]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }

    // Add the X gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", "lightgrey")
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    // Add the Y gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("color", "lightgrey")
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }
  
  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g").attr("id", "plot");
    
    this.drawGridLines();
  }

  private drawVerticalLine(x: number) {
    this.plot.selectAll('#vertical-line').remove();

    this.plot.append('line')
        .attr('id', 'vertical-line')
        .attr('x1', this.x_scale(x))
        .attr('y1', 0)
        .attr('x2', this.x_scale(x))
        .attr('y2', this.svg_height)
        .attr('stroke', 'red')
        .attr('stroke-width', 2);
  }

  // ========================= SVG FUNCTIONS ========================= //
}


