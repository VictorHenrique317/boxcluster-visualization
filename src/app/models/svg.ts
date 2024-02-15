// import * as d3 from 'd3';
// import { ElementRef } from '@angular/core';
// import { DataPoint } from './datapoint';
// import { event } from '@tauri-apps/api';

// export class Svg {
//     public d3_svg: any;
//     public plot: any;

//     private width: number;
//     private height: number;

//     private x_scale: any;
//     private y_scale: any;

//     private gridlines: boolean;
//     private number_of_gridlines: number;

//     private pannable: boolean;

//     private initial_scale: number;

//     constructor(vizualization_div: ElementRef<HTMLDivElement>, width: number, height: number, 
//                 number_of_gridlines: number = 40, gridlines: boolean = true, pannable: boolean = true){

//         this.width = width;
//         this.height = height;
//         this.number_of_gridlines = number_of_gridlines;
//         this.gridlines = gridlines;
//         this.pannable = pannable;
//         this.create(vizualization_div);
//     }

//     private create(vizualization_div: ElementRef<HTMLDivElement>){
//       this.d3_svg = d3.select(vizualization_div.nativeElement)
//       .append('svg')
//         .attr('width', this.width)
//         .attr('height',this.height);
//     }

//     public resize(width: number, height: number, y_correction=0){
//       this.d3_svg
//         .attr('width', width)
//         .attr('height', height);

//       let x_scale;

//       if(this.pannable){ // Only the pannable visualization will have square aspect ratio
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (height - y_correction)/1]);

//       }else if(!this.pannable){
//         x_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([0, (width/1)]);
//       }
  
//       let y_scale = d3.scaleLinear()
//         .domain([-1, 1])
//         .range([(height - y_correction)/1, 0]);

//       this.x_scale = x_scale;
//       this.y_scale = y_scale;
//       this.width = width;
//       this.height = height;
  
//       this.createPlot();
//     }

//     private drawGridLines() {
//       let makeXGridlines = () => { return d3.axisBottom(this.x_scale).ticks(this.number_of_gridlines) }
//       let makeYGridlines = () => { return d3.axisLeft(this.y_scale).ticks(this.number_of_gridlines) }
  
//       // Add the X gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("transform", "translate(0," + this.height + ")")
//         .attr("color", "lightgrey")
//         .call(makeXGridlines()
//             .tickSize(-this.height)
//             .tickFormat(() => "")
//         )
  
//       // Add the Y gridlines
//       this.plot.append("g")			
//         .attr("class", "grid")
//         .attr("color", "lightgrey")
//         .call(makeYGridlines()
//             .tickSize(-1 * this.width)
//             // .tickSize(-300)
//             .tickFormat(() => "")
//         )
//     }
    
//     private createPlot(){
//       if(this.plot != undefined){ this.d3_svg.select("#plot").remove(); }
//       this.plot = this.d3_svg.append("g").attr("id", "plot");
      
//       if(this.pannable){ // Only the pannable square visualization will execute this
//         let panning_zoom = d3.zoom()
//           .scaleExtent([1.4, 10]) // This control how much you can unzoom (x1) and zoom (x10)
//           // .translateExtent([[0, 0], [this.height, this.height/1.2]])
//           .translateExtent([[0, 0], [this.height, this.height]])
//           .on("start", (event, d) => { this.d3_svg.attr("cursor", "grabbing"); })
//           .on("zoom", (event) => { this.plot.attr("transform", event.transform); })
//           .on("end", (event, d) => {this.d3_svg.attr("cursor", "default")});
    
//         this.d3_svg.call(panning_zoom);

//         // Apply initial zoom level
//         this.initial_scale=  1.4;
//         let x_translation_factor = 0.0;
//         // let y_translation_factor = 0.15;
//         let y_translation_factor = 0.2;
//         let initial_transform = d3.zoomIdentity
//           .translate(-this.width*(x_translation_factor), -this.height*(y_translation_factor))
//           // .translate(-this.width*(x_translation_factor), 0)
//           .scale(this.initial_scale);
//         this.d3_svg.call(panning_zoom.transform, initial_transform);
//       }
      
//       if(this.gridlines){ this.drawGridLines(); }
//       // this.drawDataPoints();
//     }

//     public drawVerticalLine(x: number) {
//       // Remove any existing line
//       this.plot.selectAll('#vertical-line').remove();
  
//       // Draw a new line
//       this.plot.append('line')
//           .attr('id', 'vertical-line')
//           .attr('x1', this.x_scale(x))
//           .attr('y1', 0)
//           .attr('x2', this.x_scale(x))
//           .attr('y2', this.height)
//           .attr('stroke', 'red')
//           .attr('stroke-width', 2);
//   }

//   public getXScale(){
//     return this.x_scale;
//   }

//   public getYScale(){
//     return this.y_scale;
//   }

//   public getInitialScale(){
//     return this.initial_scale;
//   }
// }