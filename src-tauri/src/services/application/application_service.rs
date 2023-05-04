#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, Circle};
use plotters::style::{WHITE, IntoFont};
use crate::model::identifier_mapper::IdentifierMapper;
use crate::database::datapoint::DataPoint;
use crate::services::io_service::IoService;
use super::application_state_service::ApplicationStateService;

pub struct ApplicationService<'a>{
    io_service: IoService,
    application_state_service: ApplicationStateService<'a>,
}

impl ApplicationService<'_>{
    pub fn new<'a>(tensor_path: &String, patterns_path: &String) -> ApplicationService<'a>{
        return ApplicationService{
            io_service: IoService::new(tensor_path, patterns_path),
            application_state_service: ApplicationStateService::new(),
        };
    }

    fn acquireInformation(&mut self){
        let patterns = self.io_service.readPatterns();
        self.identifier_mapper = IdentifierMapper::new(patterns);
        self.tensor = self.io_service.readTensor();
    }

    fn createModel(&mut self){
        let dag_creator = DagCreator::new(&self.identifier_mapper);
        let dag = dag_creator.create();
        self.identifier_mapper.insertDagNodeRepresentations(
            dag.extractNodes()
        );
    }

    fn analyseModel(&mut self){
        self.metrics_service = MetricsService::new(&self.identifier_mapper, &self.tensor);
        
        let coords = MDSService::fitTransform(&self.metrics_service.distances, &self.identifier_mapper);

        let data_point_representations = DataPoint::createDataPoints(&self.identifier_mapper, &coords);
        self.identifier_mapper.insertDataPointRepresentations(data_point_representations);
    }

    fn testPlot(&self){
        let root = BitMapBackend::new("scatter.png", (1600, 900)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut x_range = 0.0;
        let mut y_range = 0.0;
        for identifier_representation in self.identifier_mapper.getRepresentations(){
            let datapoint = identifier_representation.asDataPoint();
            
            let positive_x_range = x_range.clone();
            let negative_x_range = x_range.clone() * -1.0;

            let positive_y_range = y_range.clone();
            let negative_y_range = y_range.clone() * -1.0;

            if datapoint.x > positive_x_range{ x_range = datapoint.x.clone().abs(); }
            else if datapoint.x < negative_x_range{ x_range = datapoint.x.clone().abs(); }

            if datapoint.y > positive_y_range{ y_range = datapoint.y.clone().abs(); }
            else if datapoint.y < negative_y_range{ y_range = datapoint.y.clone().abs(); }
        }

        x_range *= 1.1;
        y_range *= 1.1;

        let mut chart = ChartBuilder::on(&root)
            .caption("Scatter Plot", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1.0 * x_range..x_range, -1.0 * y_range..y_range).unwrap();

        chart.configure_mesh().draw().unwrap();

        // Enforcing that overlapping points are drawn in the correct order
        let mut representations = self.identifier_mapper.getRepresentations();
        representations.sort_by(|a, b| 
            b.asDataPoint().size.partial_cmp(&a.asDataPoint().size).unwrap()); 
            
        for identifier_representation in representations{
            let datapoint = identifier_representation.asDataPoint();
            chart.draw_series(
                std::iter::once(Circle::new((
                    datapoint.x, 
                    datapoint.y), 
                    datapoint.size as i32,
                    datapoint.color.filled()
                    // ShapeStyle {
                    //     color: datapoint.color.to_rgba(),
                    //     filled: false,
                    //     stroke_width: datapoint.stroke_width,
                    // }
                ))
            ).unwrap();
        }

        println!("PLOTTED TEST GRAPH");
    }

    pub fn initialize(&mut self){
        let start_time = Instant::now();
        
        self.acquireInformation();
        self.createModel();
        self.analyseModel();
        self.testPlot();
        
        let end_time = Instant::now();
        let duration = end_time - start_time;
    
        println!("Total time taken: {:?}", duration);
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        // return self.dag.getFlattenedSupers().clone();
        todo!()
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        // return self.dag.getFlattenedSubs().clone();
        todo!()
    }

    pub fn getDistances(&self) -> HashMap<u32, HashMap<u32, f64>>{
        return self.metrics_service.distances.get().clone();
    }


}