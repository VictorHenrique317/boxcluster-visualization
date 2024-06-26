import { ElementRef, Injectable } from '@angular/core';
import { Color } from 'src/app/models/color';
import * as d3 from 'd3';

@Injectable({
  providedIn: 'root'
})
export class SvgService {

  constructor() { }

  



  // private drawGridLines() {
  //   let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(40) }
  //   let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(40) }

  //   // Add the X gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("transform", "translate(0," + this.height + ")")
  //     .attr("color", "grey")
  //     .call(makeXGridlines()
  //         .tickSize(-this.height)
  //         .tickFormat(() => "")
  //     )

  //   // Add the Y gridlines
  //   this.plot.append("g")			
  //     .attr("class", "grid")
  //     .attr("color", "grey")
  //     .call(makeYGridlines()
  //         .tickSize(-1 * this.width)
  //         .tickFormat(() => "")
  //     )
  // }

  // private createPlot(svg: any, width: number, height: number){
  //   svg.select("#plot").remove();
  //   let plot = svg.append("g").attr("id", "plot");
  
  //   let panning_zoom = d3.zoom()
  //     .scaleExtent([1, 10]) // This control how much you can unzoom (x1) and zoom (x10)
  //     .translateExtent([[0, 0], [width, height]])
  //     .on("start", (event, d) => { svg.attr("cursor", "grabbing"); })
  //     .on("zoom", (event) => { plot.attr("transform", event.transform); })
  //     .on("end", (event, d) => {svg.attr("cursor", "default")});
  
  //   svg.call(panning_zoom);
  
  //   // Apply initial zoom level
  //   let initial_scale = 1.2;
  //   let translation_factor = 0.1;
  //   let initial_transform = d3.zoomIdentity
  //     .translate(-width*(translation_factor), -height*(translation_factor))
  //     .scale(initial_scale);
  //   svg.call(panning_zoom.transform, initial_transform);
  
  //   this.drawGridLines();
  //   this.drawDataPoints();
  // }


}
