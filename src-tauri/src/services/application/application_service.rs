#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, time::Instant};
use itertools::Itertools;

use crate::{common::generic_error::GenericError, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, pattern::Pattern, raw_pattern::RawPattern}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper, io::translator::Translator}, services::{io_service::IoService, plot_service::PlotService}};
use super::application_state_service::ApplicationStateService;

use ndarray::{IxDynImpl, Dim};

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

    pub fn getDatapointsWithSubPatterns(&self) -> Result<Vec<DataPoint>, GenericError>{
        let datapoints = self.getDataPoints()?;
        let mut datapoints_with_subpatterns: Vec<DataPoint> = vec![];

        for datapoint in datapoints.iter(){
            let identifier = datapoint.identifier;

            let subpatterns = &self.getIdentifierMapper()?.getRepresentation(&identifier)?.asDagNode()?.subs;
            if subpatterns.len() > 0{ datapoints_with_subpatterns.push(datapoint.clone()); }
        }
        
        return Ok(datapoints_with_subpatterns);
    }

    pub fn ascendDag(&mut self) -> Result<Vec<DataPoint>, GenericError>{
        let result = self.application_state_service.ascendDag()?;
        if !result { return Ok(Vec::new());}

        PlotService::plot(&self.application_state_service);
        return self.getDataPoints();
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<Vec<DataPoint>, GenericError> {
        let result = self.application_state_service.descendDag(next_identifier)?;
        if !result { return Ok(Vec::new());}

        PlotService::plot(&self.application_state_service);
        return self.getDataPoints();
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
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers()?;
        // let identifiers = self.application_state_service.getAllIdentifiers()?;
        println!("Getting datapoints for identifiers: {:?}", &visible_identifiers);
        let datapoints: Vec<DataPoint> = self.application_state_service.identifierMapper()?
            .getOrderedDataPointsFrom(&visible_identifiers).into_iter()
            .map(|datapoint| datapoint.clone())
            .collect();

        return Ok(datapoints);
    }
    
    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return self.application_state_service.getAllSubPatternsIdentifiers();
    }

    pub fn getRawPattern(&self, identifier: &u32) -> Result<RawPattern, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers()?;

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

        println!("Got intersection_percentages: {:?}", &intersection_percentages);
        
        let current_pattern = self.getIdentifierMapper()?.getIdentifier(identifier)?.asPattern()?;
        let mut all_dims_intersections: HashSet<Dim<IxDynImpl>> = HashSet::new();
        let intersections: Result<HashMap<u32, (f64, Vec<Vec<String>>)>, GenericError> = intersection_percentages.into_iter()
            // .filter(|(other_identifier, _)| *other_identifier != *identifier)
            .map(|(other_identifier, percentage)| {

                let other_pattern = self.getIdentifierMapper()?.getIdentifier(&other_identifier)?.asPattern()?;
                
                let dims_intersections = current_pattern.dimIntersection(&other_pattern)?;
                let indices_intersections: Vec<Dim<IxDynImpl>> = current_pattern.intersection(&other_pattern).iter()
                    .map(|index| Dim(index.clone()))
                    .collect();

                // Here we have to verify if other_pattern is subpattern of current_pattern
                let intersection_area: u32 = dims_intersections.iter()
                    .map(|dim| dim.len() as u32)
                    .product();

                println!("Pattern {} intersection with pattern {} has size {}", identifier, other_identifier, intersection_area);
                
                if intersection_area != other_pattern.size{ // other_pattern is not subpattern of current_pattern
                    for index_intersection in indices_intersections.iter(){
                        all_dims_intersections.insert(index_intersection.clone());
                    }
                }
                
                // Translate the intersections to the original dimensions
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
        let intersections = intersections?;

        println!("Indices intersection size: {:?}", all_dims_intersections.len());

        let total_intersection_percentage: f64 = all_dims_intersections.len() as f64 / current_pattern.size as f64;
        let total_untouched_percentage = 1.0 - total_intersection_percentage;

        if total_intersection_percentage < 0.0 || total_intersection_percentage > 1.0{
            return Err(GenericError::new(format!("Total intersection percentage ({:?}) is not between 0.0 and 1.0", total_intersection_percentage).as_str(), file!(), &line!()));
        }

        if total_untouched_percentage < 0.0 || total_untouched_percentage > 1.0{
            return Err(GenericError::new(format!("Total untouched percentage ({:?}) is not between 0.0 and 1.0", total_untouched_percentage).as_str(), file!(), &line!()));
        }
        
        let intersections_details = IntersectionsDetails::new(*identifier, 
            total_untouched_percentage, total_intersection_percentage, intersections);
        
        return Ok(intersections_details);
    }

    pub fn getCurrentDagLevel(&self) -> Result<u32, GenericError>{
        return Ok(self.application_state_service.getCurrentDagLevel()?);
    }

    pub fn getCurrentLevelBackgroundDensity(&self) -> Result<f64, GenericError> {
        return self.application_state_service.getCurrentLevelBackgroundDensity();
    }

    pub fn getAllDimsValues(&self) -> Result<Vec<Vec<String>>, GenericError> {
        let mut all_dims_values: Vec<HashSet<String>> = self.getIdentifierMapper()?.getRepresentation(&1)?.asPattern()?
            .dims_values.iter().map(|_| HashSet::new()).collect();

        for representation in self.getIdentifierMapper()?.getRepresentations().iter(){
            let pattern = representation.asRawPattern(self.getTranslator())?;

            for (i, dim_values) in pattern.dims_values.iter().enumerate(){
                for value in dim_values.iter(){
                    all_dims_values.get_mut(i).expect("Should have dimension").insert(value.to_string());
                }
            }
        }

        let all_dims_values: Vec<Vec<String>> = all_dims_values.iter()
            .map(|dim_values| dim_values.iter().map(|value| value.to_string()).collect())
            .collect();

        return Ok(all_dims_values);
    }

    pub fn filterDatapoints(&self, filters: &Vec<Vec<String>>) -> Result<Vec<DataPoint>, GenericError>{
        let current_level_identifiers = self.application_state_service.getDagService()?.getCurrentLevelIdentifiers();
        let raw_patterns: Vec<RawPattern> = self.application_state_service.identifierMapper()?
            .getOrderedRawPatternsFrom(current_level_identifiers, self.getTranslator());

        let filters: Vec<HashSet<String>> = filters.iter()
            .map(|filter| filter.iter().map(|value| value.to_string()).collect())
            .collect();

        let mut filtered_datapoints: Vec<DataPoint> = vec![];
        for raw_pattern in raw_patterns.iter(){
            let mut match_filters = true;
            for (dim, filter) in filters.iter().enumerate(){
                let current_dim: HashSet<String> = raw_pattern.dims_values.get(dim).expect("Should have dimension").iter()
                    .map(|value| value.to_string()).collect();

                if current_dim.intersection(&filter).count() == 0{ // Current dim does not match filter
                    match_filters = false;
                    break;
                }
            }

            if match_filters{
                let datapoint = self.getIdentifierMapper()?.getRepresentation(&raw_pattern.identifier)?.asDataPoint()?.clone();
                filtered_datapoints.push(datapoint);
            }
        }

        return Ok(filtered_datapoints);
    }

    pub fn getNbOfSubpatterns(&self, identifier: &u32) -> Result<u32, GenericError>{
        let nb_of_subpatterns = self.application_state_service.identifierMapper()?.getRepresentation(identifier)?.asDagNode()?.subs.len();
        return Ok(nb_of_subpatterns as u32);
    }
}