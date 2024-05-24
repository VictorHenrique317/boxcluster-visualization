#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use itertools::Itertools;
use plotters::data;

use crate::{common::generic_error::GenericError, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, pattern::Pattern, raw_pattern::RawPattern}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper, io::translator::Translator}, services::{io_service::IoService, plot_service::PlotService}};
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
            self.application_state_service.getMetricsService()?.all_initial_visible_distances.get()
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

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return self.application_state_service.getAllSubPatternsIdentifiers();
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

    pub fn getIntersectionsPercentages(&self, identifier: &u32) -> Result<HashMap<u32, f64>, GenericError> {
        let intesections_percentages = self.application_state_service.getMetricsService()?.intersections_percentages.get();
        return Ok(
            intesections_percentages.get(identifier)
                .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?
                .clone()
        );
    }

    pub fn getIntersectionDetails(&self, identifier: &u32) -> Result<IntersectionsDetails, GenericError>{
        let intersection_percentages: HashMap<u32, f64> = match self.application_state_service.getMetricsService()?
            .intersections_percentages.get().get(identifier){

            Some(intersection_percentages) => intersection_percentages.clone(),
            None => HashMap::new(),
        };

        let total_untouched_percentage = intersection_percentages.get(identifier)
            .expect("Should have a total untouched percentage, even if its 0").clone();
        let total_intersection_percentage = 1.0 - total_untouched_percentage;
        
        let current_pattern = self.getIdentifierMapper()?.getIdentifier(identifier)?.asPattern()?;
        let all_dims_intersections: Result<HashMap<u32, (f64, Vec<Vec<String>>)>, GenericError> = intersection_percentages.into_iter()
            .filter(|(other_identifier, _)| *other_identifier != *identifier)
            .map(|(other_identifier, percentage)| {

                let other_pattern = self.getIdentifierMapper()?.getIdentifier(&other_identifier)?.asPattern()?;
                
                let dims_intersections = current_pattern.dimIntersection(&other_pattern)?;
                let dims_intersections = self.getTranslator()
                    .untranslateLineDims(&dims_intersections)?.iter()
                    .map(|line| {
                        let values: Vec<String> = line.split(",").map(|dim| dim.to_string()).collect_vec();
                        return values;
                    })
                    .collect();

                return Ok((other_identifier, (percentage, dims_intersections)));
            })
            .collect();
        let all_dims_intersections = all_dims_intersections?;
        
        let intersections_details = IntersectionsDetails::new(*identifier, 
            total_untouched_percentage, total_intersection_percentage, all_dims_intersections);
        
        return Ok(intersections_details);
    }

    

}