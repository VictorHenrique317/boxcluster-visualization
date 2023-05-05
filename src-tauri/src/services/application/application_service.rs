#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use crate::services::{io_service::IoService, plot_service::PlotService};
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

        let patterns = self.io_service.readPatterns();
        let tensor = self.io_service.readTensor();
        self.application_state_service.changeTensor(tensor, patterns);

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(self.application_state_service.identifierMapper());
    }

    pub fn changeTensor(&mut self, tensor_path: &String, patterns_path: &String){
        self.io_service = IoService::new(tensor_path, patterns_path);
        let tensor = self.io_service.readTensor();
        let patterns = self.io_service.readPatterns();

        self.application_state_service.changeTensor(tensor, patterns);
    }

    pub fn changePatterns(&mut self, patterns_path: &String){
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns();

        self.application_state_service.changePatterns(patterns);
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        // return self.dag.getFlattenedSupers().clone();
        todo!()
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        // return self.dag.getFlattenedSubs().clone();
        todo!()
    }

    // pub fn getDistances(&self) -> HashMap<u32, HashMap<u32, f64>>{
    //     return self.metrics_service.distances.get().clone();
    // }


}