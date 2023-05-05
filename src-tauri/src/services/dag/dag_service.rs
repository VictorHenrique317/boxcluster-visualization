#![allow(non_snake_case)]
use crate::database::dag_node::DagNode;
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::{DagCreatorService, self};

pub struct DagService{
}

impl DagService {
    fn createFlatDagNodes(identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let mut nodes: Vec<DagNode> = Vec::new();
        for id in identifier_mapper.getIdentifiers(){
            nodes.push(DagNode::new(&id));
        }
        return nodes;
    }

    pub fn createAndArrange(identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let flat_dag_nodes = DagService::createFlatDagNodes(identifier_mapper);
        let dag_creator_service = DagCreatorService::new(identifier_mapper);
        return dag_creator_service.create(flat_dag_nodes);
    }
}