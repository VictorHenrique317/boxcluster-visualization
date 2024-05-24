#![allow(non_snake_case)]
use crate::common::generic_error::GenericError;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;

use crate::model::analysis::metrics::metric::Metric;
use crate::model::identifier_mapper::{IdentifierMapper, self};
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::{MetricsService, self};

#[derive(Default)]
pub struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,

    metrics_service: Option<MetricsService>,
    dag_service: Option<DagService>,

    current_identifier: u32,
    current_level_identifiers: Vec<u32>,
    visible_identifiers: Vec<u32>,
}

impl ApplicationStateService{
    pub fn init(&mut self, tensor: Tensor, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        self.tensor = Some(tensor);
        self.changePatterns(patterns)?;

        return Ok(());
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        // Inserts the pattern representations
        let mut identifier_mapper = IdentifierMapper::new(patterns);

        // Inserts the dag node representations
        identifier_mapper.insertDagNodeRepresentations(
            DagService::createAndArrange(&identifier_mapper)?,
        )?;
        
        let dag_service = DagService::new(&identifier_mapper)?;
        self.dag_service = Some(dag_service);

        self.current_level_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getFontNodes();

        self.visible_identifiers = self.current_level_identifiers.clone();

        // Inserts the data point representations
        let metrics_service = MetricsService::new(
            &identifier_mapper,
            self.tensor.as_ref()
                .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?,
            &self.visible_identifiers
        )?;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &metrics_service.coordinates)?
        )?;

        self.metrics_service = Some(metrics_service);
        self.identifier_mapper = Some(identifier_mapper);

        return Ok(());
    }

    fn update(&mut self, new_current_level_identifiers: &Option<Vec<u32>>) -> Result<(), GenericError>{
        let tensor = self.tensor.as_ref()
            .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?;

        let identifier_mapper = self.identifier_mapper.as_mut()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()))?;

        let lazy = match new_current_level_identifiers {
            Some(_) => false, // Changing the current level identifiers has to be done eagerly
            None => true, // Here we do not need to re-calculate rss_evolution
        };

        let identifiers_used_to_update = match new_current_level_identifiers {
            Some(new_current_level_identifiers) => new_current_level_identifiers, // Updates all identifiers and reset visible identifiers (dag movement)
            None => &self.visible_identifiers, // Updates only the visible identifiers, (truncation)
        };

        self.metrics_service.as_mut().ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .update(tensor, identifier_mapper, identifiers_used_to_update, &lazy)?;

        let coordinates = &self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .coordinates;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, coordinates)?
        )?;

        // Should also insert dagNode representations
        
        if !lazy{ // Reset everything because current_level_identifiers is gonna be changed
            self.current_level_identifiers = identifiers_used_to_update.clone();
            self.visible_identifiers = identifiers_used_to_update.clone();
        }

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<(), GenericError>{
        if self.current_identifier == 0{ return Ok(()); }

        let previous_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .ascendDag(self.identifierMapper()?, &self.current_identifier)?;

        self.update(&Some(previous_identifiers))?;

        return Ok(());
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<(), GenericError>{
        let next_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .descendDag(self.identifierMapper()?, next_identifier)?;

        if next_identifiers.len() == 0{ return Ok(()); }

        self.update(&Some(next_identifiers))?;
        return Ok(());
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<(), GenericError>{
        let mut all_identifiers = self.identifierMapper()?
            .getIdentifiers().clone();
            
        all_identifiers.sort();
        all_identifiers.truncate(*new_size as usize);

        let mut current_level_identifiers = self.current_level_identifiers.clone();
        current_level_identifiers.sort();

        self.visible_identifiers = current_level_identifiers.iter()
            .filter(|identifier| all_identifiers.contains(&identifier))
            .map(|identifier| identifier.clone())
            .collect();

        self.update(&None)?;

        return Ok(());
    }

    pub fn identifierMapper(&self) -> Result<&IdentifierMapper, GenericError>{
        return self.identifier_mapper.as_ref()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()));
    }

    pub fn getAllIdentifiers(&self) -> Result<&Vec<u32>, GenericError>{
        return Ok(
            &self.current_level_identifiers
        );
    }

    pub fn getVisibleIdentifiers(&self) -> &Vec<u32>{
        return &self.visible_identifiers;
    }

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return Ok(
            self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getSubNodes()
        );
    }

    pub fn getMetricsService(&self) -> Result<&MetricsService, GenericError>{
        return Ok(
            self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
        );
    }

    pub fn getDagService(&self) -> Result<&DagService, GenericError>{
        return self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()));
    }
}