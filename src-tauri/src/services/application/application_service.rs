#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use crate::{services::{io_service::IoService, plot_service::PlotService}, model::analysis::metrics::metric::Metric};
use super::application_state_service::ApplicationStateService;

pub struct ApplicationService{
    io_service: IoService,
    application_state_service: ApplicationStateService,
}

impl ApplicationService{
    pub fn new(tensor_path: &String, patterns_path: &String) -> ApplicationService{
        return ApplicationService{
            io_service: IoService::new(tensor_path, patterns_path),
            application_state_service: ApplicationStateService::new(),
        };
    }

    pub fn init(&mut self){
        let start_time = Instant::now();

        let tensor = self.io_service.readTensor();
        let patterns = self.io_service.readPatterns();
        self.application_state_service.changeTensor(tensor, patterns);

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&self.application_state_service);

        // let pattern_1 = self.application_state_service.identifierMapper().getRepresentation(&1).asPattern();
        // let pattern_2 = self.application_state_service.identifierMapper().getRepresentation(&2).asPattern();
        // let pattern_3 = self.application_state_service.identifierMapper().getRepresentation(&3).asPattern();
        // let pattern_4 = self.application_state_service.identifierMapper().getRepresentation(&4).asPattern();

        // dbg!(pattern_1.intersection(pattern_2).len());
        // dbg!(pattern_1.intersection(pattern_3).len());
        // dbg!(pattern_1.intersection(pattern_4).len());
        
        // dbg!(pattern_2.intersection(pattern_3).len());
        // dbg!(pattern_2.intersection(pattern_4).len());

        // dbg!(pattern_3.intersection(pattern_4).len());
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

    pub fn descendDag(&mut self, nex_identifier: &u32){
        println!("\nDescending dag to: {}", nex_identifier);
        self.application_state_service.descendDag(nex_identifier);
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

    pub fn getRssEvolution(&self) -> &Vec<(u32, f64)>{
        return self.application_state_service.getMetricsService().rss_evolution.get();
    }

}