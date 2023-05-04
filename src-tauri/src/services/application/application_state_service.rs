#![allow(non_snake_case)]
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;
use crate::model::identifier_mapper::IdentifierMapper;
use crate::services::dag::dag_service::DagService;
use crate::services::metrics_service::MetricsService;

pub(in crate::services::application) struct ApplicationStateService<'a>{
    tensor: Option<Tensor>,

    dag_service: Option<DagService<'a>>,
    metrics_service: Option<MetricsService>,

    identifier_mapper: Option<IdentifierMapper>,
    visible_identifiers: Vec<u32>, // TODO: Implement
    
    
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

    fn changeState(&mut self, tensor: Tensor, patterns: Vec<Pattern>){
        self.tensor = Some(tensor);
        self.identifier_mapper = Some(IdentifierMapper::new(patterns));

        self.dag_service = Some(
            DagService::new(&self.identifier_mapper.as_ref().unwrap())
        );

        self.metrics_service = Some(
            MetricsService::new(
                &self.identifier_mapper.as_ref().unwrap(),
                &self.tensor.as_ref().unwrap()
            )
        );

        // UPDATE VISIBLE PATTERNS

    }
}