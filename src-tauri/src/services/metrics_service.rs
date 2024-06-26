#![allow(non_snake_case)]

use crate::model::analysis::metrics::intersection::intersection_metrics::IntersectionMetrics;
use crate::model::analysis::metrics::intersection::intersections_percentages::IntersectionsPercentages;
use crate::model::analysis::metrics::intersections_predictions::IntersectionsPredictions;
use crate::{model::{analysis::metrics::{empty_model_rss::EmptyModelRss, distances::Distances, coordinates::Coordinates, rss_evolution::RssEvolution}, identifier_mapper::IdentifierMapper}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct MetricsService{
    pub empty_model_rss: EmptyModelRss,
    pub rss_evolution: RssEvolution,
    pub all_initial_visible_distances: Distances,
    pub coordinates: Coordinates,
    pub intersections_percentages: IntersectionsPercentages,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, visible_identifiers: &Vec<u32>) -> Result<MetricsService, GenericError>{
        println!("Calculating metrics...");

        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper)?;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &identifier_mapper.getOrderedPatterns(),
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;

        let empty_model_rss = EmptyModelRss::new(tensor);
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedPatterns();

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &empty_model_rss,
            &patterns,
            &mut prediction_matrix,
            &untouched_delta_rss,
            &intersections_indices
        )?;

        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions,
            &visible_identifiers,
        )?;

        let coordinates = Coordinates::new(
            &distances,
        )?;

        println!("All metrics done!");
        return Ok(
            MetricsService {
                empty_model_rss: empty_model_rss,
                rss_evolution: rss_evolution,
                all_initial_visible_distances: distances,
                coordinates: coordinates,
                intersections_percentages: intersections_percentages,
             }
        );
    }

    pub fn update(&mut self, tensor: &Tensor, identifier_mapper: &IdentifierMapper, visible_identifiers: &Vec<u32>, lazy: &bool)
            -> Result<(), GenericError>{

        let visible_patterns = identifier_mapper.getOrderedPatternsFrom(visible_identifiers);
        
        let coordinates = Coordinates::new(
            &self.all_initial_visible_distances.getView(identifier_mapper, visible_identifiers)?,
        )?;
        self.coordinates = coordinates;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &visible_patterns,
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;
        self.intersections_percentages = intersections_percentages;

        if !lazy{ // Re-calculate rss_evolution
            let rss_evolution = RssEvolution::new(
                identifier_mapper,
                tensor,
                &self.empty_model_rss,
                &visible_patterns,
                &mut prediction_matrix,
                &untouched_delta_rss,
                &intersections_indices
            )?;

            self.rss_evolution = rss_evolution;
        
        }else if *lazy{ // Just truncate
            let new_size = visible_identifiers.len() as u32;
            self.rss_evolution.truncate(&new_size);
        }

        return Ok(());
    }
}