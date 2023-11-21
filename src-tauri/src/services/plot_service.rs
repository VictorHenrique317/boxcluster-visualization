#![allow(non_snake_case)]
use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, Circle}, style::{WHITE, Color, IntoFont, RGBAColor, RGBColor, TextStyle}};

use crate::{model::{identifier_representation::IdentifierRepresentation}};

use super::application::application_state_service::ApplicationStateService;

pub struct PlotService{}

impl PlotService{
    pub fn plot(application_state: &ApplicationStateService){
        // let root = BitMapBackend::new("scatter.png", (1600, 900)).into_drawing_area();
        // root.fill(&WHITE).unwrap();

        // let identifier_mapper = application_state.identifierMapper();

        // let visible_identifiers = application_state.getVisibleIdentifiers();
        // let visible_representations: Vec<&IdentifierRepresentation> = identifier_mapper.getMapping()
        //     .iter()
        //     .filter(|(identifier, _)| visible_identifiers.contains(identifier))
        //     .map(|(_, representation)| representation)
        //     .collect();
    
        // let mut x_range = 0.0;
        // let mut y_range = 0.0;
        // for identifier_representation in visible_representations.iter(){
        //     let datapoint = identifier_representation.asDataPoint();
            
        //     let positive_x_range = x_range.clone();
        //     let negative_x_range = x_range.clone() * -1.0;
    
        //     let positive_y_range = y_range.clone();
        //     let negative_y_range = y_range.clone() * -1.0;
    
        //     if datapoint.x > positive_x_range{ x_range = datapoint.x.clone().abs(); }
        //     else if datapoint.x < negative_x_range{ x_range = datapoint.x.clone().abs(); }
    
        //     if datapoint.y > positive_y_range{ y_range = datapoint.y.clone().abs(); }
        //     else if datapoint.y < negative_y_range{ y_range = datapoint.y.clone().abs(); }
        // }
    
        // x_range *= 1.1;
        // y_range *= 1.1;
    
        // let mut chart = ChartBuilder::on(&root)
        //     .caption("Scatter Plot", ("sans-serif", 50).into_font())
        //     .margin(5)
        //     .x_label_area_size(30)
        //     .y_label_area_size(30)
        //     .build_cartesian_2d(-1.0 * x_range..x_range, -1.0 * y_range..y_range).unwrap();
    
        // chart.configure_mesh().draw().unwrap();
    
        // // Enforcing that overlapping points are drawn in the correct order
        // let mut representations = visible_representations;
        // representations.sort_by(|a, b| 
        //     b.asDataPoint().size.partial_cmp(&a.asDataPoint().size).unwrap()); 
            
        // for identifier_representation in representations{
        //     let datapoint = identifier_representation.asDataPoint();
        //     let mut color = RGBColor(datapoint.r as u8, datapoint.g as u8, datapoint.b as u8).filled();
            

        //     // 2 e 7
        //     // if datapoint.identifier == 2 {color = RGBColor(0, 255, 0).filled();}
        //     // if datapoint.identifier == 7 {color = RGBColor(0, 0, 255).filled();}

        //     // let pattern = identifier_representation.asPattern();
        //     // dbg!(datapoint.color);
        //     // dbg!(pattern.identifier);
            
        //     chart.draw_series(
        //         std::iter::once(Circle::new((
        //             datapoint.x, datapoint.y), 
        //             2 * datapoint.size as i32,
        //             color.filled()
        //             // ShapeStyle {
        //             //     color: datapoint.color.to_rgba(),
        //             //     filled: false,
        //             //     stroke_width: datapoint.stroke_width,
        //             // }
        //         ))
        //     ).unwrap();
        // }
    
        // println!("PLOTTED TEST GRAPH");
    }
}