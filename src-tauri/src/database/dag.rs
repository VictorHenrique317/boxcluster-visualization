#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use super::{dag_node::DagNode, pattern::Pattern};

#[derive(Default)]
pub struct Dag {
    nodes: HashMap<u32, DagNode>,
}

impl Dag {
    pub fn new(patterns: &Vec<&Pattern>) -> Self {
        return Dag { 
            nodes: Dag::createNodes(patterns),
        };
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        return self.nodes.keys().map(|i| *i).collect();
    }


    pub fn getNumberOfNodes(&self) -> u32 {
        return self.nodes.len() as u32;
    }

    pub fn isEdge(&self, node: &u32) -> bool {
        let node_p = self.nodes.get(node).unwrap();
        return node_p.subs.len() == 0;
    }

    pub fn isFont(&self, node: &u32) -> bool {
        return self.nodes.get(node).unwrap().supers.len() == 0;
    }

    pub fn hasSubs(&self, node: &u32) -> bool {
        return self.nodes.get(node).unwrap().subs.len() != 0;
    }

    pub fn getOverllapings(&self) -> HashMap<u32, HashSet<u32>>{
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (id, node) in self.nodes.iter(){
            overlappings.insert(*id, node.overlappings.clone());
        }
        
        return overlappings;
    }
    
    pub fn extractNodes(self) -> HashMap<u32, DagNode>{
        return self.nodes;
    }

    fn createNodes(_patterns: &[&Pattern]) -> HashMap<u32, DagNode> {
        todo!()
    }

}
