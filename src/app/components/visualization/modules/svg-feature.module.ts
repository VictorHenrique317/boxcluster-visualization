import * as d3Tip from "d3-tip";
import { ChangeDetectorRef, ElementRef, EventEmitter, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import * as d3 from 'd3';
import { DataPoint } from 'src/app/models/datapoint';
import { legendCircle } from 'src/js/circle_legend.js';
import { Legend } from 'src/js/color_legend.js';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ]
})
export class SvgFeatureModule {
  public datapoint_hover_in = new EventEmitter<number>();
  public datapoint_hover_out = new EventEmitter<number>();
  public datapoint_click = new EventEmitter<number>();

  private locked_datapoint: DataPoint;
  private datapoints: Array<DataPoint>;
  private datapoints_mapping: Map<number, DataPoint>;
  
  private visualization_div: ElementRef;
  public plot: any;
  public svg: any;

  private zoom_level: number;
  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  private expansionFactor = 0.5;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;

  private tooltip;
  private transition_duration = 300;

  private cdr: ChangeDetectorRef;
  private api_service;

  private filters: string[][] = [];
  private truncation_size: number;

  constructor(cdr: ChangeDetectorRef){ 
    this.cdr = cdr;
  }

  public async init(visualization_div: ElementRef, svg_width: number, svg_height: number, api_service, expansionFactor: number){
    this.visualization_div = visualization_div;
    this.svg_width = svg_width;
    this.svg_height = svg_height;
    this.api_service = api_service;
    this.expansionFactor = expansionFactor

    this.tooltip = d3Tip.default()
      .attr('class', 'd3-tip')
      .offset([-10, 0])
      .html(function(d) {
        return "\
          <div style='background-color:#ededed; padding: 0.5em 0.5em 0.5em 0.5em; border-radius: 10px; border: 1px dashed black;'>\
            <strong>ID:</strong> <span style='color:#BC2602'>" + d.identifier + "</span><br>\
            <strong>Area:</strong> <span style='color:#BC2602'>" + d.pattern_size + "</span><br>\
            <strong>Density:</strong> <span style='color:#BC2602'>" + Math.round(d.density * 100) / 100 + "</span>\
          </div>\
          ";
      });
    
    this.svg = this.createSvg();
    await this.resizeSvg(this.svg_width, this.svg_height, this.datapoints);
    this.cdr.detectChanges();
    
    this.zoom_level = this.initial_scale;
  }

  public createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
      .append('svg')
        .attr('width', this.svg_width)
        .attr('height',this.svg_height)
        .on('dblclick', () => {  });

    return svg;
  }

  public async resizeSvg(width: number, height: number, datapoints: Array<DataPoint>){
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
    await this.drawDataPoints(datapoints);
  }

  private createPlot(){
    if(this.plot != undefined){ this.svg.select("#plot").remove(); }
    this.plot = this.svg.append("g")
      .attr("id", "plot")
      .on('dblclick', () => {  });
    
    let panning_zoom = d3.zoom()
      .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
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
    let y_translation_factor = 0.2;
    let initial_transform = d3.zoomIdentity
      .translate(-this.svg_width*(x_translation_factor), -this.svg_height*(y_translation_factor))
      .scale(this.initial_scale);
    this.svg.call(panning_zoom.transform, initial_transform);
    
    this.drawGridLines();
    this.drawUnselectionRect();
  }

  private drawGridLines() {
    let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
    let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
    let grey_tonality = 220;
    let color = `rgb(${grey_tonality}, ${grey_tonality}, ${grey_tonality})`;
    
    this.plot.append("g") // Add the X gridlines
      .attr("class", "grid")
      .attr("transform", "translate(0," + this.svg_height + ")")
      .attr("color", color)
      .call(makeXGridlines()
          .tickSize(-this.svg_height)
          .tickFormat(() => "")
      )

    this.plot.append("g") // Add the Y gridlines
      .attr("class", "grid")
      .attr("color", color)
      .call(makeYGridlines()
          .tickSize(-1 * this.svg_width)
          // .tickSize(-300)
          .tickFormat(() => "")
      )
  }

  private drawUnselectionRect(){
    this.plot.append('rect')
        .attr('id', 'overlay')
        .attr('x', 0)
        .attr('y', 0)
        .attr('width', this.svg_width)
        .attr('height', this.svg_height)
        .style('fill', 'rgba(255, 0, 0, 1)')
        .lower()
        .on('click', (event, d) => { 
          this.locked_datapoint = undefined;
          this.toggleHighlight(undefined);
          this.datapoint_click.emit(null) 
        });
  }

  private drawCircleLegend(){
    let min_pattern_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let max_pattern_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.pattern_size)));
    let mean_pattern_size = 0;
    for(let i = 0; i < this.datapoints.length; i++){
      mean_pattern_size += this.datapoints[i].pattern_size;
    }
    mean_pattern_size /= this.datapoints.length;
    mean_pattern_size = Math.round(mean_pattern_size);

    let min_size = Math.min(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;
    let max_size = Math.max(...this.datapoints.map(datapoint => Math.abs(datapoint.size))) * this.zoom_level;

    let legend = legendCircle(null)
      .scale(
        d3.scaleLinear()
            .domain([min_pattern_size, max_pattern_size])
            .range([min_size, max_size])
      )
      .tickValues([min_pattern_size, max_pattern_size])
      .tickFormat((d, i, e) => `${d}${i === e.length - 1 ? " cells" : ""}`)
      .tickSize(max_size); // defaults to 5
    
    const legend_x_padding = 10;
    const legend_y_padding = 10;
  
    this.svg.select("#circle_legend").remove();
    this.svg.append("g")
      .attr('id', 'circle_legend')
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

    let legend = Legend(d3.scaleLinear([0, 1], ["rgba(255,255,255,1)", "rgba(255,0,0,1)"]), {
      title: "Density",
      width: legend_width,
    })

    let legendGroup = this.svg.append('g')
      .attr('id', 'color_legend')
      .attr("transform", `translate(${legend_x}, 0)`);
      
    legendGroup.node().appendChild(legend);
  }

  private scalingFunction(datapoints: Array<DataPoint>) {
    console.log("Calling scaling function");
    // let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    // let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));
    // let max_module = Math.max(x_max_module, y_max_module);

    let scaled_datapoints: Array<DataPoint> = [...datapoints];
    let screen_coverage = 0.5;
    console.log("Screen coverage: " + screen_coverage);
    // let screen_coverage = 0.8;
    scaled_datapoints.forEach(datapoint => {
      //   let result_x = datapoint.x / x_max_module;
      //   let result_y = datapoint.y / y_max_module;

      // if (isNaN(result_x) || !isFinite(result_x)) { result_x = datapoint.x; }
      // if (isNaN(result_y) || !isFinite(result_y)) { result_y = datapoint.y; }

        datapoint.x = datapoint.x / ((1-screen_coverage) + 1);
        datapoint.y = datapoint.y / ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  private toggleHighlight(datapoint: DataPoint){
    if(this.locked_datapoint){ return; }

    let highlight_circle = this.plot.selectAll('.highlight');
    if(highlight_circle){ highlight_circle.remove(); }
    
    if(datapoint){ // Add a EMPTY circle with id highlight, the circle should not block mouse hover and click events
       // Draw a new blue circle on the coordinates of datapoint
      let highlight_radius = datapoint.size * 1.6;
      let highlight_color = 'rgba(114, 232, 247)';
      let highlight_opacity = 0.5;
      let stroke_width = highlight_radius/3;

      this.plot.append('circle')
        .attr('class', 'highlight')
        .attr('cx', this.x_scale(datapoint.x))
        .attr('cy', this.y_scale(datapoint.y))
        .attr('r', highlight_radius)
        .attr('fill', 'none')
        .attr('stroke', highlight_color)
        .attr('stroke-width', stroke_width)
        .attr('opacity', highlight_opacity)
        .style('pointer-events', 'none');

      this.plot.append('circle')
        .attr('class', 'highlight')
        .attr('cx', this.x_scale(datapoint.x))
        .attr('cy', this.y_scale(datapoint.y))
        .attr('r', highlight_radius*1.4)
        .attr('fill', 'none')
        .attr('stroke', highlight_color)
        .attr('stroke-width', stroke_width/2)
        .attr('opacity', highlight_opacity)
        .style('pointer-events', 'none');
      
    }
  }

  public deactivateHighlight(){
    this.locked_datapoint = undefined;
    this.toggleHighlight(undefined);
  }

  public async drawDataPoints(datapoints: Array<DataPoint>, force_redraw: boolean = false) {
    if(datapoints == undefined || datapoints == null){ return; }
    if(this.plot == undefined){ return; }

    if (datapoints.length > this.truncation_size){
      datapoints = datapoints.slice(0, this.truncation_size);
      console.warn("Warning: Something went wrong on truncation");
    }
    
    console.log("Received " + datapoints.length + " datapoints to draw");
  
    let transition_duration = this.transition_duration;
    if(force_redraw){ 
      this.plot.selectAll('.datapoint').remove();
      transition_duration = 0; 
    }

    this.datapoints = datapoints;
    this.datapoints_mapping = new Map<number, DataPoint>();
    this.datapoints.forEach(datapoint => this.datapoints_mapping.set(datapoint.identifier, datapoint));

    this.plot.call(this.tooltip);

    let filtered_datapoints: DataPoint[] = await this.api_service.filterDatapoints(
        this.datapoints.map(datapoint => datapoint.identifier),
        this.filters
    );
    console.log("Have " + filtered_datapoints.length + " filtered datapoints");
    console.log(filtered_datapoints);

    // let scaled_datapoints = filtered_datapoints;
    let scaled_datapoints = this.scalingFunction(filtered_datapoints);

    // if(!force_redraw){  }

    let final_datapoints = [];
    for(const datapoint of scaled_datapoints){ // Only to truncate subpatterns
      if (datapoint.identifier <= this.truncation_size) {
        final_datapoints.push(datapoint);
      }
    }

    const circles = this.plot.selectAll('.datapoint')
        .data(final_datapoints, d => d.identifier);

    circles.exit()
        .transition()
        .duration(transition_duration)
        .attr('r', 0)
        .remove(); 

    circles.transition()
        .duration(transition_duration) 
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
        .attr('class', 'datapoint')
        .attr('cy', d => this.y_scale(parseFloat(d.y)))
        .attr('r', 0)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer')
        .style('stroke', 'rgba(255, 0, 0, 1')
        .on('mouseover', (event, d) => { 
          this.toggleHighlight(d);
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier);
        })
        .on('mouseout', (event, d) => { 
          this.toggleHighlight(undefined);
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
        })
        .on('click', (event, d) => {
          if((this.locked_datapoint) && (this.locked_datapoint.identifier == d.identifier)){ 
            // Unhighlight the locked datapoint
            this.locked_datapoint = undefined;
            this.toggleHighlight(undefined);
          }else{
            // Highlight the clicked datapoint and lock
            this.locked_datapoint = undefined;
            this.toggleHighlight(d);
            this.locked_datapoint = d;
          }

          this.datapoint_click.emit(d.identifier);
         })
        .transition()
        .duration(transition_duration)
        .attr('r', d => d.size);
    
    this.drawCircleLegend();
    this.drawColorLegend();
    await this.drawTextLabels();
  }

  public async drawTextLabels(){
    this.removeTextLabels();
    for (const datapoint of this.datapoints) {
      console.log("Getting nb of subpatterns for: ", datapoint.identifier);
      let subpatterns: number[] = await this.api_service.getSubpatterns(datapoint.identifier, this.filters)
      let number = 0;

      for (const subpattern of subpatterns) {
        if (subpattern <= this.truncation_size) {
          number += 1;
        }
      }
      
      console.log("Drawing text label for: ", datapoint.identifier, " with nb: ",number);
      this.drawTextLabel(datapoint.identifier, number.toString());      
    }
  }

  public drawTextLabel(identifier: number, text: string) {
    if (text == "0") { return; }

    let datapoint = this.getDatapoint(identifier);
    if (!datapoint) { return; }

    let scaled_datapoint = this.scalingFunction([datapoint])[0];

    this.plot.append('text')
      .attr('class', 'datapoint-label')
      .attr('x', this.xScale(scaled_datapoint.x))
      .attr('y', this.yScale(scaled_datapoint.y))
      .attr('text-anchor', 'middle')
      .attr('pointer-events', 'none')
      .attr('dominant-baseline', 'central')
      .text(text);
  }

public removeTextLabels() {
    console.log("Removing text labels");
    this.plot.selectAll('.datapoint-label').remove();
}

  public setBackgroundColor(density: number) {
    let color = `rgba(255, 0, 0, ${density})`;
    this.plot.select("#overlay")
      .transition()
      .duration(300)
      .style('fill', color);
  }

  public showTooltip(datapoint: DataPoint, circle: any){
    this.tooltip.show(datapoint, circle);
  }

  public hideTooltip(datapoint: DataPoint, circle: any){
    this.tooltip.hide(datapoint, circle);
  }

  public setExpansionFactor(factor: number){
    this.expansionFactor = factor;
  }

  public setFilters(filters: string[][]){
    this.filters = filters;
  }

  public setTruncationSize(truncation_size: number){
    this.truncation_size = truncation_size;
  }

  public getExpansionFactor(){
    return this.expansionFactor;
  }

  public xScale(x: number){
    return this.x_scale(x);
  }

  public yScale(y: number){
    return this.y_scale(y);
  }

  public getDrawnDataPoints(){
    return this.datapoints;
  }

  public getSvgWidth(){
    return this.svg_width;
  }

  public getSvgHeight(){
    return this.svg_height;
  }

  public getDatapoint(identifier: number){
    return this.datapoints_mapping.get(identifier);
  }
}
