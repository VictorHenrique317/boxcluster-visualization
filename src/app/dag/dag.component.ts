import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/models/datapoint';
import { event, invoke } from '@tauri-apps/api';
import { SvgService } from '../services/svg.service';
import { DagViewService } from '../services/dag-view.service';
import { Subscription } from 'rxjs';
import { Color } from 'src/models/color';
import * as d3 from 'd3';

// https://angular.io/guide/template-syntax

@Component({
  selector: 'app-dag',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './dag.component.html',
  styleUrls: ['./dag.component.scss']
})
export class DagComponent implements AfterViewInit{
  @ViewChild('dagWindow') dagWindow: ElementRef<HTMLBodyElement>;
  private datapoints_subscription: Subscription;
  private subscribed_datapoints: Array<DataPoint>;
  
  @ViewChild('vizualization_div') vizualization_div: ElementRef<HTMLDivElement>;
  private svg: any;
  private plot: any;
  private drag: any;
  private y_correction = 70;
  private plot_padding = 50;
  private x_scale: any;
  private y_scale: any;

  private totalDx = 0;
  private totalDy = 0;
  private maximum_dislocation_multiplier = 0.5;
  private maximum_dx: number;
  private maximum_dy: number;

  private scale: number = 1.0;
  private scaleMultiplier: number = 0.8;
  private minimum_scale: number = 0.8;

  // Variables to keep track of the mouse position and left-button status 
  private isDragging = false;
  private previousMousePosition = {
    x: 0,
    y: 0
  };

  constructor(private svg_service: SvgService, private dagview_service: DagViewService){
    this.datapoints_subscription = this.dagview_service.datapoints$.subscribe(value => {
      this.subscribed_datapoints = value;
      this.drawDataPoints();
    });
  }
  
  ngAfterViewInit(){
    this.dagview_service.updateDataPoints();

    this.createPlot();
  }

  ngOnDestroy(){
    this.datapoints_subscription.unsubscribe();
  }

  private createPlot(){
    let width = 1024;
    let height = 720;

    this.svg = d3.select(this.vizualization_div.nativeElement)
      .append('svg')
        .attr('width', width)
        .attr('height',height);

    this.plot = this.svg.append('g');

    this.drag = d3.drag()
      .on("start", this.dragstarted)
      .on("drag", this.dragged)
      .on("end", this.dragended);

    this.updatePlot(width, height);
  }

  private updatePlot(width: number, height: number) {
    d3.select('svg').selectAll('*').remove();

    this.svg
      .attr('width', width)
      .attr('height', height);

      // X axis
    let x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, width]);

    let x_axis = d3.axisBottom(x_scale);

    // Y axis
    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([height - this.y_correction, 0]);

    let y_axis = d3.axisRight(y_scale);

    this.svg.append('g')
      .attr('transform', `translate(0,${(height - this.y_correction)/2})`)
      .call(x_axis);

    this.svg.append('g')
      .attr('transform', `translate(${width/2},0)`)
      .call(y_axis);

    this.x_scale = x_scale;
    this.y_scale = y_scale;

    this.drawGridLines(width, height);
    this.drawDataPoints();
  }

  private drawGridLines(width: number, height: number){
    // // Define the gridlines
    // let xGridlines = d3.axisBottom(this.x_scale)
    //   .ticks(20)
    //   .tickSize(-height + 2)
    //   .tickFormat(() => "");

    // let yGridlines = d3.axisLeft(this.y_scale)
    //   .ticks(20)
    //   .tickSize(-width + 2)
    //   .tickFormat(() => "");

    // // Add the X gridlines
    // this.svg.append("g")			
    //   .attr("class", "grid")
    //   .attr("transform", "translate(0," + (height) + ")")
    //   .call(xGridlines);

    // // Add the Y gridlines
    // this.svg.append("g")			
    //   .attr("class", "grid")
    //   .attr("transform", "translate(" + 0 + ",0)")
    //   .call(yGridlines);
  }

  public onResize(event) {
    let width = this.dagWindow.nativeElement.clientWidth;
    let height = this.dagWindow.nativeElement.clientHeight;

    this.updatePlot(width, height);
  }

  private scaleToFitPlot(datapoints: Array<DataPoint>) {
    let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let scale_x: boolean = x_max_module > 1;
    let scale_y: boolean = y_max_module > 1;
    datapoints.forEach(datapoint => {
        datapoint.x /= x_max_module * 1.1;
        datapoint.y /= y_max_module * 1.1;
        // if(scale_x){ datapoint.x /= x_max_module; }
        // if(scale_y){ datapoint.y /= y_max_module; }
    });

    return scaled_datapoints;
  }

  private drawDataPoints() {
    if(this.svg == null){ return; }

    this.svg.selectAll('circle').remove();
    
    let scaled_datapoints = this.scaleToFitPlot(this.subscribed_datapoints);
  
    let circles = this.svg.selectAll('circle')
      .data(scaled_datapoints)
      .enter()
      .append('circle')
        .attr('cx', d => this.x_scale(d.x))
        .attr('cy', d => this.y_scale(d.y))
        .attr('r', d => d.size)
        // .attr('stroke-width', d => d.stroke_width)
        .attr('fill', d => `rgb(${d.r}, ${d.g}, ${d.b})`);
  }

  private dragstarted(event, d) {
    
    d3.select(this.vizualization_div.nativeElement).raise().attr("stroke", "black");
  }

  private dragged(event, d) {
    console.log("dragged");
    d3.select(this.vizualization_div.nativeElement).attr("cx", d.x = event.x).attr("cy", d.y = event.y);
  }

  private dragended(event, d) {
    d3.select(this.vizualization_div.nativeElement).attr("stroke", null);
  }
}
