#![allow(non_snake_case)]

use crate::model::analysis::metrics::metric::Metric;
use crate::{model::{analysis::{metrics::{empty_model_rss::EmptyModelRss, distances::Distances, coordinates::Coordinates, rss_evolution::RssEvolution}, intersections_predictions::IntersectionsPredictions}, identifier_mapper::IdentifierMapper}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct MetricsService{
    pub empty_model_rss: EmptyModelRss,
    pub rss_evolution: RssEvolution,
    pub distances: Distances,
    pub coordinates: Coordinates,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor) -> Result<MetricsService, GenericError>{
        println!("Calculating metrics...");

        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper)?;
        let empty_model_rss = EmptyModelRss::new(tensor);
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedPatterns();

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &empty_model_rss,
            &patterns
        )?;

        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions
        )?;

        let coordinates = Coordinates::new(
            identifier_mapper,
            &distances,
        )?;

        println!("All metrics done!");
        return Ok(
            MetricsService {
                empty_model_rss: empty_model_rss,
                rss_evolution: rss_evolution,
                distances: distances,
                coordinates: coordinates,
             }
        );
    }

    pub fn update(&mut self, tensor: &Tensor, identifier_mapper: &IdentifierMapper, visible_identifiers: &Vec<u32>, lazy: &bool)
            -> Result<(), GenericError>{
        
        let coordinates = Coordinates::new(
            identifier_mapper,
            &self.distances.getView(identifier_mapper, visible_identifiers)?,
        )?;

        self.coordinates = coordinates;

        if !lazy{ // Re-calculate rss_evolution
            let rss_evolution = RssEvolution::new(
                identifier_mapper,
                tensor,
                &self.empty_model_rss,
                &identifier_mapper.getOrderedPatternsFrom(visible_identifiers)
            )?;

            self.rss_evolution = rss_evolution;
        
        }else if *lazy{ // Just truncate
            let new_size = visible_identifiers.len() as u32;
            self.rss_evolution.truncate(&new_size);
        }

        return Ok(());
    }
}