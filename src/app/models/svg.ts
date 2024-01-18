import * as d3 from 'd3';
import { ElementRef } from '@angular/core';
import { DataPoint } from './datapoint';

export class Svg {
    private d3_svg: any;
    private plot: any;

    private scaling_function: any;
    private hover_function: any;
    private positioning_ref: any;
    private tooltip_component: any;
    private datapoints: Array<DataPoint>;
    private scaled_datapoints: Array<DataPoint>;

    private width: number;
    private height: number;

    private x_scale: any;
    private y_scale: any;

    private gridlines: boolean;
    private number_of_gridlines: number;

    private pannable: boolean;

    constructor(vizualization_div: ElementRef<HTMLDivElement>, width: number, height: number, datapoints: Array<DataPoint>, 
                scaling_function: (arc: Array<any>) => Array<any>,
                hover_function: (mouse_position: [number, number], datapoint: DataPoint, tooltip_component: any, positioning_ref: any) => void,
                positioning_ref: any,
                tooltip_component: any,
                number_of_gridlines: number = 40, gridlines: boolean = true, pannable: boolean = true){

        this.datapoints = datapoints;
        this.scaling_function = scaling_function;
        this.hover_function = hover_function;
        this.positioning_ref = positioning_ref;
        this.tooltip_component = tooltip_component;
        this.width = width;
        this.height = height;
        this.number_of_gridlines = number_of_gridlines;
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

      let x_scale;

      if(this.pannable){ // Only the pannable visualization will have square aspect ratio
        x_scale = d3.scaleLinear()
        .domain([-1, 1])
        .range([0, (height - y_correction)/1]);

      }else if(!this.pannable){
        x_scale = d3.scaleLinear()
        .domain([-1, 1])
        .range([0, (width/1)]);
      }
  
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
      let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
      let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
  
      // Add the X gridlines
      this.plot.append("g")			
        .attr("class", "grid")
        .attr("transform", "translate(0," + this.height + ")")
        .attr("color", "lightgrey")
        .call(makeXGridlines()
            .tickSize(-this.height)
            .tickFormat(() => "")
        )
  
      // Add the Y gridlines
      this.plot.append("g")			
        .attr("class", "grid")
        .attr("color", "lightgrey")
        .call(makeYGridlines()
            .tickSize(-1 * this.width)
            // .tickSize(-300)
            .tickFormat(() => "")
        )
    }
  
    private createPlot(){
      if(this.plot != undefined){ this.d3_svg.select("#plot").remove(); }
      this.plot = this.d3_svg.append("g").attr("id", "plot");
      
      if(this.pannable){ // Only the pannable square visualization will execute this
        let panning_zoom = d3.zoom()
          .scaleExtent([1.6, 10]) // This control how much you can unzoom (x1) and zoom (x10)
          .translateExtent([[0, 0], [this.height, this.height/1.15]])
          .on("start", (event, d) => { this.d3_svg.attr("cursor", "grabbing"); })
          .on("zoom", (event) => { this.plot.attr("transform", event.transform); })
          .on("end", (event, d) => {this.d3_svg.attr("cursor", "default")});
    
        this.d3_svg.call(panning_zoom);

        // Apply initial zoom level
        let initial_scale = 1.6;
        let x_translation_factor = 0.05;
        let y_translation_factor = 0.3;
        let initial_transform = d3.zoomIdentity
          .translate(-this.width*(x_translation_factor), -this.height*(y_translation_factor))
          .scale(initial_scale);
        this.d3_svg.call(panning_zoom.transform, initial_transform);
      }
      
      if(this.gridlines){ this.drawGridLines(); }
      this.drawDataPoints();
    }
    
    private drawDataPoints() {
      if(this.plot == undefined){ return; }
    
      this.scaled_datapoints = this.scaling_function(this.datapoints);
    
      const circles = this.plot.selectAll('circle')
          .data(this.scaled_datapoints, d => d.identifier); // Each datapoint has a unique identifier
    
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
          .on('mouseover', (event, d) => { 
            if (this.hover_function != null){ this.hover_function(d3.pointer(event), d, this.tooltip_component, this.positioning_ref); }
           })
          .on('mouseout', (event, d) => { 
            if(this.hover_function != null){ this.hover_function(d3.pointer(event), null, this.tooltip_component, this.positioning_ref); }
           })
          .transition() // Transition to final state
          .duration(1000) // Duration of the animation in milliseconds
          .attr('r', d => d.size); // End with actual radius
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

    public getPlot(){
      return this.plot;
    }

    public getScaledDatapoints(){
      return this.scaled_datapoints;
    }

    public getXScale(){
      return this.x_scale;
    }

    public getYScale(){
      return this.y_scale;
    }
}