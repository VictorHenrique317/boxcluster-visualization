#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::database::dag_node::DagNode;
use debug_print::debug_println;

pub (in crate::services::dag) struct DagArrangerService{
    fonts: Vec<u32>,
    nodes: HashMap<u32, DagNode>,
}

impl DagArrangerService{
    pub fn new() -> DagArrangerService{
        return DagArrangerService {
            fonts: Vec::new(),
            nodes: HashMap::new(),
        };
    }

    pub fn init(&mut self, nodes: Vec<DagNode>){
        let nodes: HashMap<u32, DagNode> = nodes.into_iter()
            .map(|node| (node.identifier, node))
            .collect();

        self.nodes = nodes;
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

    pub fn addOverlappingNode(&mut self, overlapped_node: &u32, overlapping_node: &u32){
        let overlapped_node = self.nodes.get_mut(overlapped_node).unwrap();
        overlapped_node.overlappings.insert(*overlapping_node);
    }

    pub fn addBellow(&mut self, adding_node: &u32, parent: &u32){
        let adding_node_p = self.nodes.get_mut(&adding_node).unwrap();
        adding_node_p.supers.push(*parent);

        let parent_p = self.nodes.get_mut(&parent).unwrap();
        parent_p.subs.push(*adding_node);
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

    pub fn traverse(&self, to_node: &u32) -> &Vec<u32>{
        return &self.nodes.get(to_node).unwrap().subs;
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

    pub fn getNode(&self, identifier: &u32) -> &DagNode{
        return self.nodes.get(identifier).unwrap();
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        let mut nodes: Vec<u32> = Vec::new();
        for node in self.nodes.values(){
            nodes.push(node.identifier);
        }
        return nodes;
    }

    pub fn getFontNodes(&self) -> Vec<&DagNode> {
        let mut font_nodes: Vec<&DagNode> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(self.getNode(font));
        }   

        return font_nodes;
    }

    pub fn getFontNodesIdentifiers(&self) -> Vec<u32>{
        let mut font_nodes: Vec<u32> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(*font);
        }   

        return font_nodes;
    }

    pub fn finish(self) -> Vec<DagNode> {
        return self.nodes.into_iter()
            .map(|(_, node)| node)
            .collect();
    }
}