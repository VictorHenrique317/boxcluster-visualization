import { ChangeDetectorRef, Component, ViewContainerRef } from '@angular/core';
import { ComponentPortal, PortalModule } from '@angular/cdk/portal';
import { CommonModule } from '@angular/common';
import {MatCardModule} from '@angular/material/card';
import { ViewChild } from '@angular/core'
import { ElementRef } from '@angular/core'
import { AfterViewInit } from '@angular/core'
import {cover, contain} from 'intrinsic-scale';
import { DataPoint } from 'src/app/models/datapoint';
import { event, invoke } from '@tauri-apps/api';
import { SvgService } from 'src/app/services/svg/svg.service';
import { Subscription } from 'rxjs';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';
import { Svg } from 'src/app/models/svg';
import { ActivatedRoute } from '@angular/router';
import { PortalDirective } from '../../directives/portal-directive.directive';
import { RssViewComponent } from 'src/app/components/visualization/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { RssViewDrawerComponent } from "./rss-view-drawer/rss-view-drawer.component";
import { animate, state, style, transition, trigger } from '@angular/animations';

@Component({
    selector: 'app-visualization',
    standalone: true,
    templateUrl: './visualization.component.html',
    styleUrls: ['./visualization.component.scss'],
    imports: [
      CommonModule,
      MatCardModule, 
      PortalModule, 
      RssViewComponent, 
      RssViewDrawerComponent],
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
      ]
})
export class VisualizationComponent implements AfterViewInit{
  @ViewChild('body') body: ElementRef<HTMLBodyElement>;
  
  @ViewChild('vizualization_div') vizualization_div: ElementRef<HTMLDivElement>;
  private svg: Svg;

  @ViewChild('rss_view') rss_view: RssViewComponent;
  protected rss_view_enabled: boolean = null;
  protected rss_view_drawer_enabled: boolean = false;
  
  private y_correction = 70;

  private datapoints: Array<DataPoint>;

  constructor(private route: ActivatedRoute, private svg_service: SvgService, private cdr: ChangeDetectorRef){ }

  ngOnInit(){
    this.updateDataPoints();
    console.log("Initializing dag view component with: " + this.datapoints.length + " datapoints");
    console.log(this.datapoints);

    // this.rss_view_enabled = true;
  }
  
  ngAfterViewInit(){
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;
    
    this.svg = new Svg(this.vizualization_div, width, height, this.datapoints.slice(), this.scalingFunction, 40);
    this.svg.resize(width, height, this.y_correction);
    
    
    // this.cdr.detectChanges();
  }

  private updateDataPoints(){
    if (environment.dev_mode){
      let datapoints: DataPoint[] = [
        new DataPoint(1, 0, 5.000000476837158, 2, -0.09219828993082047, -0.13575132191181183, 185, 0, 0, 0.5),
        new DataPoint(2, 0, 5.000000476837158, 2, -0.1434944123029709, -0.025769922882318497, 172, 0, 0, 0.1),
        new DataPoint(3, 0, 5.000000476837158, 2, 0.13556334376335144, 0.04386178404092789, 172, 0, 0, 0.3),
        new DataPoint(4, 0, 5.000000476837158, 2, -0.07058414816856384, 0.12081196159124374, 172, 0, 0, 0.9),
        new DataPoint(5, 0, 5.000000476837158, 2, -0.10426267236471176, 0.050395093858242035, 170, 0, 0, 1),
        new DataPoint(6, 0, 5.000000476837158, 2, 0.019231455400586128, 0.12873762845993042, 167, 0, 0, 0.6),
        new DataPoint(7, 0, 5.313292980194092, 2, -0.012485436163842678, -0.1166980192065239, 159, 0, 0, 0.7),
        new DataPoint(8, 0, 5.000000476837158, 2, 0.0902477353811264, -0.09746352583169937, 200, 0, 0, 0.9),
        new DataPoint(9, 0, 5.000000476837158, 2, 0.11125318706035614, -0.029057452455163002, 164, 0, 0, 0.1),
        new DataPoint(10, 0, 5.000000476837158, 2, 0.07510804384946823, 0.07280625402927399, 164, 0, 0, 0.1),
        new DataPoint(11, 0, 10.591045379638672 ,2 ,0.017545919865369797 ,-0.0036888536997139454 ,108 ,0 ,0, 0.6),
        new DataPoint(12, 0, 9.654894828796387 ,2 ,-0.0016832565888762474 ,-0.0028650283347815275 ,250 ,0 ,0, 0.8),
        new DataPoint(13, 0, 8.077374458312988 ,2 ,-0.022442487999796867 ,0.0031338557600975037 ,119 ,0 ,0, 0.2),
        new DataPoint(14, 0, 13.373766899108887 ,2 ,0.0032756526488810778 ,0.0005078032845631242 ,101 ,0 ,0, 0.6),
        new DataPoint(15, 0, 12.493330955505371 ,2 ,-0.01358407735824585 ,-0.009488344192504883 ,180 ,0 ,0, 0.4),
        new DataPoint(16, 0, 9.654894828796387 ,2 ,-0.020130377262830734 ,-0.032513659447431564 ,112 ,0 ,0, 0.3),
        new DataPoint(17, 0, 8.406118392944336 ,2 ,-0.012485436163842678 ,-0.03319296985864639 ,117 ,0 ,0, 0.9),
        new DataPoint(18, 0, 8.88748836517334 ,2 ,-0.010542476549744606 ,-0.021444378420710564 ,70 ,0 ,0, 0.15),
        new DataPoint(19, 0, 8.737260818481445 ,2 ,-0.010542476549744606 ,-0.021444378420710564 ,116 ,0 ,0, 0.7),
        new DataPoint(20, 0, 8.88748836517334  ,2 ,-0.020931201055645943 ,-0.015955956652760506, 30, 0, 0, 1)
  
        // new DataPoint(1, 0, 5.000000476837158, 2, -1, 1, 185, 70, 0),
        // new DataPoint(2, 0, 5.000000476837158, 2, 1, 1, 172, 83, 0),
        // new DataPoint(3, 0, 5.000000476837158, 2, -1, -1, 172, 83, 0),
        // new DataPoint(4, 0, 5.000000476837158, 2, 1, -1, 172, 83, 0),
        // new DataPoint(4, 0, 5.000000476837158, 2, 0, 0, 172, 83, 0),
  
      ];

      this.datapoints = datapoints;
      if(this.svg != undefined){ this.svg.setDatapoints(this.datapoints); }
      return;
    }

    invoke("getDataPoints").then((result: Array<DataPoint>) =>{
      this.datapoints = result;
      if(this.svg != undefined){ this.svg.setDatapoints(this.datapoints); }
      
    }).catch((error: any) => {
      console.log(error);
    });
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
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.svg.resize(width, height, this.y_correction);
  }

  protected enableRssView(){
    this.rss_view_enabled = true;
    this.cdr.detectChanges();

    setTimeout(() => {
      this.rss_view_drawer_enabled = false;
      this.cdr.detectChanges();
    }, 200);
  }

  protected disableRssView(){
    this.rss_view_enabled = false;
    this.cdr.detectChanges();

    setTimeout(() => {
      this.rss_view_drawer_enabled = true;
      this.cdr.detectChanges();
    }, 400);
  }

  public onTruncation(event){
    let new_size = event;
    console.log("Truncating datapoints to only: " + new_size);

    if(environment.dev_mode) {
      this.updateDataPoints(); // Resetting to all original datapoints in dev mode

      let new_datapoints = [];

      for(let i = 0; i < new_size; i++){
        new_datapoints.push(this.datapoints[i]);
      }

      this.datapoints = new_datapoints; // Truncation
      if(this.svg != undefined){ this.svg.setDatapoints(this.datapoints); } // Updating the svg
      return;
    }

    invoke("truncateModel", {newSize: new_size}).then((result: any) => { // This performs the truncation
      this.updateDataPoints(); // This updates to the the new truncated datapoints
      
    }).catch((error: any) => {
      console.log(error);
    });
  }

}
