#![allow(non_snake_case)]
use std::collections::{HashMap, HashSet};

use crate::{model::{analysis::{metrics::{empty_model_rss::EmptyModelRss, distances::Distances}, intersections_predictions::IntersectionsPredictions}, identifier_mapper::IdentifierMapper}, database::tensor::Tensor};


#[derive(Default)]
pub struct MetricsService{
    mds_service: MDSService,

    pub empty_model_rss: EmptyModelRss,
    pub distances: Distances,
    pub coords: Coordinate,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor) -> MetricsService{
        println!("Calculating metrics...");
        let empty_model_rss = EmptyModelRss::new(tensor);
        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper);
        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions
        );

        println!("\nAll metrics done!");
        return MetricsService { 
            empty_model_rss: empty_model_rss,
            distances: distances,
         };
    }
}