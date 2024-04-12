import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import * as d3 from 'd3';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { race } from 'rxjs';
import { IntersectionDetailsDialogComponent } from './intersection-details-dialog/intersection-details-dialog.component';
import { IntersectionDetails } from 'src/app/models/intersection_details';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class IntersectionModeFeatureModule {
  private intersection_mode:boolean = false;
  private clicked_datapoint_data: DataPoint;
  private transition_duration: number = 300;

  private svg_feature: SvgFeatureModule;
  private dialog_service: DialogService;

  constructor(svg_feature: SvgFeatureModule, dialog_service: DialogService) {
    this.svg_feature = svg_feature;
    this.dialog_service = dialog_service;
  }

  private connectDatapoints(center: DataPoint, intersections:Map<number, number>, intersections_colors: Map<number, string>){
    let circles = new Map<number, DataPoint>(this.svg_feature.plot.selectAll('circle').data()
      .map(d => [d.identifier, d]));

    for(let [identifier, percentage] of intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){ continue; } // itself
      if(identifier == 0){ continue; } // Excess intersections

      let stroke_width = 6 * percentage + 2; // 2 to 8

      let x1 = this.svg_feature.xScale(center.x);
      let y1 = this.svg_feature.yScale(center.y);
      let line = this.svg_feature.plot.append('line')
        .datum({x1: x1, y1: y1})  // Bind the original coordinates to the line
        .attr('class', 'intersection_line')
        .attr('pointer-events', 'none')
        .attr('x1', this.svg_feature.xScale(center.x))  // Start position (x) of the line
        .attr('y1', this.svg_feature.yScale(center.y))  // Start position (y) of the line
        .attr('x2', this.svg_feature.xScale(center.x))  // Initially, end position (x) is the same as start position
        .attr('y2', this.svg_feature.yScale(center.y))  // Initially, end position (y) is the same as start position
        .attr('stroke', intersections_colors.get(identifier))
        .attr('stroke-width', stroke_width);
      
      let related_circle = circles.get(identifier);
      line
        .transition('mouseover')
        .duration(this.transition_duration*2)
        .attr('x2', this.svg_feature.xScale(related_circle.x))  // Actual end position (x) of the line
        .attr('y2', this.svg_feature.yScale(related_circle.y));  // Actual end position (y) of the line
    }
  }

  private highlightDatapoints(identifiers: Array<number>, intersections_colors: Map<number, string>){
    let circles = this.svg_feature.plot.selectAll('circle');
    let circles_visibility = 0.2;

    circles
      .raise()
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a*circles_visibility})`)
      .style('stroke', `rgba(255, 0, 0, ${circles_visibility})`);

    let identifiers_set = new Set(identifiers);
    let highligthed_circles = circles.filter(d => identifiers_set.has(d.identifier));
    highligthed_circles
      .raise()
      .transition('mouseover')
      .duration(this.transition_duration)
      // .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .attr('fill', d => intersections_colors.get(d.identifier))
      // .style('stroke', `rgba(255, 0, 0, 1)`)
      .style('stroke', d=> intersections_colors.get(d.identifier))
      .style('stroke-dasharray', '10000,10000');
  }

  private createIntesectionColorMapping(intersections: Map<number, number>): Map<number, string>{
    let colors = ["#eb4da3", "#a614b3", "#8a4aed", "#1731e8", "#3ea6ed", "#0c8e81"]

    let sorted_intersections = new Map(Array.from(intersections.entries()).sort((a, b) => b[1] - a[1]));

    let intersections_colors: Map<number, string> = new Map();
    let i = 0;
    for(const [identifier, percentage] of sorted_intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){
        intersections_colors.set(identifier, "#d71610");
        continue;
      }

      if(i > colors.length - 1){ console.warn("Not enough colors for intersections.");}
      intersections_colors.set(identifier, colors[i % colors.length]);
      i++;
    }

    return intersections_colors;
  }

  private createIntersectionChart(circle: any, intersections: Map<number, number>, chart_radius: number, 
    intersections_colors: Map<number, string>): Map<number, string>{
    let pie = d3.pie()
      .value((d: any) => d.value);

    let data: Array<any> = Array.from(intersections, ([key, value]) => ({key, value}));
    let pie_data = pie(data);
    
    let original_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => this.clicked_datapoint_data.size);
    let pie_chart_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(chart_radius);

    let pie_group = this.svg_feature.plot.append('g')
      .attr('class', 'pie_chart')
      .attr('transform', `translate(${circle.attr('cx')}, ${circle.attr('cy')})`);

    pie_group.selectAll('path')
      .data(pie_data)
      .enter()
      .append('path')
      .attr('pointer-events', 'none')
      .attr('d', original_arc)
      .attr('fill', 'red')
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('d', pie_chart_arc)
      .attr('fill', (d: any) => {
        let color = intersections_colors.get(d.data.key);
        return color;
      });

    return intersections_colors;
  }

  private async showIntersections(datapoint: DataPoint, event){
    let raw_data;
    let clicked_circle: any;
    if(!environment.dev_mode){
      clicked_circle = this.svg_feature.plot.selectAll('circle')
        .filter(d => d.identifier == datapoint.identifier);

      this.clicked_datapoint_data = clicked_circle.node().__data__;

      raw_data = await invoke("getIntersectionsPercentages", {identifier: this.clicked_datapoint_data.identifier})
        .catch((error: any) => {
          console.error(error);
          this.dialog_service.openErrorDialog("Error while getting intersections.");
      });
      
    }else{
      clicked_circle = this.svg_feature.plot.selectAll('circle')
          // .filter(d => d.identifier == 14);
          .filter(d => d.identifier == datapoint.identifier);

      this.clicked_datapoint_data = clicked_circle.node().__data__;

      raw_data = await fs.readTextFile(await resolveResource('resources/intersections2.json'));
      raw_data = JSON.parse(raw_data);
    }

    console.log("Fetched intersections for datapoint: " + this.clicked_datapoint_data.identifier);
    console.log(raw_data);

    let intersections = new Map<number, number>();
    for (let key in raw_data) { intersections.set(Number(key), Number(raw_data[key])); }

    let intersections_colors = this.createIntesectionColorMapping(intersections);

    this.highlightDatapoints(Array.from(intersections.keys()), intersections_colors);
    this.connectDatapoints(this.clicked_datapoint_data, intersections, intersections_colors);

    let expansion_factor = 4;
    clicked_circle
      .attr('r', this.clicked_datapoint_data.size)
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('r', this.clicked_datapoint_data.size * expansion_factor)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, 1)`);
    let chart_radius = this.clicked_datapoint_data.size * expansion_factor;
    this.createIntersectionChart(clicked_circle, intersections, chart_radius, intersections_colors);
  }

  private hideIntersections(){
    let intersection_lines = this.svg_feature.svg.selectAll('.intersection_line');
    intersection_lines
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('x2', d => d.x1)  // End position (x) becomes the start position
      .attr('y2', d => d.y1)  // End position (y) becomes the start position
      .remove();

    let circles = this.svg_feature.plot.selectAll('circle');
    circles
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .attr('r', d => d.size)
      .style('stroke', `rgba(255, 0, 0, 1)`)
      .style('stroke-dasharray', '1,1');

    if(this.clicked_datapoint_data != null){
      let circle_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => this.clicked_datapoint_data.size);

      let pie_chart = this.svg_feature.svg.selectAll('.pie_chart');
      pie_chart.selectAll('path')
        .transition('mouseout')
        .duration(this.transition_duration)
        .attr('d', circle_arc)
        .remove();  // Remove the paths after the transition
    }

    this.clicked_datapoint_data = null;
  }

  public isOnIntersectionMode(){
    return this.intersection_mode;
  }

  public toggleIntersectionMode(){
    this.intersection_mode = !this.intersection_mode;
    
    let circles = this.svg_feature.plot.selectAll('circle'); 
    if(this.intersection_mode){ // Activate intersection mode
      console.log("Intersection mode activated.");
      this.svg_feature.plot.append('rect')
        .attr('id', 'overlay')
        .attr('x', 0)
        .attr('y', 0)
        .attr('width', this.svg_feature.getSvgWidth())
        .attr('height', this.svg_feature.getSvgHeight())
        .style('fill', 'rgba(0, 0, 0, 0)')
        .lower()
        .on('click', (event, d) => { this.hideIntersections(); });

      circles
        .on('click', (event, d) => {
          let old_clicked_datapoint: DataPoint = null;
          if(this.clicked_datapoint_data != null || this.clicked_datapoint_data != undefined){
            old_clicked_datapoint = this.clicked_datapoint_data;
          }

          this.hideIntersections();

          if(old_clicked_datapoint == null){
            // No datapoint was clicked before, show intersections
            this.showIntersections(d, event);

          }else if(old_clicked_datapoint.identifier != d.identifier){
            // Did not click the same datapoint, show intersections
            this.showIntersections(d, event)
          }
         })
        .transition()
        .duration(this.transition_duration)
        .style('stroke', 'rgba(255, 0, 0, 1)')
        .style('stroke-dasharray', '1,1');
    }

    else if(!this.intersection_mode){ // Deactivate intersection mode
      console.log("Intersection mode deactivated.");
      this.svg_feature.plot.selectAll('#overlay').remove();

      this.hideIntersections();

      this.svg_feature.resetDatapointEvents();
      circles
        .transition()
        .duration(this.transition_duration)
        .style('stroke-dasharray', '10000,10000');
    }
  }

  public getClickedDatapoint(){
    return this.clicked_datapoint_data;
  }

  public async showIntersectionDetails(){
    if(this.clicked_datapoint_data == null){
      console.warn("No clicked datapoint to show details.");
      return;
    }

    let data: any;
    if(!environment.dev_mode){
      data = await invoke("getIntersectionDetails", {identifier: this.clicked_datapoint_data.identifier})

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/intersection_details.json'));
      data = JSON.parse(rawdata);
    }

    let intersections: Map<number, [number, Array<Array<string>>]> = new Map();
    for (let key in data.intersections) { 
      let value = data.intersections[key];
      let percentage = Math.round(value[0]*100)/100;
      let dims_intersections = value[1];
      intersections.set(Number(key), [percentage, dims_intersections]);
    }

    let intersection_details: IntersectionDetails = new IntersectionDetails(
      data.identifier,
      Math.round(data.total_untouched_percentage * 100)/100,
      Math.round(data.total_intersection_percentage * 100)/100,
      intersections
    );

    let dialog_data = {
      intersection_details: intersection_details
    }

    this.dialog_service.open(IntersectionDetailsDialogComponent, 
      IntersectionDetailsDialogComponent.WIDTH, 
      IntersectionDetailsDialogComponent.HEIGHT, 
      dialog_data);
  }
}
