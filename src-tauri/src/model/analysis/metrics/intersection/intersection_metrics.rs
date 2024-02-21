use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use ndarray::{Dim, IxDynImpl};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{common::generic_error::GenericError, database::{pattern::Pattern, tensor::Tensor}, model::identifier_mapper::IdentifierMapper};

use super::{intersections_indices::IntersectionsIndices, prediction_matrix::PredictionMatrix, untouched_delta_rss::UntouchedDeltaRss};

pub struct IntersectionMetrics {}

impl IntersectionMetrics{
    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    pub fn calculate(tensor: &Tensor, identifier_mapper: &IdentifierMapper) 
            -> Result<(PredictionMatrix, UntouchedDeltaRss, IntersectionsIndices), GenericError>{

        let prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();
        let untouched_rss_s: HashMap<u32, (u32, f64)>= HashMap::new();
        let intersections_indices: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();

        let patterns: &Vec<&Pattern> = &identifier_mapper.getOrderedPatterns();

        for pattern in patterns {
            let node = identifier_mapper.getRepresentation(&pattern.identifier)?.asDagNode()?;

            for other_pattern in patterns {
                let other_node = identifier_mapper.getRepresentation(&other_pattern.identifier)?.asDagNode()?;
                
                if node.overlappings.contains(&other_node.identifier) || other_node.overlappings.contains(&node.identifier) {
                    overlappings.entry(pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(other_pattern.identifier);

                    overlappings.entry(other_pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(pattern.identifier);
                }
            }
        }

        let prediction_matrix: Arc<Mutex<HashMap<Dim<IxDynImpl>, f64>>> = Arc::new(Mutex::new(prediction_matrix));
        let untouched_rss_s: Arc<Mutex<HashMap<u32, (u32, f64)>>> = Arc::new(Mutex::new(untouched_rss_s));
        let intersections_indices: Arc<Mutex<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>>> = Arc::new(Mutex::new(intersections_indices));
        
        patterns.par_iter().try_for_each(|pattern| -> Result<(), GenericError> {

            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself

                let self_overlappings = overlappings.get(&pattern.identifier);
                match self_overlappings {
                    None => continue, // This pattern doesnt overlap any other pattern
                    Some(self_overlappings) => {
                        if !self_overlappings.contains(&other_pattern.identifier) { continue; } // These two do not overlap
                    },
                };

                let intersection_indices: Vec<Dim<IxDynImpl>> = pattern.intersection(other_pattern)
                    .into_iter()
                    .map(|index| Dim(index))
                    .collect();

                for index in intersection_indices.iter() {
                    all_intersection_indices.insert(index.clone());
                    prediction_matrix.lock()
                        .as_mut()
                        .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
                        .insert(index.clone(), tensor.density);
                }

                if !intersection_indices.is_empty() { // There are intersections
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                }
            }

            if !pattern_intersections.is_empty(){ // This pattern has intersections with other patterns
                intersections_indices.lock()
                    .as_mut()
                    .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
                    .insert(pattern.identifier, pattern_intersections);
            }

            let prediction = &pattern.density;
            let mut untouched_size: u32 = 0;
            let untouched_rss: f64 = pattern.indices_as_dims.clone().into_iter()
                .filter(|index| !all_intersection_indices.contains(index))
                .map(|index| -> Result<f64, GenericError> {
                    let actual_value = tensor.dims_values.get(&index)
                        .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

                    let prediction_rss = IntersectionMetrics::calculateRss(actual_value, prediction);
                    let lambda_0_rss = IntersectionMetrics::calculateRss(actual_value, &tensor.density);
                    let delta_rss = prediction_rss - lambda_0_rss;

                    untouched_size += 1;
                    Ok(delta_rss)
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();

            untouched_rss_s.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
                .insert(pattern.identifier, (untouched_size, untouched_rss));

            return Ok(());
        })?;

        let prediction_matrix = PredictionMatrix::new(prediction_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
            .clone());

        let untouched_rss_s = UntouchedDeltaRss::new(untouched_rss_s.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
            .clone());

        let intersections_indices = IntersectionsIndices::new(intersections_indices.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
            .clone());

        return Ok((prediction_matrix, untouched_rss_s, intersections_indices));
    }
}