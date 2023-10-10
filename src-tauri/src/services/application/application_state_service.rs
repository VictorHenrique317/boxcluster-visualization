#![allow(non_snake_case)]
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;

use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::MetricsService;

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
    pub fn init(&mut self, tensor: Tensor, patterns: Vec<Pattern>){
        self.tensor = Some(tensor);
        self.changePatterns(patterns);
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>){
        // Inserts the pattern representations
        let mut identifier_mapper = IdentifierMapper::new(patterns);

        // Inserts the dag node representations
        identifier_mapper.insertDagNodeRepresentations(
            DagService::createAndArrange(&identifier_mapper),
        );

        // Inserts the data point representations
        let metrics_service = MetricsService::new(
                &identifier_mapper,
                &self.tensor.as_ref().unwrap(),
            );
        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &metrics_service.coordinates)
        );

        self.identifier_mapper = Some(identifier_mapper);
        let dag_service = DagService::new(self.identifierMapper());
        self.dag_service = Some(dag_service);

        self.current_level_identifiers = self.dag_service.as_ref().unwrap().getFontNodes();
        self.visible_identifiers = self.current_level_identifiers.clone();
        self.metrics_service = Some(metrics_service);
    }

    fn update(&mut self, new_current_level_identifiers: Vec<u32>) {
        let tensor = self.tensor.as_ref().unwrap();

        self.metrics_service.as_mut().unwrap()
            .update(tensor, self.identifier_mapper.as_ref().unwrap(), &new_current_level_identifiers);

        self.current_level_identifiers = new_current_level_identifiers.clone();
        self.visible_identifiers = new_current_level_identifiers;
    }

    pub fn ascendDag(&mut self) {
        if self.current_identifier == 0{ return; }

        let previous_identifiers = self.dag_service.as_ref().unwrap()
            .ascendDag(self.identifierMapper(), &self.current_identifier);
        self.update(previous_identifiers);
    }

    pub fn descendDag(&mut self, next_identifier: &u32) {
        let next_identifiers = self.dag_service.as_ref().unwrap()   
            .descendDag(self.identifierMapper(), next_identifier);

        if next_identifiers.len() == 0{
            return;
        }

        self.update(next_identifiers);
    }

    pub fn truncateModel(&mut self, new_size: &u32){
        let mut visible_identifiers = self.current_level_identifiers.clone();
        visible_identifiers.sort();
        visible_identifiers.truncate(*new_size as usize);
        
        self.metrics_service.as_mut().unwrap()
            .rss_evolution.truncate(new_size);
        self.visible_identifiers = visible_identifiers;
    }

    pub fn identifierMapper(&self) -> &IdentifierMapper{
        return self.identifier_mapper.as_ref().unwrap();
    }

    pub fn visibleIdentifiers(&self) -> &Vec<u32>{
        return &self.current_level_identifiers;
    }

    pub fn getMetricsService(&self) -> &MetricsService{
        return self.metrics_service.as_ref().unwrap();
    }

    pub fn getDagService(&self) -> &DagService{
        return self.dag_service.as_ref().unwrap();
    }
}