#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use crate::{services::{io_service::IoService, plot_service::PlotService}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}};
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
    pub fn new(tensor_path: &String, patterns_path: &String) -> ApplicationService{
        let mut instance = ApplicationService{
            io_service: IoService::new(tensor_path, patterns_path),
            application_state_service: ApplicationStateService::new(),
        };

        ApplicationService::init(&mut instance);
        return instance;
    }

    fn init(instance: &mut ApplicationService){
        let start_time = Instant::now();

        let tensor = instance.io_service.readTensor();
        let patterns = instance.io_service.readPatterns();
        instance.application_state_service.changeTensor(tensor, patterns);

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&instance.application_state_service);
    }

    pub fn changeTensor(&mut self, tensor_path: &String, patterns_path: &String){
        println!("\nChanging tensor to: {}", tensor_path);
        self.io_service = IoService::new(tensor_path, patterns_path);
        let tensor = self.io_service.readTensor();
        let patterns = self.io_service.readPatterns();

        self.application_state_service.changeTensor(tensor, patterns);
        PlotService::plot(&self.application_state_service);
    }

    pub fn changePatterns(&mut self, patterns_path: &String){
        println!("\nChanging patterns to: {}", patterns_path);
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns();

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

    pub fn getIdentifierMapper(&self) -> &IdentifierMapper {
        return self.application_state_service.identifierMapper();
    }

}