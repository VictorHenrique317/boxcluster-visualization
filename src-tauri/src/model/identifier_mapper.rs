#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::database::{pattern::Pattern, dag_node::{DagNode, self}, datapoint::DataPoint};

use super::identifier_representation::IdentifierRepresentation;

pub struct IdentifierMapper{
    mapping: HashMap<u32, IdentifierRepresentation>, // WARNING: ID's start at 1
}

impl IdentifierMapper{
    pub fn new(pattern_representations: Vec<Pattern>) -> IdentifierMapper{
        return IdentifierMapper { 
            mapping: IdentifierMapper::createInitialMapping(pattern_representations),
        };
    }

    fn createInitialMapping(pattern_representations: Vec<Pattern>) -> HashMap<u32, IdentifierRepresentation>{
        let mut mapping: HashMap<u32, IdentifierRepresentation> = HashMap::new();

        for pattern_representation in pattern_representations {
            mapping.insert(pattern_representation.identifier, IdentifierRepresentation::new(pattern_representation));
        }

        return mapping;
    }

    pub fn insertDagNodeRepresentations(&mut self, dag_nodes_representations: Vec<DagNode>) {
        let dag_nodes_representations: HashMap<u32, DagNode> = dag_nodes_representations.into_iter()
            .map(|dag_node| (dag_node.identifier, dag_node))
            .collect();

        for (identifier, dag_nodes_representation) in dag_nodes_representations {
            let identifier_representation = self.mapping.get_mut(&identifier).unwrap();
            identifier_representation.insertDagNodeRepresentation(dag_nodes_representation);
        }
    }

    pub fn insertDataPointRepresentations(&mut self, data_point_representations: Vec<DataPoint>) {
        let data_point_representations: HashMap<u32, DataPoint> = data_point_representations.into_iter()
            .map(|data_point| (data_point.identifier, data_point))
            .collect();
        
        for (identifier, data_point_representation) in data_point_representations {
            let identifier_representation = self.mapping.get_mut(&identifier).unwrap();
            identifier_representation.insertDataPointRepresentation(data_point_representation);
        }
    }

    pub fn getRepresentation(&self, identifier: &u32) -> &IdentifierRepresentation{
        return self.mapping.get(identifier).unwrap();
    }

    pub fn getRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        return self.mapping.values().collect();
    }

    pub fn getIdentifiers(&self) -> Vec<u32>{
        let mut keys: Vec<u32> = self.mapping.keys().cloned().collect();
        keys.sort();
        return keys;
    }

    pub fn getOrderedRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        let mut keys: Vec<u32> = self.getIdentifiers();

        let values: Vec<&IdentifierRepresentation> = keys.iter()
            .map(|k| self.mapping.get(k).unwrap())
            .collect();
        return values;
    }

    pub fn length(&self) -> u32{
        return self.mapping.len() as u32;
    }
}