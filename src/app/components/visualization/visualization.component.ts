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
import { ActivatedRoute } from '@angular/router';
import { RssViewComponent } from 'src/app/components/main_options/rss-view/rss-view.component';
import { environment } from '../../../environments/environment';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { DataPointTooltipComponent } from "./datapoint-tooltip/datapoint-tooltip.component";
import { DatapointInfoDialogComponent } from "./datapoint-info-dialog/datapoint-info-dialog.component";
import { Pattern } from "src/app/models/pattern";
import { DialogService } from "src/app/services/dialog/dialog.service";
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';

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
  private svg: any;
  private plot: any;

  private zoom_level: number;
  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;
  
  private tooltip;

  private intersection_mode: boolean = false;

  constructor(public dialog_service: DialogService, private cdr: ChangeDetectorRef){ }
  ngAfterViewInit(): void {
    console.log("Initializing visualization component");
    this.svg_width = this.body.nativeElement.clientWidth;
    this.svg_height = this.body.nativeElement.clientHeight;

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
    
    this.svg = this.createSvg();
    this.resizeSvg(this.svg_width, this.svg_height);
    this.cdr.detectChanges();
    
    this.zoom_level = this.initial_scale;

    this.updateDataPoints();
  }

  public async updateDataPoints(){
    console.log("Invoking getDataPoints");
    
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDataPoints").catch((error: any) => {
        console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching data points.");
      });

    } else if (environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints2.json'));
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
    let max_module = Math.max(x_max_module, y_max_module);

    let scaled_datapoints: Array<DataPoint> = datapoints;
    // let screen_coverage = 0.05;
    let screen_coverage = 0.5;
    datapoints.forEach(datapoint => {
        // if(max_module > -0.5 && max_module < 0.5){
        //   max_module = 1;
        // }

        let result_x = datapoint.x / x_max_module;
        let result_y = datapoint.y / y_max_module;

      if (isNaN(result_x) || !isFinite(result_x)) {
          result_x = datapoint.x;
      }

      if (isNaN(result_y) || !isFinite(result_y)) {
          result_y = datapoint.y;
      }

        // datapoint.x /= x_max_module * ((1-screen_coverage) + 1);
        // datapoint.y /= y_max_module * ((1-screen_coverage) + 1);

        datapoint.x = result_x / ((1-screen_coverage) + 1);
        datapoint.y = result_y / ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  private drawDataPoints() {
    if(this.plot == undefined){ return; }

    this.plot.call(this.tooltip);
  
    let scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('circle')
        .data(scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
  
    circles.exit()
        .transition() // Add exit animation
        .duration(1000) // Duration of the animation in milliseconds
        .attr('r', 0) // Reduce radius to 0
        .remove(); // Remove datapoints that are not in the new dataset
  
    circles.transition() // Animate existing datapoints
        .duration(1000) // Duration of the animation in milliseconds
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)));
  
    circles.enter().append('circle') // Add new datapoints with animation
        .attr('cx', d => {
            const result = this.x_scale(parseFloat(d.x));
            return result;
        })
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', 0) // Start from radius 0
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer') // Set cursor to pointer
        .style('stroke', 'rgba(255, 0, 0, 1')
        .on('mouseover', (event, d) => { this.tooltip.show(d, event.currentTarget); })
        .on('mouseout', (event, d) => { this.tooltip.hide(d, event.currentTarget); })
        .on('click', (event, d) => { this.onDatapointClick('300ms', '300ms', d.identifier); })
        .transition() // Transition to final state
        .duration(1000) // Duration of the animation in milliseconds
        .attr('r', d => d.size); // End with actual radius
    
    this.drawCircleLegend();
    this.drawColorLegend();
  }

  private async getRawPattern(identifier: number){
    let pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
      console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching pattern.");
    });

    return pattern;
  }

  private drawCircleLegend(){
    let min_pattern_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let max_pattern_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let mean_pattern_size = 0;
    for(let i = 0; i < this.datapoints.length; i++){
      mean_pattern_size += this.datapoints[i].pattern_size;
    }
    mean_pattern_size /= this.datapoints.length;

    let min_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;
    let max_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;

    let legend = legendCircle(null)
      .scale(
        d3.scaleLinear()
            .domain([min_pattern_size, max_pattern_size])
            .range([min_size, max_size])
      )
      .tickValues([min_pattern_size, mean_pattern_size, max_pattern_size])
      .tickFormat((d, i, e) => `${d}${i === e.length - 1 ? " Cells" : ""}`)
      .tickSize(max_size); // defaults to 5
    
    const svg_width = this.svg.attr('width');
    const svg_height = this.svg.attr('height');
    const legend_x_padding = 10;
    const legend_y_padding = 10;
  
    // Remove the old legend if it exists
    this.svg.select("#circle_legend").remove();
  
    this.svg.append("g")
      .attr('id', 'circle_legend') // Add a unique id to the legend
      .attr('transform', `translate(${legend_x_padding}, ${legend_y_padding})`)
      .call(legend);
  }

  private drawColorLegend(){
    let oldLegend = document.getElementById("color_legend");
    if(oldLegend){
        oldLegend.parentNode.removeChild(oldLegend);
    }

    let svg_width = this.svg.attr('width');
    let legend_width = 320;
    const legend_x_padding = 10;

    let legend_x = svg_width - (legend_width + legend_x_padding);

    let legend = Legend(d3.scaleLinear([0, 1], ["rgba(255,0,0,0)", "red"]), {
      title: "Density",
      width: legend_width,
    })

    let legendGroup = this.svg.append('g')
      .attr('id', 'color_legend')
      .attr("transform", `translate(${legend_x}, 0)`);
      
    legendGroup.node().appendChild(legend);
}

  private async onDatapointClick(enterAnimationDuration: string, exitAnimationDuration: string, identifier: number): Promise<void> {
    let pattern;
    if(!environment.dev_mode){
      pattern = await this.getRawPattern(identifier);

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/pattern.json'));
      pattern = JSON.parse(rawdata);
    }

    let dialog_data = {
      pattern: pattern
    }
    this.dialog_service.open(DatapointInfoDialogComponent, '500px', '590px', dialog_data);
  }

  public onResize(event) {
    let width = this.body.nativeElement.clientWidth;
    let height = this.body.nativeElement.clientHeight;

    this.resizeSvg(width, height);
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
      console.error(error);
      this.dialog_service.openErrorDialog("Error while truncating datapoints.");
    });
  }

  private highlightDatapoint(datapoint: DataPoint){
    let circles = this.plot.selectAll('circle');
    let duration = 300;
    let circles_visibility = 0.2;

    circles
      .transition()
      .duration(duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a*circles_visibility})`)
      .style('stroke', `rgba(255, 0, 0, ${circles_visibility})`)
      .style('stroke-dasharray', '1,1');

    circles
      .filter(d => d.identifier == datapoint.identifier)
      .transition()
      .duration(duration)
      .style('stroke', 'rgba(255, 0, 0, 1)')
      .style('stroke-dasharray', '0,0');
  }

  private async showIntersections(datapoint: DataPoint, event){
    this.highlightDatapoint(datapoint);

    let intersections:any;
    if(!environment.dev_mode){
      intersections = await invoke("getIntersectionPercentagesFor", {identifier: datapoint.identifier})
        .catch((error: any) => {
          console.error(error);
          this.dialog_service.openErrorDialog("Error while getting intersections.");
        });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/intersections.json'));
      intersections = JSON.parse(rawdata);
    }

    for(const identifier in intersections){
      let percentage = intersections[identifier];

      let related_circle = this.plot.selectAll('circle');
      related_circle = related_circle.filter(d => d.identifier == identifier);
      
      let transform = d3.zoomTransform(this.plot.node());
      this.svg.append('line')
        .attr('class', 'intersection_line')
        .attr('pointer-events', 'none')
        .attr('x1', transform.applyX(this.x_scale(datapoint.x)))  // Start position (x) of the line
        .attr('y1', transform.applyY(this.y_scale(datapoint.y)))  // Start position (y) of the line
        .attr('x2', transform.applyX(related_circle.attr('cx')))  // End position (x) of the line
        .attr('y2', transform.applyY(related_circle.attr('cy')))  // End position (y) of the line
        .attr('stroke', 'black')
        .attr('stroke-width', 2);
            
    }
  }


  private removeHighlight(){
    let circles = this.plot.selectAll('circle');
    let duration = 300;

    circles
      .transition()
      .duration(duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .style('stroke', `rgba(255, 0, 0, 1)`)
      .style('stroke-dasharray', '1,1');

    let intersection_lines = this.svg.selectAll('.intersection_line');
    intersection_lines.remove();
  }

  private hiddeIntersections(){
    this.removeHighlight();
  }

  public isOnIntersectionMode(){
    return this.intersection_mode;
  }

  public toggleIntersectionMode(){
    this.intersection_mode = !this.intersection_mode;
    let circles = this.plot.selectAll('circle');
    
    let duration = 300;
    if(this.intersection_mode == true){ // Activate intersection mode
        circles
          .on('mouseover', (event, d) => { this.showIntersections(d, event) })
          .on('mouseout', (event, d) => { this.hiddeIntersections(); })
          .on('click', (event, d) => {})
          .transition()
          .duration(duration)
          .style('stroke', 'rgba(255, 0, 0, 1)')
          .style('stroke-dasharray', '1,1');

        

        

      return;
    }
  
    if(this.intersection_mode == false){ // Deactivate intersection mode);
      circles
        .on('mouseover', (event, d) => { this.tooltip.show(d, event.currentTarget); })
        .on('mouseout', (event, d) => { this.tooltip.hide(d, event.currentTarget); })
        .on('click', (event, d) => { this.onDatapointClick('300ms', '300ms', d.identifier); })
        .transition()
        .duration(duration)
        .style('stroke-dasharray', '0,0');
  
      return;
    }
  }

  
  


  // ========================= SVG FUNCTIONS ========================= //
  private createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
      .append('svg')
        .attr('width', this.svg_width)
        .attr('height',this.svg_height);

    return svg;
  }

  public resizeSvg(width: number, height: number){
    this.svg
      .attr('width', width)
      .attr('height', height);

    let x_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([0, (height - this.y_correction)/1]);

    let y_scale = d3.scaleLinear()
      .domain([-1, 1])
      .range([(height - this.y_correction)/1, 0]);

    this.x_scale = x_scale;
    this.y_scale = y_scale;
    this.svg_width = width;
    this.svg_height = height;

    this.createPlot();
  }

  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g").attr("id", "plot");
    
    let panning_zoom = d3.zoom()
      .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
      // .translateExtent([[0, 0], [this.height, this.height/1.2]])
      .translateExtent([[0, 0], [this.svg_height, this.svg_height]])
      .on("start", (event, d) => { this.svg.attr("cursor", "grabbing"); })
      .on("zoom", (event) => { 
        this.plot.attr("transform", event.transform); 
        if(event.sourceEvent instanceof WheelEvent){
          this.zoom_level = event.transform.k;
          this.drawCircleLegend();
        }
      })
      .on("end", (event, d) => {this.svg.attr("cursor", "default")});

    this.svg.call(panning_zoom);

    // Apply initial zoom level
    let x_translation_factor = 0.0;
    // let y_translation_factor = 0.15;
    let y_translation_factor = 0.2;
    let initial_transform = d3.zoomIdentity
      .translate(-this.svg_width*(x_translation_factor), -this.svg_height*(y_translation_factor))
      // .translate(-this.width*(x_translation_factor), 0)
      .scale(this.initial_scale);
    this.svg.call(panning_zoom.transform, initial_transform);
    
    this.drawGridLines();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
    let grey_tonality = 220;
    let color = `rgb(${grey_tonality}, ${grey_tonality}, ${grey_tonality})`;
    // Add the X gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", color)
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    // Add the Y gridlines
    this.plot.append("g")			
      .attr("class", "grid")
      .attr("color", color)
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }

  // ========================= SVG FUNCTIONS ========================= //
}
