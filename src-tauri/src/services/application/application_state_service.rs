#![allow(non_snake_case)]
use crate::database::datapoint::DataPoint;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;
use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::MetricsService;

pub(in crate::services::application) struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,
    visible_identifiers: Vec<u32>,

    metrics_service: Option<MetricsService>,
}

impl ApplicationStateService{
    pub fn new() -> ApplicationStateService{
        return ApplicationStateService{
            tensor: None,
            identifier_mapper: None,
            visible_identifiers: vec![],

            metrics_service: None,
        };
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>){
        self.visible_identifiers = patterns.iter()
            .map(|pattern| pattern.identifier)
            .collect();

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
        self.metrics_service = Some(metrics_service);
    }

    pub fn changeTensor(&mut self, tensor: Tensor, patterns: Vec<Pattern>){
        self.tensor = Some(tensor);
        self.changePatterns(patterns);
    }

    pub fn changeVisibleIdentifiers(&mut self, visible_identifiers: &Vec<u32>){
        self.visible_identifiers = visible_identifiers.clone();
    }

    pub fn identifierMapper(&self) -> &IdentifierMapper{
        return self.identifier_mapper.as_ref().unwrap();
    }
}