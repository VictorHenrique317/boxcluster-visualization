import * as d3Tip from "d3-tip";
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
import { Subscription, take } from 'rxjs';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';
import { Svg } from 'src/app/models/svg';
import { ActivatedRoute } from '@angular/router';
import { RssViewComponent } from 'src/app/components/main_options/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { DataPointTooltipComponent } from "./datapoint-tooltip/datapoint-tooltip.component";
import { DatapointInfoDialogComponent } from "./datapoint-info-dialog/datapoint-info-dialog.component";
import { MatDialog } from "@angular/material/dialog";
import { Pattern } from "src/app/models/pattern";

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
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  
  private datapoints: Array<DataPoint>;

  @ViewChild('vizualization_div') visualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;
  private tooltip;
  private y_correction = 0;

  constructor(private route: ActivatedRoute, public dialog: MatDialog, private cdr: ChangeDetectorRef){ }
  ngAfterViewInit(): void {
    console.log("Initializing visualization component");
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.tooltip = d3Tip.default()
      .attr('class', 'd3-tip')
      .offset([-10, 0])
      .html(function(d) {
        return "\
          <div style='background-color:#ededed; padding: 0.5em 0.5em 0.5em 0.5em; border-radius: 10px; border: 1px dashed black;'>\
            <strong>ID:</strong> <span style='color:#BC2602'>" + d.identifier + "</span><br>\
            <strong>Size:</strong> <span style='color:#BC2602'>" + d.pattern_size + "</span><br>\
            <strong>Density:</strong> <span style='color:#BC2602'>" + Math.round(d.density * 100) / 100 + "</span>\
          </div>\
          ";
      });
    
    this.svg = new Svg(this.visualization_div, width, height, 40, true, true);
    this.svg.resize(width, height, this.y_correction);
    this.cdr.detectChanges();

    this.updateDataPoints();
  }
  
  public async updateDataPoints(){
    console.log("Invoking getDataPoints");
    
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDataPoints").catch((error: any) => {
        console.error(error);
      });

    } else if (environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints.json'));
      datapoints = JSON.parse(rawdata);
    }

    console.log("Received datapoints:");
    console.log(datapoints);

    this.datapoints = datapoints;
    this.drawDataPoints();

    // this.onDatapointClick('300ms', '300ms', 1); // TODO: REMOVE
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

  private drawDataPoints() {
    if(this.svg.plot == undefined){ return; }

    this.svg.plot.call(this.tooltip);
  
    let scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.svg.plot.selectAll('circle')
        .data(scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
  
    circles.exit()
        .transition() // Add exit animation
        .duration(1000) // Duration of the animation in milliseconds
        .attr('r', 0) // Reduce radius to 0
        .remove(); // Remove datapoints that are not in the new dataset
  
    circles.transition() // Animate existing datapoints
        .duration(1000) // Duration of the animation in milliseconds
        .attr('cx', d => {
            const result = this.svg.getXScale()(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.svg.getYScale()(parseFloat(d.y)));
  
    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.svg.getXScale()(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.svg.getYScale()(parseFloat(d.y)))
        .attr('r', 0) // Start from radius 0
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer') // Set cursor to pointer
        .on('mouseover', (event, d) => { this.tooltip.show(d, event.currentTarget); })
        .on('mouseout', (event, d) => { this.tooltip.hide(d, event.currentTarget); })
        .on('click', (event, d) => { this.onDatapointClick('300ms', '300ms', d.identifier); })
        .transition() // Transition to final state
        .duration(1000) // Duration of the animation in milliseconds
        .attr('r', d => d.size); // End with actual radius
  }

  private async onDatapointClick(enterAnimationDuration: string, exitAnimationDuration: string, identifier: number): Promise<void> {
    let pattern;
    if(!environment.dev_mode){
      pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
        console.error(error);
      });

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/pattern.json'));
      pattern = JSON.parse(rawdata);
    }

    const dialogRef = this.dialog.open(DatapointInfoDialogComponent, {
      width: '500px',
      height: '590px',
      enterAnimationDuration,
      exitAnimationDuration,
      
      data: {
        pattern: pattern
      }
    });

    // Executes when the dialog is closed
    dialogRef.afterClosed().pipe(take(1)).subscribe(result => {});
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg.resize(width, height, this.y_correction);
    this.drawDataPoints();
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
      if(this.svg != undefined){ this.drawDataPoints(); } // Updating the svg
      return;
    }

    invoke("truncateModel", {newSize: new_size}).then((datapoint_changes: any) => { // This performs the truncation
      this.updateDataPoints();

    }).catch((error: any) => {
      console.log(error);
    });
  }
}
