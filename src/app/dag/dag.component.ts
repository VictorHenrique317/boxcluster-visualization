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
import { Svg } from 'src/models/svg';
import { ActivatedRoute } from '@angular/router';

// https://angular.io/guide/template-syntax

@Component({
  selector: 'app-dag',
  standalone: true,
  imports: [CommonModule, MatCardModule],
  templateUrl: './dag.component.html',
  styleUrls: ['./dag.component.scss']
})
export class DagComponent implements AfterViewInit{
  private DEV_MODE: boolean;

  private datapoints_subscription: Subscription;
  private dev_mode_subscription: Subscription;

  @ViewChild('dagWindow') dagWindow: ElementRef<HTMLBodyElement>;
  private subscribed_datapoints: Array<DataPoint>;
  
  @ViewChild('vizualization_div') vizualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;
  // private plot: any;
  // private width:number; 
  // private height:number;
  private y_correction = 70;
  // private plot_padding = 50;
  // private x_scale: any;
  // private y_scale: any;

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

  constructor(private route: ActivatedRoute, private svg_service: SvgService, private dagview_service: DagViewService){
    this.datapoints_subscription = this.dagview_service.datapoints$.subscribe(value => {
      this.subscribed_datapoints = value;
      
      if(this.svg != undefined){ this.svg.setDatapoints(value); }
    });
  }
  
  ngAfterViewInit(){
    this.dev_mode_subscription =  this.route.queryParams.subscribe(params => {
      this.DEV_MODE = params['dev_mode'] === 'true' ? true : false;
    });

    this.dagview_service.updateDataPoints(this.DEV_MODE);

    console.log("Initializing dag view component with: " + this.subscribed_datapoints.length + " datapoints");

    let width = this.dagWindow.nativeElement.clientWidth;
    let height = this.dagWindow.nativeElement.clientHeight;
    this.svg = new Svg(this.vizualization_div, width, height, this.subscribed_datapoints, this.scalingFunction);
    this.svg.resize(width, height, this.y_correction);

    
  }

  ngOnDestroy(){
    this.datapoints_subscription.unsubscribe();
    this.dev_mode_subscription.unsubscribe();
  }

  private scalingFunction(datapoints: Array<DataPoint>) {
    let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let screen_coverage = 0.7;
    datapoints.forEach(datapoint => {
        datapoint.x /= x_max_module * ((1-screen_coverage) + 1);
        datapoint.y /= y_max_module * ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  public onResize(event) {
    let width = this.dagWindow.nativeElement.clientWidth;
    let height = this.dagWindow.nativeElement.clientHeight;

    this.svg.resize(width, height, this.y_correction);
  }
}
