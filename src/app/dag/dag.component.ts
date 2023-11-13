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
  private width:number; 
  private height:number;
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
      this.updatePlot();
    });
  }
  
  ngAfterViewInit(){
    this.dagview_service.updateDataPoints();
    this.createSvg();
  }

  ngOnDestroy(){
    this.datapoints_subscription.unsubscribe();
  }

  private createSvg(){
    let width = 1024;
    let height = 720;

    this.svg = d3.select(this.vizualization_div.nativeElement)
      .append('svg')
        .attr('width', width)
        .attr('height',height);

    this.rescaleSvg(width, height);
  }

  private rescaleSvg(width: number, height: number){
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

    // this.svg.append('g')
    //   .attr('transform', `translate(0,${(height - this.y_correction)/2})`)
    //   .call(x_axis);

    // this.svg.append('g')
    //   .attr('transform', `translate(${width/2},0)`)
    //   .call(y_axis);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.width = width;
    this.height = height;

    this.updatePlot();
  }

  private drawGridLines(){
    // Define the gridlines
    let xGridlines = d3.axisBottom(this.x_scale)
      .ticks(20)
      .tickSize(-this.height + 2)
      .tickFormat(() => "");

    let yGridlines = d3.axisLeft(this.y_scale)
      .ticks(20)
      .tickSize(-this.width + 2)
      .tickFormat(() => "");

    // Add the X gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(0," + (this.height) + ")")
      .call(xGridlines);

    // Add the Y gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(" + 0 + ",0)")
      .call(yGridlines);
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
    if(this.plot == null){ return; }
    let scaled_datapoints = this.scaleToFitPlot(this.subscribed_datapoints);

    this.plot.selectAll('circle')
      .data(scaled_datapoints)
      .enter()
      .append('circle')
        .attr('cx', d => this.x_scale(d.x))
        .attr('cy', d => this.y_scale(d.y))
        .attr('r', d => d.size)
        // .attr('stroke-width', d => d.stroke_width)
        .attr('fill', d => `rgb(${d.r}, ${d.g}, ${d.b})`);
  }

  private updatePlot(){
    if (this.svg == null) { return; }
    
    this.svg.selectAll('*').remove();

    this.plot = this.svg.append("g")
      .attr("id", "plot")
      .attr("transform", "translate("+ this.totalDx +", "+ this.totalDy +")");

    this.drawGridLines();
    this.drawDataPoints();
  }

  public onResize(event) {
    let width = this.dagWindow.nativeElement.clientWidth;
    let height = this.dagWindow.nativeElement.clientHeight;

    this.rescaleSvg(width, height);
  }

  public mouseDownHandler(e) {
    this.isDragging = true;
    this.previousMousePosition = {
      x: e.clientX,
      y: e.clientY
    };
  }
  
  public mouseMoveHandler(e) {
    if (this.isDragging == false) { return; }

    const dx = e.clientX - this.previousMousePosition.x;
    const dy = e.clientY - this.previousMousePosition.y;

    // Update the total translation
    let temp_total_dx = (this.totalDx + dx);
    let temp_total_dy = (this.totalDy + dy);

    // Update maximum dx and dy based on the current scale
    this.maximum_dx = this.width * this.scale * this.maximum_dislocation_multiplier;
    this.maximum_dy = this.height * this.scale * this.maximum_dislocation_multiplier;
    
    // console.log(this.scale);
    if ((temp_total_dx / this.scale) > this.maximum_dx / this.scale) {return;} // Left side block
    if (temp_total_dx / this.scale < -this.maximum_dx) {return;} // Right side block
    if (temp_total_dy / this.scale < -this.maximum_dy) {;return;} // Bottom side block
    if ((temp_total_dy / this.scale) > this.maximum_dy / this.scale) {return;} // Top side block

    this.totalDx = temp_total_dx;
    this.totalDy = temp_total_dy;

    // console.log(this.totalDx, this.totalDy);

    // Update the mouse position
    this.previousMousePosition = {
        x: e.clientX,
        y: e.clientY
      };
  
    this.updatePlot();
  }
  
  public mouseUpHandler() {
    this.isDragging = false;
  }
  
  // public wheelHandler(event: WheelEvent) {
  //   event.preventDefault();
  
  //   // Calculate the mouse position in canvas coordinates
  //   const rect = this.canvas.nativeElement.getBoundingClientRect();
  //   const mouseX = (event.clientX - rect.left - this.totalDx) / this.scale;
  //   const mouseY = (event.clientY - rect.top - this.totalDy) / this.scale;
  
  //   // Update the scale
  //   const oldScale = this.scale;
  //   if (event.deltaY < 0) { // Zoom in
  //     this.scale /= this.scaleMultiplier;

  //   } else { // Zoom out
  //     let temp_scale = this.scale * this.scaleMultiplier;
  //     if (temp_scale < this.minimum_scale){ return;} // Minimum amount of zoom
  //     this.scale = temp_scale;
  //   }
  
  //   // Update the translation to center the zoom effect at the mouse position
  //   this.totalDx -= mouseX * (this.scale - oldScale);
  //   this.totalDy -= mouseY * (this.scale - oldScale);
  
  //   // Redraw the canvas
  //   this.canvas_service.clearCanvas(this.canvas);
  //   this.drawDataPoints();

  //   // console.log(this.scale);
  // }
}
