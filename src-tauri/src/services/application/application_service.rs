#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use crate::{services::{io_service::IoService, plot_service::PlotService}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}, database::datapoint::DataPoint};
use super::application_state_service::ApplicationStateService;

pub struct ApplicationService{
    io_service: IoService,
    application_state_service: ApplicationStateService,
}

impl Default for ApplicationService{
    fn default() -> Self {
        return ApplicationService{
            io_service: IoService::default(),
            application_state_service: ApplicationStateService::default(),
        };
    }
}

impl ApplicationService{
    pub fn init(&mut self, tensor_path: &String, patterns_path: &String){
        let start_time = Instant::now();
        println!("Initializing model...");

        self.io_service = IoService::new(tensor_path, patterns_path);
        let tensor = self.io_service.readTensor();
        let patterns = self.io_service.readPatterns().unwrap();

        self.application_state_service = ApplicationStateService::default();
        self.application_state_service.init(tensor, patterns);

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&self.application_state_service);
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        let identifier_mapper = self.application_state_service.identifierMapper();
        return self.application_state_service.getDagService().getFlattenedSupers(identifier_mapper);
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        let identifier_mapper = self.application_state_service.identifierMapper();
        return self.application_state_service.getDagService().getFlattenedSubs(identifier_mapper);
    }

    pub fn getDistances(&self) -> &HashMap<u32, HashMap<u32, f64>>{
        return self.application_state_service.getMetricsService().distances.get();
    }

    pub fn getIdentifierMapper(&self) -> &IdentifierMapper {
        return self.application_state_service.identifierMapper();
    }

    // ================ External API ================

    pub fn changePatterns(&mut self, patterns_path: &String){
        println!("\nChanging patterns to: {}", patterns_path);
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns().unwrap();

        self.application_state_service.changePatterns(patterns);
        PlotService::plot(&self.application_state_service);
    }

    pub fn ascendDag(&mut self){
        println!("\nAscending dag");
        self.application_state_service.ascendDag();
        PlotService::plot(&self.application_state_service);
    }

    pub fn descendDag(&mut self, next_identifier: &u32){
        println!("\nDescending dag to: {}", next_identifier);
        self.application_state_service.descendDag(next_identifier);
        PlotService::plot(&self.application_state_service);
    }

    pub fn truncateModel(&mut self, new_size: &u32){
        println!("\nTruncating model to {} patterns", new_size);
        self.application_state_service.truncateModel(&new_size);
        PlotService::plot(&self.application_state_service);
    }

    pub fn getDataPoints(&self) -> Vec<DataPoint>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();
        return self.application_state_service.identifierMapper()
            .getOrderedDataPointsFrom(visible_identifiers).into_iter()
            .map(|datapoint| datapoint.clone())
            .collect();
    }

    pub fn getFullRssEvolution(&self) -> Vec<f64>{
        return self.application_state_service.getMetricsService().rss_evolution.get().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect();
    }

    pub fn getTruncatedRssEvolution(&self) -> Vec<f64>{
        return self.application_state_service.getMetricsService().rss_evolution.getTruncated().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect();
    }

}