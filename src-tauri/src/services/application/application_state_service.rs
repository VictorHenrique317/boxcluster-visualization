#![allow(non_snake_case)]
use crate::database::datapoint::DataPoint;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;
use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::mds_service::MDSService;
use crate::services::metrics_service::MetricsService;

pub(in crate::services::application) struct ApplicationStateService<'a>{
    tensor: Option<Tensor>,

    dag_service: Option<DagService<'a>>,
    metrics_service: Option<MetricsService>,

    identifier_mapper: Option<IdentifierMapper>,
    visible_identifiers: Vec<u32>,
    
    
}

impl ApplicationStateService<'_>{
    pub fn new<'a>() -> ApplicationStateService<'a>{
        return ApplicationStateService{
            tensor: None,

            dag_service: None,
            metrics_service: None,

            identifier_mapper: None,
            visible_identifiers: vec![],
            
        };
    }
    pub fn changePatterns(&mut self, patterns: Vec<Pattern>){
        let identifier_mapper = IdentifierMapper::new(patterns);
        
        let dag_service = DagService::new(&identifier_mapper);
        identifier_mapper.insertDagNodeRepresentations(
            dag_service.createAndArrange()
        );

        let metrics_service = MetricsService::new(
                &identifier_mapper,
                &self.tensor.as_ref().unwrap()
            );
        let coords = MDSService::fitTransform(&metrics_service.distances, &identifier_mapper);
        let data_point_representations = DataPoint::createDataPoints(&identifier_mapper, &coords);
        self.identifier_mapper.insertDataPointRepresentations(data_point_representations);
        
        
        self.identifier_mapper = Some(identifier_mapper);
        self.dag_service = Some(dag_service);
        self.metrics_service = Some(metrics_service);
        self.visible_identifiers = patterns.iter()
            .map(|pattern| pattern.identifier)
            .collect();
    }

    pub fn changeTensor(&mut self, tensor: Tensor, patterns: Vec<Pattern>){
        self.tensor = Some(tensor);
        self.changePatterns(patterns);
    }

    pub fn changeVisibleIdentifiers(&mut self, visible_identifiers: &Vec<u32>){
        self.visible_identifiers = visible_identifiers.clone();
    }
}