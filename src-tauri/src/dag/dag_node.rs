use std::collections::HashSet;


pub struct DagNode{
    pub identifier: u32,
    pub supers: Vec<u32>,
    pub subs: Vec<u32>,

    // This pattern is overlapped by these ones, here only appear the patterns that overlaps AND 
    // have greater density.
    pub overlappings: HashSet<u32>, 
}

impl DagNode{
    pub fn new(identifier: &u32) -> DagNode{
        return DagNode { 
            identifier: *identifier,
            supers: Vec::new(), 
            subs: Vec::new(), 
            overlappings: HashSet::new() };
    }

    // pub fn addOverlappingPattern(&mut self, overlapper_id: &u32){
    //     if self.overlappings.contains(overlapper_id){ return; }
    //     self.overlappings.push(*overlapper_id);
    // }


}