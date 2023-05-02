#![allow(non_snake_case)]
use std::collections::HashSet;
use std::{collections::HashMap, time::Instant};
use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, EmptyElement, Circle};
use plotters::series::PointSeries;
use plotters::style::{WHITE, RED, IntoFont, ShapeStyle, Color};

use crate::common::identifier_mapper::IdentifierMapper;
use crate::metrics::datapoint::DataPoint;
use crate::post_analysis::datapoints_creator::DataPointsCreator;
use crate::post_analysis::multidim_scaling::MultiDimScaling;
use crate::{io::{tensor_reader::TensorReader, translator::Translator, pattern_reader::PatternReader}, dag::{dag_creator::DagCreator}, metrics::{metrics::Metrics, metric::Metric}, tensor::tensor::Tensor};

pub struct ApplicationManager{
    tensor_path: String,
    patterns_path: String,
    translator: Translator,

    tensor: Tensor,
    identifier_mapper: IdentifierMapper,

    metrics: Metrics,
}

impl ApplicationManager{
    pub fn new(tensor_path: &String, patterns_path: &String) -> ApplicationManager{
        return ApplicationManager{
            tensor_path: tensor_path.to_owned(),
            patterns_path: patterns_path.to_owned(),
            translator: Default::default(),
            tensor: Default::default(),
            identifier_mapper: Default::default(),
            metrics: Default::default(),
        };
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

        for identifier_representation in self.identifier_mapper.getRepresentations(){
            let datapoint = identifier_representation.asDataPoint();
            // dbg!(&datapoint.stroke_width);
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

    fn acquireInformation(&mut self){
        self.translator = Translator::new(&self.tensor_path);
        let tensor_reader = TensorReader::new(
            &self.tensor_path,
            &self.translator);
        self.tensor = tensor_reader.read();

        let pattern_reader = PatternReader::new(
            &self.patterns_path,
            &self.translator);
        let patterns = pattern_reader.read();

        self.identifier_mapper = IdentifierMapper::new(patterns);
    }

    fn createModel(&mut self){
        let dag_creator = DagCreator::new(&self.identifier_mapper);
        let dag = dag_creator.create();
        self.identifier_mapper.insertDagNodeRepresentations(
            dag.extractNodes()
        );
    }

    fn analyseModel(&mut self){
        self.metrics = Metrics::new(&self.identifier_mapper, &self.tensor);
        
        let coords = MultiDimScaling::fitTransform(&self.metrics.distances, &self.identifier_mapper);

        let data_point_representations = DataPointsCreator::create(&self.identifier_mapper, &coords);
        self.identifier_mapper.insertDataPointRepresentations(data_point_representations);
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

    // pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
    //     return self.dag.getFlattenedSupers().clone();
    // }

    // pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
    //     return self.dag.getFlattenedSubs().clone();
    // }

    pub fn getDistances(&self) -> HashMap<u32, HashMap<u32, f64>>{
        return self.metrics.distances.get().clone();
    }


}