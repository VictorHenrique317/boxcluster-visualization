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

  private datapoints: Array<DataPoint>;
  
  private visualization_div: ElementRef;
  public plot: any;
  public svg: any;

  private zoom_level: number;
  private initial_scale: number = 1.4;
  private number_of_gridlines: number = 40;
  private y_correction = 0;
  
  private svg_width: number;
  private svg_height: number;
  private x_scale: any;
  private y_scale: any;

  private tooltip;
  private transition_duration = 300;

  private cdr: ChangeDetectorRef;

  constructor(cdr: ChangeDetectorRef){ 
    this.cdr = cdr;
  }

  public init(visualization_div: ElementRef, svg_width: number, svg_height: number){
    this.visualization_div = visualization_div;
    this.svg_width = svg_width;
    this.svg_height = svg_height;

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
  }

  public createSvg(){
    let svg = d3.select(this.visualization_div.nativeElement)
      .append('svg')
        .attr('width', this.svg_width)
        .attr('height',this.svg_height)
        .on('dblclick', () => {  });

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
    this.drawDataPoints(this.datapoints);
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
        .style('fill', 'rgba(0, 0, 0, 0)')
        .lower()
        .on('click', (event, d) => { this.datapoint_click.emit(null) });
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
      .tickValues([min_pattern_size, mean_pattern_size, max_pattern_size])
      .tickFormat((d, i, e) => `${d}${i === e.length - 1 ? " Cells" : ""}`)
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

    let legend = Legend(d3.scaleLinear([0, 1], ["rgba(255,0,0,0)", "red"]), {
      title: "Density",
      width: legend_width,
    })

    let legendGroup = this.svg.append('g')
      .attr('id', 'color_legend')
      .attr("transform", `translate(${legend_x}, 0)`);
      
    legendGroup.node().appendChild(legend);
  }

  private scalingFunction(datapoints: Array<DataPoint>) {
    let x_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.x)));
    let y_max_module = Math.max(...datapoints.map(datapoint => Math.abs(datapoint.y)));
    let max_module = Math.max(x_max_module, y_max_module);

    let scaled_datapoints: Array<DataPoint> = datapoints;
    let screen_coverage = 0.5;
    datapoints.forEach(datapoint => {
        let result_x = datapoint.x / x_max_module;
        let result_y = datapoint.y / y_max_module;

      if (isNaN(result_x) || !isFinite(result_x)) { result_x = datapoint.x; }
      if (isNaN(result_y) || !isFinite(result_y)) { result_y = datapoint.y; }

        datapoint.x = result_x / ((1-screen_coverage) + 1);
        datapoint.y = result_y / ((1-screen_coverage) + 1);
    });

    return scaled_datapoints;
  }

  public drawDataPoints(datapoints: Array<DataPoint>) {
    if(datapoints == undefined || datapoints == null){ return; }
    if(this.plot == undefined){ return; }
    
    this.datapoints = datapoints;

    this.plot.call(this.tooltip);

    let scaled_datapoints = this.scalingFunction(this.datapoints);
    const circles = this.plot.selectAll('circle')
        .data(scaled_datapoints, d => d.identifier);

    circles.exit()
        .transition()
        .duration(this.transition_duration)
        .attr('r', 0)
        .remove(); 

    circles.transition()
        .duration(this.transition_duration) 
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
        .attr('r', 0)
        .attr('fill', d => `rgba(${d.r}, ${d.g}, ${d.b}, ${d.a})`)
        .style('cursor', 'pointer')
        .style('stroke', 'rgba(255, 0, 0, 1')
        .on('mouseover', (event, d) => { 
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier);
        })
        .on('mouseout', (event, d) => { 
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
        })
        .on('click', (event, d) => { 
          this.datapoint_click.emit(d.identifier);
         })
        .transition()
        .duration(this.transition_duration)
        .attr('r', d => d.size);
    
    this.drawCircleLegend();
    this.drawColorLegend();
  }

  public resetDatapointEvents(){
    let circles = this.plot.selectAll('circle'); 
    circles
        .on('mouseover', (event, d) => { 
          this.tooltip.show(d, event.currentTarget);
          this.datapoint_hover_in.emit(d.identifier)
         })
        .on('mouseout', (event, d) => { 
          this.tooltip.hide(d, event.currentTarget); 
          this.datapoint_hover_out.emit(d.identifier);
         })
        .on('click', (event, d) => { 
          this.datapoint_click.emit(d.identifier);
         });
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

}