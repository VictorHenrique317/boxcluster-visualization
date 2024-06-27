import { EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';
import { SvgFeatureModule } from './svg-feature.module';
import * as d3 from 'd3';
import { environment } from 'src/environments/environment';
import { fs, invoke } from '@tauri-apps/api';
import { DialogService } from 'src/app/services/dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { IntersectionDetailsDialogComponent } from './intersection-details-dialog/intersection-details-dialog.component';
import { IntersectionDetails } from 'src/app/models/intersection_details';
import { ApiService } from 'src/app/services/api/api.service';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class IntersectionModeFeatureModule {
  private intersection_mode:boolean = false;
  private old_clicked_datapoint = null;
  private clicked_datapoint_data: DataPoint = null;
  private intersection_details: IntersectionDetails;
  private transition_duration: number = 300;

  private svg_feature: SvgFeatureModule;
  private dialog_service: DialogService;
  private api_service: ApiService

  constructor(svg_feature: SvgFeatureModule, dialog_service: DialogService, api_service: ApiService) {
    this.svg_feature = svg_feature;
    this.dialog_service = dialog_service;
    this.api_service = api_service;
  }

  private connectDatapoints(center: DataPoint, intersections:Map<number, number>){
    let svg_circles = this.svg_feature.plot.selectAll('.datapoint');
    let id_to_datapoint = new Map<number, DataPoint>(svg_circles.data()
      .map(d => [d.identifier, d]));

    for(let [identifier, percentage] of intersections.entries()){
      if(identifier == this.clicked_datapoint_data.identifier){ continue; } // itself
      if(identifier == 0){ continue; } // Excess intersections
      let related_datapoint = id_to_datapoint.get(identifier) || null;
      if (related_datapoint == null) { continue; } // Related circle is a subpattern

      let related_circle = svg_circles.filter(d => d.identifier == identifier);

      let stroke_width = 6 * percentage + 2; // 2 to 8

      let x1 = this.svg_feature.xScale(center.x);
      let y1 = this.svg_feature.yScale(center.y);
      let line = this.svg_feature.plot.append('line')
        .datum({identifier:identifier, x1: x1, y1: y1})  // Bind the original coordinates to the line
        .raise()
        .attr('class', 'intersection_line')
        .attr('x1', this.svg_feature.xScale(center.x))  // Start position (x) of the line
        .attr('y1', this.svg_feature.yScale(center.y))  // Start position (y) of the line
        .attr('x2', this.svg_feature.xScale(center.x))  // Initially, end position (x) is the same as start position
        .attr('y2', this.svg_feature.yScale(center.y))  // Initially, end position (y) is the same as start position
        .attr('stroke', 'rgba(255, 0, 0, 0.5)')
        .attr('stroke-width', stroke_width)
        .on('mouseover', (event, l) => {
          d3.select(event.currentTarget).style('cursor', 'pointer');
          d3.select(event.currentTarget).attr('stroke-width', stroke_width * 3);
        })
        .on('mouseout', (event, l) => {
          d3.select(event.currentTarget).style('cursor', 'default');
          d3.select(event.currentTarget).attr('stroke-width', stroke_width);
        })
        .on('click', (event, l) => {
          this.showIntersectionDetails(l.identifier);
        })
        .transition('mouseover')
        .duration(this.transition_duration*2)
        .attr('x2', this.svg_feature.xScale(related_datapoint.x))  // Actual end position (x) of the line
        .attr('y2', this.svg_feature.yScale(related_datapoint.y))  // Actual end position (y) of the line
    }
  }

  private highlightDatapoints(relationed_identifiers: Array<number>){
    let identifiers_set = new Set(relationed_identifiers);
    let gray_shade = 196;
    let gray = `rgba(${gray_shade}, ${gray_shade}, ${gray_shade}, 0.5)`;

    this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => !identifiers_set.has(d.identifier) && d.identifier != this.clicked_datapoint_data.identifier)
      .raise()
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('fill', d => gray)
      .style('stroke', d=> gray);

    // let highligthed_circles = this.svg_feature.plot.selectAll('.datapoint')
    //   .filter(d => identifiers_set.has(d.identifier));

    // highligthed_circles
    //   .raise()
    //   .transition('mouseover')
    //   .duration(this.transition_duration)
    //   .attr('fill', d => gray)
    //   .style('stroke', d=> gray);
  }

  private expandCircle(clicked_circle, expansion_factor, intersections, intersections_colors){
    clicked_circle
      .attr('r', this.clicked_datapoint_data.size)
      .transition('mouseover')
      .duration(this.transition_duration)
      .attr('r', this.clicked_datapoint_data.size * expansion_factor)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, 1)`);
  }

  private createIntersectionChart(root_circle: any, intersections: Map<number, number>, original_radius: number, chart_radius: number){
    let root_datapoint = root_circle.node().__data__;
    let pie = d3.pie()
      .value((d: any) => d.value);

    let data: Array<any> = Array.from(intersections, ([key, value]) => ({key, value}));
    let pie_data = pie(data);
    
    let original_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => original_radius);
    let pie_chart_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(chart_radius);

    let pie_group = this.svg_feature.plot.append('g')
      .attr('class', 'pie_chart')
      .attr('transform', `translate(${root_circle.attr('cx')}, ${root_circle.attr('cy')})`);

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
        let related_datapoint = this.svg_feature.getDatapoint(d.data.key);
        let r = 130;
        let g = 0;
        let b = 173;
        let a = 1;

        if(related_datapoint){ // If it isnt id 0 (which means total intersection to the clicked datapoint)
          // Dont color the percetage related to intersection with itself   
          if(related_datapoint.identifier == root_datapoint.identifier){ a = 0; }
        }

        let color = `rgba(${r}, ${g}, ${b}, ${a})`;
        return color;
      });
  }

  private createIntersectionCharts(identifiers: Array<number>, intersections: Map<number, number>){
    let clicked_datapoint = this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => d.identifier == this.clicked_datapoint_data.identifier);

    let intersection_data: Map<number, number>  = new Map<number, number>();
    let parent_current_percentage = intersections.get(this.clicked_datapoint_data.identifier);
    let complement_percentage = 1 - parent_current_percentage; // Colored with current circle color
    intersection_data.set(0, parent_current_percentage);
    intersection_data.set(this.clicked_datapoint_data.identifier, complement_percentage);

    let original_radius = this.clicked_datapoint_data.size;
    let chart_radius = this.clicked_datapoint_data.size;
    this.createIntersectionChart(clicked_datapoint, intersection_data, original_radius, chart_radius);
    intersections.delete(this.clicked_datapoint_data.identifier);
  
    let identifiers_set = new Set(identifiers);
    let circles = this.svg_feature.plot.selectAll('.datapoint')
      .filter(d => identifiers_set.has(d.identifier));

    circles.each((d, i, nodes) => {
      intersection_data = new Map<number, number>();
      let parent_current_percentage = intersections.get(d.identifier); // Colored with the parent color
      let complement_percentage = 1 - parent_current_percentage; // Colored with current circle color

      intersection_data.set(this.clicked_datapoint_data.identifier, parent_current_percentage);
      intersection_data.set(d.identifier, complement_percentage);

      original_radius = d.size;
      chart_radius = d.size;
      this.createIntersectionChart(d3.select(nodes[i]), intersection_data, original_radius, chart_radius);
    });
  }

  private async showIntersections(){
    if(this.clicked_datapoint_data == null){ return };

    let intersections = await this.api_service.getIntersectionsPercentages(this.clicked_datapoint_data.identifier);

    let relationed_datapoints: Array<number> = Array.from(intersections.keys())
      .filter(d => (d != this.clicked_datapoint_data.identifier) && (d != 0));

    this.highlightDatapoints(relationed_datapoints);
    this.connectDatapoints(this.clicked_datapoint_data, intersections);
    let expansion_factor = 1;
    // this.expandCircle(clicked_circle, expansion_factor, intersections, intersections_colors);
    this.createIntersectionCharts(relationed_datapoints, intersections);
  }

  private async hideIntersections(){
    let intersection_lines = this.svg_feature.svg.selectAll('.intersection_line');
    intersection_lines
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('x2', d => d.x1)  // End position (x) becomes the start position
      .attr('y2', d => d.y1)  // End position (y) becomes the start position
      .remove();

    let circles = this.svg_feature.plot.selectAll('.datapoint');
    circles
      .transition('mouseout')
      .duration(this.transition_duration)
      .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
      .attr('r', d => d.size)
      .style('stroke', `rgba(255, 0, 0, 1)`);

    if(this.clicked_datapoint_data != null){
      let circle_arc = d3.arc()
      .innerRadius(0)
      .outerRadius(d => this.clicked_datapoint_data.size);

      let pie_chart = this.svg_feature.svg.selectAll('.pie_chart');
      pie_chart.selectAll('path')
        .transition('mouseout')
        .duration(this.transition_duration)
        .attr('d', d=> d.size)
        .remove();  // Remove the paths after the transition
    }
  }

  public async toggleIntersections(identifier: number){
    this.hideIntersections();
    await this.updateClickedDatapoint(identifier);

    if(identifier == null || identifier==undefined){return;}

    if((this.old_clicked_datapoint != null) && (identifier == this.old_clicked_datapoint.identifier)){ // Datapoint was clicked again
      await this.updateClickedDatapoint(null);
    }

    this.showIntersections();
  }

  private async updateClickedDatapoint(identifier: number) {
    this.old_clicked_datapoint = this.clicked_datapoint_data;

    if(identifier == null){
      this.clicked_datapoint_data = null;
      this.intersection_details = null;
      return;
    }

    let clicked_circle = this.svg_feature.plot.selectAll('.datapoint')
      // .filter(d => d.identifier == 13); // Fix black color
      .filter(d => d.identifier == identifier);
    this.clicked_datapoint_data = clicked_circle.node().__data__;

    this.intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);
  }

  public clickedPatternHasIntersections(): boolean {
    if(this.intersection_details == null){
      return false;
    }

    return this.intersection_details.intersections.size > 0;
  }

  public async showIntersectionDetails(intersector_id: number){
    if(this.clicked_datapoint_data == null){
      console.warn("No clicked datapoint to show details.");
      return;
    }

    let intersection_details = await this.api_service.getIntersectionDetails(this.clicked_datapoint_data.identifier);

    let dialog_data = {
      intersector: intersector_id,
      intersection_details: intersection_details
    }

    this.dialog_service.open(IntersectionDetailsDialogComponent, 
      IntersectionDetailsDialogComponent.WIDTH, 
      IntersectionDetailsDialogComponent.HEIGHT, 
      dialog_data);
  }
}
