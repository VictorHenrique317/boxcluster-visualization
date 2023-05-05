#![allow(non_snake_case)]
use crate::database::dag_node::DagNode;
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::DagCreatorService;

pub struct DagService<'a>{
    identifier_mapper: &'a IdentifierMapper,
    dag_creator_service: DagCreatorService<'a>,
}

impl DagService<'_> {
    pub fn new<'a>(identifier_mapper: &IdentifierMapper) -> DagService<'a> {
        return DagService {
            identifier_mapper: identifier_mapper,
            dag_creator_service: DagCreatorService::new(identifier_mapper),
        }
    }

    fn createFlatDagNodes(&self, identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let mut nodes: Vec<DagNode> = Vec::new();
        for id in identifier_mapper.getIdentifiers(){
            nodes.push(DagNode::new(&id));
        }
        return nodes;
    }

    pub fn createAndArrange(&self) -> Vec<DagNode> {
        let flat_dag_nodes = self.createFlatDagNodes(&self.identifier_mapper);
        return self.dag_creator_service.create(flat_dag_nodes);
    }
}