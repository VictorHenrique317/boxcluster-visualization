#![allow(non_snake_case)]
use debug_print::debug_println;
use std::collections::{HashMap, HashSet};

use super::{dag_node::DagNode, pattern::Pattern};

#[derive(Default)]
pub struct Dag {
    fonts: Vec<u32>,
    nodes: HashMap<u32, DagNode>,
}

impl Dag {
    pub fn new(patterns: &Vec<&Pattern>) -> Self {
        return Dag { 
            fonts: Vec::new(), 
            nodes: Dag::createNodes(patterns),
        };
    }

    fn createNodes(patterns: &Vec<&Pattern>) -> HashMap<u32, DagNode>{
        let mut nodes: HashMap<u32, DagNode> = HashMap::new();
        for (i, pattern) in patterns.iter().enumerate(){
            nodes.insert(pattern.identifier, DagNode::new(&pattern.identifier));
        }
        return nodes;
    }

    pub fn addOverlappingNode(&mut self, overlapped_node: &u32, overlapping_node: &u32){
        let overlapped_node = self.nodes.get_mut(overlapped_node).unwrap();
        overlapped_node.overlappings.insert(*overlapping_node);
    }

    pub fn moveSubtreeBellow(&mut self, moving_node: &u32, new_parent: &u32) {
        let mut moving_node_p = self.nodes.get_mut(&moving_node).unwrap();
        let old_parents: Vec<u32> = moving_node_p.supers.clone();
        moving_node_p.supers = vec![*new_parent]; // Removes old parents and adds new super of moving node

        for old_parent in old_parents{ // Deletes moving node from its old parents
            let old_parent_p = self.nodes.get_mut(&old_parent).unwrap();
            old_parent_p.subs.retain(|p| p != moving_node);
        }

        let new_parent = self.nodes.get_mut(&new_parent).unwrap();
        new_parent.subs.push(*moving_node); // Adds moving node to its new super
    }

    pub fn addBellow(&mut self, adding_node: &u32, parent: &u32){
        let adding_node_p = self.nodes.get_mut(&adding_node).unwrap();
        adding_node_p.supers.push(*parent);

        let parent_p = self.nodes.get_mut(&parent).unwrap();
        parent_p.subs.push(*adding_node);
    }

    pub fn addFont(&mut self, new_font: &u32){
        debug_println!("    {} is now a font", new_font);
        if !self.fonts.contains(new_font){
            self.fonts.push(*new_font);
        }
    }

    pub fn removeFont(&mut self, old_font: &u32){
        debug_println!("    {} is not a font anymore", old_font);
        self.fonts.retain(|f| f != old_font);
    }

    pub fn traverse(&self, to_node: &u32) -> &Vec<u32>{
        return &self.nodes.get(to_node).unwrap().subs;
    }

    pub fn getNode(&self, identifier: &u32) -> &DagNode{
        return self.nodes.get(identifier).unwrap();
    }

    // pub fn getNodes(&self) -> HashMap<u32, &DagNode>{
    //     return self.nodes.iter()
    //         .map(|(id, n)| (*id, n))
    //         .collect();
    // }

    pub fn getFontNodes(&self) -> Vec<&DagNode> {
        let mut font_nodes: Vec<&DagNode> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(self.getNode(font));
        }   

        return font_nodes;
    }
    
    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        return self.nodes.keys().map(|i| *i).collect();
    }

    pub fn getFontNodesIdentifiers(&self) -> Vec<u32>{
        let mut font_nodes: Vec<u32> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(*font);
        }   

        return font_nodes;
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

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_subs.insert(*id, node.subs.clone());
        }        

        return flattened_subs;
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_supers.insert(*id, node.supers.clone());
        }        

        return flattened_supers;
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

}
