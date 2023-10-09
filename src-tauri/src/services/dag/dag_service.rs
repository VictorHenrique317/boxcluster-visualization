#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::database::dag_node::DagNode;
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::{DagCreatorService};

pub struct DagService{
    font_nodes: Vec<u32>,
}

impl DagService{
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

    fn calculateFontNodes(identifier_mapper: &IdentifierMapper) -> Vec<u32>{
        let mut font_nodes: Vec<u32> = Vec::new();
        for representation in identifier_mapper.getRepresentations(){
            let dag_node = representation.asDagNode();

            if dag_node.supers.len() == 0{
                font_nodes.push(dag_node.identifier);
            }
        }

        return font_nodes;
    }

    pub fn new(identifier_mapper: &IdentifierMapper) -> DagService{
        return DagService{
            font_nodes: DagService::calculateFontNodes(identifier_mapper),
        };
    }

    pub fn getFontNodes(&self) -> Vec<u32> {
        return self.font_nodes.clone();
    }

    pub fn ascendDag(&self, identifier_mapper: &IdentifierMapper, current_identifier: &u32) -> Vec<u32> {
        let supers = &identifier_mapper.getRepresentation(current_identifier).asDagNode().supers;
        if supers.len() == 0{
            return self.getFontNodes();
        }

        return supers.clone();
    }

    pub fn descendDag(&self, identifier_mapper: &IdentifierMapper, next_identifier: &u32) -> Vec<u32> {
        let dag_node = identifier_mapper.getRepresentation(next_identifier).asDagNode();
        return dag_node.subs.clone();
    }

    pub fn getFlattenedSubs(&self, identifier_mapper: &IdentifierMapper) -> HashMap<u32, Vec<u32>>{
        let dag_nodes: Vec<&DagNode> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes{
            flattened_subs.insert(dag_node.identifier, dag_node.subs.clone());
        }

        return flattened_subs;
    }

    pub fn getFlattenedSupers(&self, identifier_mapper: &IdentifierMapper) -> HashMap<u32, Vec<u32>>{
        let dag_nodes: Vec<&DagNode> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes{
            flattened_supers.insert(dag_node.identifier, dag_node.supers.clone());
        }

        return flattened_supers;
    }
    
}