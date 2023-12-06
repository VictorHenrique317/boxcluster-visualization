#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{database::{pattern::Pattern, dag_node::DagNode, datapoint::DataPoint}, common::generic_error::GenericError};

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

    pub fn insertDagNodeRepresentations(&mut self, dag_nodes_representations: Vec<DagNode>) -> Result<(), GenericError>{
        let dag_nodes_representations: HashMap<u32, DagNode> = dag_nodes_representations.into_iter()
            .map(|dag_node| (dag_node.identifier, dag_node))
            .collect();

        for (identifier, dag_nodes_representation) in dag_nodes_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDagNodeRepresentation(dag_nodes_representation);
        }

        return Ok(());
    }

    pub fn insertDataPointRepresentations(&mut self, data_point_representations: Vec<DataPoint>) -> Result<(), GenericError>{
        let data_point_representations: HashMap<u32, DataPoint> = data_point_representations.into_iter()
            .map(|data_point| (data_point.identifier, data_point))
            .collect();
        
        for (identifier, data_point_representation) in data_point_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDataPointRepresentation(data_point_representation);
        }

        return Ok(());
    }

    pub fn getRepresentation(&self, identifier: &u32) -> Result<&IdentifierRepresentation, GenericError>{
        return self.mapping.get(identifier)
            .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()));
    }

    pub fn getRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        return self.mapping.values().collect();
    }

    pub fn getRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        return identifiers.iter()
            .filter_map(|identifier| self.getRepresentation(identifier).ok())
            .collect();
    }

    pub fn getOrderedRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        let mut identifiers = identifiers.clone();
        identifiers.sort();

        let representations = self.getRepresentationsFrom(&identifiers);
        // Representations will be naturally ordered
        return representations;
    }

    pub fn getIdentifiers(&self) -> Vec<u32>{
        let mut keys: Vec<u32> = self.mapping.keys().cloned().collect();
        keys.sort();
        return keys;
    }

    pub fn getMapping(&self) -> &HashMap<u32, IdentifierRepresentation>{
        return &self.mapping;
    }

    pub fn getOrderedRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        let keys: Vec<u32> = self.getIdentifiers();

        let values: Vec<&IdentifierRepresentation> = keys.iter()
            .map(|k| self.mapping.get(k)
                .expect("Should have gotten identifier representation"))
            .collect();
        return values;
    }

    pub fn getOrderedPatterns(&self) -> Vec<&Pattern> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedPatternsFrom(&self, identifiers: &Vec<u32>) -> Vec<&Pattern> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedDataPoints(&self) -> Vec<&DataPoint> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn getOrderedDataPointsFrom(&self, identifiers: &Vec<u32>) -> Vec<&DataPoint> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn length(&self) -> u32{
        return self.mapping.len() as u32;
    }
}