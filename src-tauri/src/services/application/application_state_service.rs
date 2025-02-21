#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fmt::format;

use crate::common::generic_error::GenericError;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;

use crate::model::analysis::metrics::coordinates::Coordinates;
use crate::model::analysis::metrics::intersection::intersections_percentages::IntersectionsPercentages;
use crate::model::analysis::metrics::metric::Metric;
use crate::model::analysis::metrics::rss_evolution::RssEvolution;
use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::MetricsService;

#[derive(Eq, Hash, PartialEq)]
struct StateIdentifier{
    font_identifier: u32,
    dag_level: u32,
    nb_visible_identifiers: u32,
}

impl StateIdentifier{
    pub fn new(font_identifier: u32, dag_level: u32, nb_visible_identifiers: u32) -> StateIdentifier{
        return StateIdentifier{
            font_identifier: font_identifier,
            dag_level: dag_level,
            nb_visible_identifiers: nb_visible_identifiers,
        };
    }
}

#[derive(Clone)]
pub struct State{
    pub rss_evolution: RssEvolution,
    pub coordinates: Coordinates,
    pub intersections_percentages: IntersectionsPercentages,
}

#[derive(Default)]
pub struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,

    states: HashMap<StateIdentifier, State>,

    dag_service: Option<DagService>,
    metrics_service: Option<MetricsService>,
}

impl ApplicationStateService{
    pub fn init(&mut self, tensor: Tensor, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        self.tensor = Some(tensor);
        self.changePatterns(patterns)?;
        self.storeCurrentState()?;

        return Ok(());
    }

    fn loadState(&mut self, state_identifier: &StateIdentifier, new_visible_identifiers: &Vec<u32>) -> Result<(), GenericError>{
        self.getMutMetricsService()?.setVisibleIdentifiers(new_visible_identifiers);

        let state = self.states.get(state_identifier)
            .ok_or(GenericError::new("State not found", file!(), &line!()))?
            .clone();

        let identifier_mapper = self.identifier_mapper.as_mut()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()))?;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &state.coordinates)?
        )?;

        self.getMutMetricsService()?.loadFrom(state);
        return Ok(());
    }

    fn storeCurrentState(&mut self) -> Result<(), GenericError>{
        let state_identifier = StateIdentifier::new(
            self.getDagService()?.getCurrentFontIdentifier(),
            self.getDagService()?.getCurrentLevel(),
            self.getVisibleIdentifiers()?.len() as u32,
        );

        let rss_evolution = self.getMetricsService()?.rss_evolution.clone();
        let coordinates = self.getMetricsService()?.coordinates.clone();
        let intersections_percentages = self.getMetricsService()?.intersections_percentages.clone();
        let state = State{
            rss_evolution: rss_evolution,
            coordinates: coordinates,
            intersections_percentages: intersections_percentages,
        };

        self.states.insert(state_identifier,state);
        return Ok(());
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        // Inserts the pattern representations
        let mut identifier_mapper = IdentifierMapper::new(patterns);

        // Inserts the dag node representations
        identifier_mapper.insertDagNodeRepresentations(
            DagService::createAndArrange(&identifier_mapper)?,
        )?;
        
        let tensor_density = self.tensor.as_ref()
            .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?
            .density;  
        
        let dag_service = DagService::new(&identifier_mapper, &tensor_density)?;
        let visible_identifiers = dag_service.getCurrentLevelIdentifiers().clone();
        self.dag_service = Some(dag_service);

        // Inserts the data point representations
        let metrics_service = MetricsService::new(
            &identifier_mapper,
            self.tensor.as_ref()
                .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?,
                visible_identifiers,
        )?;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &metrics_service.coordinates)?
        )?;

        self.metrics_service = Some(metrics_service);
        self.identifier_mapper = Some(identifier_mapper);

        return Ok(());
    }

    fn update(&mut self, new_visible_identifiers: Vec<u32>, lazy: bool, use_states: bool) -> Result<(), GenericError>{
        if use_states { 
            // Try to load the state from the states hashmap, if it exists
            let state_identifier = StateIdentifier::new(
                self.getDagService()?.getCurrentFontIdentifier(),
                self.getDagService()?.getCurrentLevel(),
                new_visible_identifiers.len() as u32,
            );

            if self.states.contains_key(&state_identifier){
                println!("  Loading state from memory...");
                self.loadState(&state_identifier, &new_visible_identifiers)?;
                return Ok(());
            }
        }

        let tensor = self.tensor.as_ref()
            .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?;

        let identifier_mapper = self.identifier_mapper.as_mut()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()))?;

        self.metrics_service.as_mut().ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .update(tensor, identifier_mapper, &new_visible_identifiers, &lazy)?;

        let coordinates = &self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .coordinates;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, coordinates)?
            )?;

        if use_states { self.storeCurrentState()?; }

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<bool, GenericError>{
        let result = self.getMutDagService()?.ascendDag();
        if !result{ return Ok(false); }
        println!("  Ascended dag to level {:?}", self.getDagService()?.getCurrentLevelIdentifiers());

        let new_visible_identifiers = self.getMutDagService()?.getCurrentLevelIdentifiers().clone();
        print!("  Visible identifiers in this level: {:?}", &new_visible_identifiers);
        self.update(new_visible_identifiers, false, true)?;
        return Ok(true);
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<bool, GenericError>{
        let subs = self.identifierMapper()?.getRepresentation(next_identifier)?
            .asDagNode()?.subs.clone();
        let pattern_density = self.identifierMapper()?
            .getRepresentation(next_identifier)?.asPattern()?.density;
        let visible_identifiers= self.getVisibleIdentifiers()?;
        let result = self.getMutDagService()?.descendDag(next_identifier, &subs, &pattern_density, &visible_identifiers);
        if !result{ return Ok(false); }
        println!("  Descended dag to {}", next_identifier);

        let new_visible_identifiers = self.getDagService()?.getCurrentLevelIdentifiers().clone();
        self.update(new_visible_identifiers, false, true)?;
        return Ok(true);
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<(), GenericError>{
        let mut all_identifiers = self.identifierMapper()?
            .getIdentifiers().clone();
            
        all_identifiers.sort();
        all_identifiers.truncate(*new_size as usize);

        let mut current_level_identifiers = self.getMetricsService()?.getInitialVisibleIdentifiers().clone();
        current_level_identifiers.sort();

        let visible_identifiers: Vec<u32> = current_level_identifiers.iter()
            .filter(|identifier| all_identifiers.contains(&identifier))
            .map(|identifier| identifier.clone())
            .collect();

        println!("  New truncated visible identifiers {:?}", &visible_identifiers);
        self.update(visible_identifiers, true, true)?;

        return Ok(());
    }

    pub fn getDagService(&self) -> Result<&DagService, GenericError>{
        return self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()));
    }

    fn getMutDagService(&mut self) -> Result<&mut DagService, GenericError>{
        return self.dag_service.as_mut()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()));
    }

    pub fn identifierMapper(&self) -> Result<&IdentifierMapper, GenericError>{
        return self.identifier_mapper.as_ref()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()));
    }

    pub fn getVisibleIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return Ok(self.getMetricsService()?.visible_identifiers.clone());
    }

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return Ok(
            self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getSubNodes()
        );
    }

    fn getMutMetricsService(&mut self) -> Result<&mut MetricsService, GenericError>{
        return Ok(
            self.metrics_service.as_mut()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
        );
    }

    pub fn getMetricsService(&self) -> Result<&MetricsService, GenericError>{
        return Ok(
            self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
        );
    }

    pub fn getCurrentDagLevel(&self) -> Result<u32, GenericError>{
        return Ok(self.getDagService()?.getCurrentLevel());
    }

    pub fn getCurrentLevelBackgroundDensity(&self) -> Result<f64, GenericError>{
        return Ok(self.getDagService()?.getCurrentLevelBackgroundDensity());
    }
}