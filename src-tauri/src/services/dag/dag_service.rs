#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::database::tensor::Tensor;
use crate::{database::dag_node::DagNode, common::generic_error::GenericError};
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::DagCreatorService;

pub struct DagService{
    sub_nodes: Vec<u32>,

    current_font_identifier: u32,
    current_level_density: f64,
    current_level_identifiers: Vec<u32>,
    level_history: Vec<(u32, f64, Vec<u32>)>,
}

impl DagService{
    fn createFlatDagNodes(identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let mut nodes: Vec<DagNode> = Vec::new();
        for id in identifier_mapper.getIdentifiers(){
            nodes.push(DagNode::new(&id));
        }
        return nodes;
    }

    pub fn createAndArrange(identifier_mapper: &IdentifierMapper) -> Result<Vec<DagNode>, GenericError> {
        let flat_dag_nodes = DagService::createFlatDagNodes(identifier_mapper);
        let dag_creator_service = DagCreatorService::new(identifier_mapper);
        return dag_creator_service.create(flat_dag_nodes);
    }

    fn identifyNodes(identifier_mapper: &IdentifierMapper) -> Result<(Vec<u32>, Vec<u32>), GenericError>{
        let mut font_nodes: Vec<u32> = Vec::new();
        let mut sub_nodes: Vec<u32> = Vec::new();

        for representation in identifier_mapper.getRepresentations(){
            let dag_node = representation.asDagNode()?;

            if dag_node.supers.len() == 0{
                font_nodes.push(dag_node.identifier);
            }else{
                sub_nodes.push(dag_node.identifier);
            }
        
        }

        return Ok((font_nodes, sub_nodes));
    }

    pub fn new(identifier_mapper: &IdentifierMapper, tensor_density: &f64) -> Result<DagService, GenericError>{
        let (font_nodes, sub_nodes) = DagService::identifyNodes(identifier_mapper)?;
        return Ok(
            DagService{
                sub_nodes: sub_nodes,

                current_font_identifier: 0,
                current_level_density: *tensor_density,
                current_level_identifiers: font_nodes.clone(),
                level_history: vec![(0, *tensor_density, font_nodes)],
            }
        );
    }

    pub fn getFontNodes(&self) -> Result<Vec<u32>, GenericError> {
        return Ok(
            self.level_history.first().cloned().ok_or(GenericError::new("No font nodes", file!(), &line!()))?.2
        );
    }

    pub fn getSubNodes(&self) -> Vec<u32> {
        return self.sub_nodes.clone();
    }

    pub fn ascendDag(&mut self) -> bool {
        if self.level_history.len() == 0 { return false; }

        //Can ascend
        let _ = self.level_history.pop()
            .expect("If can ascend should have previous identifiers").clone();

        let current = self.level_history.last()
            .expect("If can descend should have next identifiers").clone();
        self.current_font_identifier = current.0;
        self.current_level_density = current.1;
        self.current_level_identifiers = current.2;
        return true;
    }

    pub fn descendDag(&mut self, font_identifier: &u32, subs: &Vec<u32>, pattern_density: &f64) -> bool {
        if subs.len() == 0{ return false; }

        // Has some sub, can descend
        self.level_history.push((*font_identifier, *pattern_density, subs.clone()));
        
        let current = self.level_history.last()
            .expect("If can descend should have next identifiers").clone();
        self.current_font_identifier = current.0;
        self.current_level_density = current.1;
        self.current_level_identifiers = current.2;
        return true;
    }

    pub fn getCurrentLevel(&self) -> u32{
        return self.level_history.len() as u32;
    }

    pub fn getCurrentFontIdentifier(&self) -> u32{
        return self.current_font_identifier;
    }

    pub fn getCurrentLevelIdentifiers(&self) -> &Vec<u32>{
        return &self.current_level_identifiers;
    }

    pub fn getFlattenedSubs(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_subs.insert(dag_node.identifier, dag_node.subs.clone());
        }

        return Ok(flattened_subs);
    }

    pub fn getFlattenedSupers(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_supers.insert(dag_node.identifier, dag_node.supers.clone());
        }

        return Ok(flattened_supers);
    }

    pub fn setCurrentLevelIdentifiers(&mut self, identifiers: &Vec<u32>, level_density: &f64){
        self.current_level_identifiers = identifiers.clone();
        self.level_history = vec![(0, *level_density, identifiers.clone())];
    }

    pub fn getCurrentLevelBackgroundDensity(&self) -> f64{
        return self.current_level_density;
    }
    
}