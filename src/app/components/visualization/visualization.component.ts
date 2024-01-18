import { resolveResource } from '@tauri-apps/api/path'
import { ChangeDetectorRef, Component, ComponentFactoryResolver, EventEmitter, InjectionToken, Input, Output, Renderer2, ViewContainerRef } from '@angular/core';
import { ComponentPortal, PortalModule } from '@angular/cdk/portal';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/app/models/datapoint';
import { event, fs, invoke } from '@tauri-apps/api';
import { BaseDirectory } from "@tauri-apps/api/fs";
import { SvgService } from 'src/app/services/svg/svg.service';
import { Subscription } from 'rxjs';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';
import { Svg } from 'src/app/models/svg';
import { ActivatedRoute } from '@angular/router';
import { RssViewComponent } from 'src/app/components/main_options/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { DataPointTooltipComponent } from "./datapoint-tooltip/datapoint-tooltip.component";

@Component({
    selector: 'app-visualization',
    standalone: true,
    templateUrl: './visualization.component.html',
    styleUrls: ['./visualization.component.scss'],
    animations: [
        trigger('slideInOut', [
            state('void', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            state('in', style({
                transform: 'translateX(0)',
                opacity: 1
            })),
            state('out', style({
                transform: 'translateX(100%)',
                opacity: 0
            })),
            transition('void => in', [
                animate('0.5s ease-in-out')
            ]),
            transition('in => out', [
                animate('0.5s ease-in-out')
            ]),
            transition('out => in', [
                animate('0.5s ease-in-out')
            ])
        ])
    ],
    imports: [
        CommonModule,
        MatCardModule,
        PortalModule,
        RssViewComponent,
        DataPointTooltipComponent
    ]
})

export class VisualizationComponent implements AfterViewInit{
  @Input() app_body: HTMLBodyElement;
  
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  
  private datapoints: Array<DataPoint>;

  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;
  private y_correction = 0;

  @ViewChild('datapoint_tooltip') datapoint_tooltip: DataPointTooltipComponent;

  constructor(private route: ActivatedRoute, private cdr: ChangeDetectorRef){ }
  ngAfterViewInit(): void {
    console.log("Initializing visualization component");
    this.updateDataPoints();
  }
  
  private createSvg(){
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;
    
    this.svg = new Svg(this.visualization_div, width, height, this.datapoints.slice(), 
      this.scalingFunction, 
      this.hoverFunction,
      this.app_body,
      this.datapoint_tooltip,
      40, true, true);
    this.svg.resize(width, height, this.y_correction);
    
    this.cdr.detectChanges();
  }

  public async updateDataPoints(){
    console.log("Invoking getDataPoints");
    
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDataPoints").catch((error: any) => {
        console.log(error);
      });

    } else if (environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints.json'));
      datapoints = JSON.parse(rawdata);
    }

    console.log("Received datapoints:");
    console.log(datapoints);

    this.datapoints = datapoints;

    if(this.svg == undefined) { 
      this.createSvg();
      
    } else if(this.svg != undefined){
      this.svg.setDatapoints(this.datapoints);
    }
  }

  private hoverFunction(mouse_position: [number, number], datapoint: DataPoint, 
      tooltip_component: DataPointTooltipComponent, positioning_ref: HTMLBodyElement){
    
    console.log(positioning_ref.clientWidth);
    console.log(positioning_ref.clientHeight);
    console.log(mouse_position[0]);
    console.log(mouse_position[1]);
    // console.log(positioning_ref.nativeElement.getBoundingClientRect().top);
    // console.log(positioning_ref.nativeElement.getBoundingClientRect().left);

    tooltip_component.setDatapoint(datapoint);
    tooltip_component.toggleVisibility();

    // Convert mouse_position to CSS units
    const left = mouse_position[0] + 'px';
    const top = mouse_position[1] + 'px';

    // Set the position of the tooltip
    tooltip_component.setPosition(top, left);
}

  private scalingFunction(datapoints: Array<DataPoint>) {
    let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let screen_coverage = 0.05;
    datapoints.forEach(datapoint => {
        if(y_max_module > -0.5 && y_max_module < 0.5){
          y_max_module = 1;
        }

        datapoint.x /= y_max_module * ((1-screen_coverage) + 1);
        datapoint.y /= y_max_module * ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg.resize(width, height, this.y_correction);
  }

  public onTruncation(event){
    let new_size = event - 1; // -1 because the first point is the null model rss
    console.log("Truncating datapoints to only: " + new_size);

    if(environment.dev_mode) {
      this.updateDataPoints(); // Reseting to all original datapoints in dev mode

      let new_datapoints = [];

      for(let i = 0; i < new_size; i++){
        new_datapoints.push(this.datapoints[i]);
      }

      this.datapoints = new_datapoints; // Truncation
      if(this.svg != undefined){ this.svg.setDatapoints(this.datapoints); } // Updating the svg
      return;
    }

    invoke("truncateModel", {newSize: new_size}).then((datapoint_changes: any) => { // This performs the truncation
      this.updateDataPoints();

    }).catch((error: any) => {
      console.log(error);
    });
  }
}
