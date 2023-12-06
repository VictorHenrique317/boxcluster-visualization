import * as d3 from 'd3';
import { ElementRef } from '@angular/core';
import { DataPoint } from './datapoint';

export class Svg {
    private d3_svg: any;
    private plot: any;

    private scaling_function: any;
    private datapoints: Array<DataPoint>;

    private width: number;
    private height: number;

    private x_scale: any;
    private y_scale: any;

    private gridlines: boolean;
    private pannable: boolean;

    constructor(vizualization_div: ElementRef<HTMLDivElement>, width: number, height: number, 
                datapoints: Array<DataPoint>, scaling_function: (arc: Array<any>) => Array<any>, 
                gridlines: boolean = true, pannable: boolean = true){
        this.datapoints = datapoints;
        this.scaling_function = scaling_function;
        this.width = width;
        this.height = height;
        this.gridlines = gridlines;
        this.pannable = pannable;
        this.create(vizualization_div);
    }

    public setDatapoints(datapoints: Array<DataPoint>){
      this.datapoints = datapoints;
      this.drawDataPoints();
    }

    private create(vizualization_div: ElementRef<HTMLDivElement>){
      this.d3_svg = d3.select(vizualization_div.nativeElement)
      .append('svg')
        .attr('width', this.width)
        .attr('height',this.height);
    }

    public resize(width: number, height: number, y_correction=0){
      this.d3_svg
        .attr('width', width)
        .attr('height', height);
  
      let x_scale = d3.scaleLinear()
        .domain([-1, 1])
        .range([0, (width/1)]);
  
      let y_scale = d3.scaleLinear()
        .domain([-1, 1])
        .range([(height - y_correction)/1, 0]);

      this.x_scale = x_scale;
      this.y_scale = y_scale;
      this.width = width;
      this.height = height;
  
      this.createPlot();
    }

    private drawGridLines() {
      let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(40) }
      let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(40) }
  
      // Add the X gridlines
      this.plot.append("g")			
        .attr("class", "grid")
        .attr("transform", "translate(0," + this.height + ")")
        .attr("color", "grey")
        .call(makeXGridlines()
            .tickSize(-this.height)
            .tickFormat(() => "")
        )
  
      // Add the Y gridlines
      this.plot.append("g")			
        .attr("class", "grid")
        .attr("color", "grey")
        .call(makeYGridlines()
            .tickSize(-1 * this.width)
            .tickFormat(() => "")
        )
    }
  
    private createPlot(){
      if(this.plot != undefined){ this.d3_svg.select("#plot").remove(); }
      this.plot = this.d3_svg.append("g").attr("id", "plot");
      
      if(this.pannable){
        let panning_zoom = d3.zoom()
        .scaleExtent([1, 10]) // This control how much you can unzoom (x1) and zoom (x10)
        .translateExtent([[0, 0], [this.width, this.height]])
        .on("start", (event, d) => { this.d3_svg.attr("cursor", "grabbing"); })
        .on("zoom", (event) => { this.plot.attr("transform", event.transform); })
        .on("end", (event, d) => {this.d3_svg.attr("cursor", "default")});
    
        this.d3_svg.call(panning_zoom);

        // Apply initial zoom level
        let initial_scale = 1.2;
        let translation_factor = 0.1;
        let initial_transform = d3.zoomIdentity
          .translate(-this.width*(translation_factor), -this.height*(translation_factor))
          .scale(initial_scale);
        this.d3_svg.call(panning_zoom.transform, initial_transform);
      }
      
      if(this.gridlines){ this.drawGridLines(); }
      this.drawDataPoints();
    }
    
    public drawDataPoints() {
      if(this.plot == undefined){ return; }

      this.plot.selectAll('circle').remove();
      
      let scaled_datapoints = this.scaling_function(this.datapoints);
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

    public drawVerticalLine(x: number) {
      // Remove any existing line
      this.plot.selectAll('#vertical-line').remove();
  
      // Draw a new line
      this.plot.append('line')
          .attr('id', 'vertical-line')
          .attr('x1', this.x_scale(x))
          .attr('y1', 0)
          .attr('x2', this.x_scale(x))
          .attr('y2', this.height)
          .attr('stroke', 'red')
          .attr('stroke-width', 2);
  }

    public getD3Svg(){
      return this.d3_svg;
    }
}