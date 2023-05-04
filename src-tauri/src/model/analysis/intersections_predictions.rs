#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::{database::pattern::Pattern, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}};

pub struct IntersectionsPredictions<'a>{
    value: HashMap<Vec<usize>, &'a Pattern>,
}

#[allow(non_camel_case_types)]
impl<'a> Metric<HashMap<Vec<usize>, &'a Pattern>> for IntersectionsPredictions<'a> {
    fn get(&self) -> &HashMap<Vec<usize>, &'a Pattern> {
        return &self.value;
    }
}

impl IntersectionsPredictions<'_>{
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> IntersectionsPredictions<'a>{
        println!("  Intersections predictions...");
        return IntersectionsPredictions { 
            value: IntersectionsPredictions::calculate(identifier_mapper),
        }
    }
    
    fn calculate<'a>(identifier_mapper: &'a IdentifierMapper) -> HashMap<Vec<usize>, &'a Pattern> {
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();
        for identifier_representation in identifier_mapper.getRepresentations(){
            let node = identifier_representation.asDagNode();
            overlappings.insert(node.identifier, node.overlappings.clone());
        }

        let intersections_predictions: Arc<Mutex<HashMap<Vec<usize>, &Pattern>>> = Arc::new(Mutex::new(HashMap::new()));

        overlappings.par_iter().for_each(|(overlapped, overlappers)| {
            let overlapped: &Pattern = identifier_mapper.getRepresentation(overlapped).asPattern();

            for overlapper in overlappers{
                let overlapper: &Pattern = identifier_mapper.getRepresentation(overlapper).asPattern();
                let intersection_indices: Vec<Vec<usize>> = overlapped.intersection(overlapper);

                for intersection_index in intersection_indices {
                    let mut intersections_predictions_lock = intersections_predictions.lock().unwrap();

                    let possible_previous_prediction = intersections_predictions_lock.get(&intersection_index); // EXPENSIVE
                    if possible_previous_prediction.is_some(){ // Multiple overlapping in one index
                        let previous_prediction = possible_previous_prediction.unwrap().density;

                        if overlapper.density < previous_prediction{ // Do not switch to current overlapper
                            continue;
                        }
                    }

                    intersections_predictions_lock.insert(intersection_index.clone(), overlapper);
                }
            }
        });
            
        return intersections_predictions.lock().unwrap().clone();    
    }
}