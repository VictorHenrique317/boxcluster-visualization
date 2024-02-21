#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use plotters::data;

use crate::{services::{io_service::IoService, plot_service::PlotService}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper, io::translator::Translator}, database::{datapoint::DataPoint, pattern::Pattern, raw_pattern::RawPattern}, common::generic_error::GenericError};
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
    pub fn init(&mut self, tensor_path: &String, patterns_path: &String) -> Result<(), GenericError>{
        let start_time = Instant::now();
        println!("Initializing model...");

        self.io_service = IoService::new(tensor_path, patterns_path)?;
        let tensor = self.io_service.readTensor()?;
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service = ApplicationStateService::default();
        self.application_state_service.init(tensor, patterns)?;

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&self.application_state_service);
        return Ok(());
    }

    pub fn getFlattenedSupers(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSupers(identifier_mapper)?
        );
    }

    pub fn getFlattenedSubs(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSubs(identifier_mapper)?
        );
    }

    pub fn getDistances(&self) -> Result<&HashMap<u32, HashMap<u32, f64>>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.distances.get()
        );
    }

    pub fn getIntersectionsPercentages(&self) -> Result<HashMap<u32, HashMap<u32, f64>>, GenericError> {
        return Ok(
            self.application_state_service.getMetricsService()?.intersections_percentages.get().clone()
        );
    }

    pub fn getIdentifierMapper(&self) -> Result<&IdentifierMapper, GenericError> {
        return self.application_state_service.identifierMapper();
    }

    pub fn getTranslator(&self) -> &Translator {
        return self.io_service.getTranslator();
    }

    // ================ External API ================

    pub fn changePatterns(&mut self, patterns_path: &String) -> Result<(), GenericError>{
        println!("\nChanging patterns to: {}", patterns_path);
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service.changePatterns(patterns)?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<(), GenericError>{
        println!("\nAscending dag");
        self.application_state_service.ascendDag()?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<(), GenericError> {
        println!("\nDescending dag to: {}", next_identifier);
        self.application_state_service.descendDag(next_identifier)?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<Vec<(f32, f32)>, GenericError> {
        println!("\nTruncating model to {} patterns", new_size);
        self.application_state_service.truncateModel(&new_size)?;
        PlotService::plot(&self.application_state_service);

        let mut datapoints = self.getDataPoints()?;
        datapoints.truncate(*new_size as usize);

        let datapoints_changes: Vec<(f32, f32)> = datapoints.into_iter()
            .map(|datapoint| (datapoint.x, datapoint.y))
            .collect();

        return Ok(datapoints_changes);
    }

    pub fn getDataPoints(&self) -> Result<Vec<DataPoint>, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();
        // let identifiers = self.application_state_service.getAllIdentifiers()?;
        let datapoints: Vec<DataPoint> = self.application_state_service.identifierMapper()?
            .getOrderedDataPointsFrom(visible_identifiers).into_iter()
            .map(|datapoint| datapoint.clone())
            .collect();

        return Ok(datapoints);
    }

    pub fn getRawPattern(&self, identifier: &u32) -> Result<RawPattern, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();

        if !visible_identifiers.contains(identifier){
            return Err(GenericError::new("Identifier not visible", file!(), &line!()));
        }

        return self.getIdentifierMapper()?.getIdentifier(identifier)?
            .asRawPattern(self.io_service.getTranslator());
    }

    pub fn getFullRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.get().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getTruncatedRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.getTruncated().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getIntersectionPercentagesFor(&self, identifier: &u32) -> Result<HashMap<u32, f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.intersections_percentages.get().get(identifier)
            .ok_or(GenericError::new(format!("Identifier {} not found", identifier).as_str(), file!(), &line!()))?
            .clone()
        );
    }

    

}