#![allow(non_snake_case)]
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;
use crate::model::analysis::metrics::metric::Metric;
use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::{DagService};
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::MetricsService;

pub struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,

    metrics_service: Option<MetricsService>,
    dag_service: Option<DagService>,

    current_identifier: u32,
    visible_identifiers: Vec<u32>,
}

impl ApplicationStateService{
    pub fn new() -> ApplicationStateService{
        return ApplicationStateService{
            tensor: None,
            identifier_mapper: None,

            metrics_service: None,
            dag_service: None,

            current_identifier: 0,
            visible_identifiers: vec![],
        };
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

        self.visible_identifiers = self.dag_service.as_ref().unwrap().getFontNodes();
        self.metrics_service = Some(metrics_service);
        
    }

    pub fn changeTensor(&mut self, tensor: Tensor, patterns: Vec<Pattern>){
        self.tensor = Some(tensor);
        self.changePatterns(patterns);
    }

    fn update(&mut self, new_visible_identifiers: Vec<u32>) {
        self.metrics_service.as_mut().unwrap()
            .update(self.identifier_mapper.as_ref().unwrap(), &new_visible_identifiers);

        self.visible_identifiers = new_visible_identifiers;
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

    pub fn truncate(&mut self, new_size: usize){
        let mut new_visible_identifiers: Vec<u32> = self.getMetricsService().rss_evolution.get().clone().iter()
            .map(|(identifier, _)| *identifier)
            .collect();

        new_visible_identifiers.truncate(new_size+1);

        self.update(new_visible_identifiers);
    }

    pub fn identifierMapper(&self) -> &IdentifierMapper{
        return self.identifier_mapper.as_ref().unwrap();
    }

    pub fn visibleIdentifiers(&self) -> &Vec<u32>{
        return &self.visible_identifiers;
    }

    pub fn getMetricsService(&self) -> &MetricsService{
        return self.metrics_service.as_ref().unwrap();
    }

    pub fn getDagService(&self) -> &DagService{
        return self.dag_service.as_ref().unwrap();
    }
}