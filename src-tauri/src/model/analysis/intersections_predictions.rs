#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::{database::pattern::Pattern, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}, common::generic_error::GenericError};

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
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<IntersectionsPredictions<'a>, GenericError>{
        println!("  Intersections predictions...");
        return Ok(
            IntersectionsPredictions { 
                value: IntersectionsPredictions::calculate(identifier_mapper)?,
            }
        );
    }
    
    fn calculate<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<HashMap<Vec<usize>, &'a Pattern>, GenericError> {

        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();
        for identifier_representation in identifier_mapper.getRepresentations(){
            let node = identifier_representation.asDagNode()?;
            overlappings.insert(node.identifier, node.overlappings.clone());
        }

        let intersections_predictions: Arc<Mutex<HashMap<Vec<usize>, &Pattern>>> = Arc::new(Mutex::new(HashMap::new()));

        overlappings.par_iter().try_for_each(|(overlapped, overlappers)| -> Result<(), GenericError>{
            let overlapped: &Pattern = identifier_mapper.getRepresentation(overlapped)?.asPattern()?;

            for overlapper in overlappers{
                let overlapper: &Pattern = identifier_mapper.getRepresentation(overlapper)?.asPattern()?;
                let intersection_indices: Vec<Vec<usize>> = overlapped.intersection(overlapper);

                for intersection_index in intersection_indices {
                    let mut intersections_predictions_lock = intersections_predictions
                        .lock()
                        .map_err(|_| GenericError::new("Could not lock intersections predictions"))?;

                    let possible_previous_prediction = intersections_predictions_lock.get_mut(&intersection_index); // EXPENSIVE
                    match possible_previous_prediction{
                        None => {}
                        Some(previous_prediction) => { // Multiple overlapping in one index

                            if overlapper.density > previous_prediction.density{ // Switch to current overlapper
                                *previous_prediction = overlapper;
                            }

                            continue;
                        }
                    };

                    intersections_predictions_lock.insert(intersection_index.clone(), overlapper);
                }
            }

            return Ok(());
        })?;
            
        return Ok(
            intersections_predictions.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections predictions"))?
            .clone()
        );
    }
}