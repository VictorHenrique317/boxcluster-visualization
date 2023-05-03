#![allow(non_snake_case)]
use std::collections::{HashMap, HashSet};
use crate::{model::common::{identifier_mapper::IdentifierMapper, intersections_predictions::IntersectionsPredictions}, database::tensor::Tensor};

use super::{empty_model_rss::EmptyModelRss, distances::Distances};

#[derive(Default)]
pub struct Metrics{
    pub empty_model_rss: EmptyModelRss,
    pub distances: Distances,
}

impl Metrics{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor) -> Metrics{
        println!("Calculating metrics...");
        let empty_model_rss = EmptyModelRss::new(tensor);
        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper);
        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions
        );

        println!("\nAll metrics done!");
        return Metrics { 
            empty_model_rss: empty_model_rss,
            distances: distances,
         };
    }
}