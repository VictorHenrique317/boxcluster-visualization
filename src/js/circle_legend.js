// https://observablehq.com/@harrystevens/circle-legend

export const legendCircle = function(context){
    let scale,
        tickValues,
        tickFormat = d => d,
        tickSize = 5;
    
    function legend(context){
      let g = context.select("g");
      if (!g._groups[0][0]){
        g = context.append("g");
      }
      g.attr("transform", `translate(${[1, 1]})`);
      
      const ticks = tickValues || scale.ticks();
      
      const max = ticks[ticks.length - 1];
      
      g.selectAll("circle")
          .data(ticks.slice().reverse())
        .enter().append("circle")
          .attr("fill", "none")
          .attr("stroke", "currentColor")
          .attr("cx", scale(max))
          .attr("cy", scale)
          .attr("r", scale);
      
      g.selectAll("line")
          .data(ticks)
        .enter().append("line")
          .attr("stroke", "currentColor")
          .attr("stroke-dasharray", "4, 2")
          .attr("x1", scale(max))
          .attr("x2", tickSize + scale(max) * 2)
          .attr("y1", d => scale(d) * 2)
          .attr("y2", d => scale(d) * 2);
      
      g.selectAll("text")
          .data(ticks)
        .enter().append("text")
          .attr("font-family", "'Helvetica Neue', sans-serif")
          .attr("font-size", 11)
          .attr("dx", 3)
          .attr("dy", 4)
          .attr("x", tickSize + scale(max) * 2)
          .attr("y", d => scale(d) * 2)
          .text(tickFormat);
    }
    
    legend.tickSize = function(_){
      return arguments.length ? (tickSize = +_, legend) : tickSize;
    }
    
    legend.scale = function(_){
      return arguments.length ? (scale = _, legend) : scale;
    }
  
    legend.tickFormat = function(_){
      return arguments.length ? (tickFormat = _, legend) : tickFormat;
    }
    
    legend.tickValues = function(_){
      return arguments.length ? (tickValues = _, legend) : tickValues;
    }
    
    return legend;
  }