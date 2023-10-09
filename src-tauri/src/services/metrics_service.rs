#![allow(non_snake_case)]


use crate::{model::{analysis::{metrics::{empty_model_rss::EmptyModelRss, distances::Distances, coordinates::Coordinates, rss_evolution::RssEvolution}, intersections_predictions::IntersectionsPredictions}, identifier_mapper::IdentifierMapper}, database::{tensor::Tensor, pattern::Pattern}};

pub struct MetricsService{
    pub empty_model_rss: EmptyModelRss,
    pub rss_evolution: RssEvolution,
    pub distances: Distances,
    pub coordinates: Coordinates,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor) -> MetricsService{
        println!("Calculating metrics...");

        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper);
        let empty_model_rss = EmptyModelRss::new(tensor);
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedPatterns();

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &empty_model_rss,
            &patterns
        );

        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions
        );

        let coordinates = Coordinates::new(
            identifier_mapper,
            &distances,
        );

        println!("All metrics done!");
        return MetricsService {
            empty_model_rss: empty_model_rss,
            rss_evolution: rss_evolution,
            distances: distances,
            coordinates: coordinates,
         };
    }

    pub fn update(&mut self, tensor: &Tensor, identifier_mapper: &IdentifierMapper, visible_identifiers: &Vec<u32>) {
        let coordinates = Coordinates::new(
            identifier_mapper,
            &self.distances.getView(identifier_mapper, visible_identifiers),
        );

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &self.empty_model_rss,
            &identifier_mapper.getOrderedPatternsFrom(visible_identifiers)
        );

        self.coordinates = coordinates;
        self.rss_evolution = rss_evolution;
    }
}