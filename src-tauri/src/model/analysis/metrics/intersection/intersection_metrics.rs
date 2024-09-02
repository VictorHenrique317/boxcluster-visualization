use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use ndarray::{Dim, IxDynImpl};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{common::generic_error::GenericError, database::{pattern::Pattern, tensor::Tensor}, model::identifier_mapper::IdentifierMapper};

use super::{intersections_percentages::IntersectionsPercentages, intersections_indices::IntersectionsIndices, prediction_matrix::PredictionMatrix, untouched_delta_rss::UntouchedDeltaRss};

pub struct IntersectionMetrics {}

impl IntersectionMetrics{
    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    pub fn calculate(tensor: &Tensor, patterns: &Vec<&Pattern>, identifier_mapper: &IdentifierMapper) 
            -> Result<(PredictionMatrix, UntouchedDeltaRss, IntersectionsIndices, IntersectionsPercentages), GenericError>{

        let prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();
        let untouched_rss_s: HashMap<u32, (u32, f64)>= HashMap::new();
        let intersections_indices: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();
        let intersections_percentages: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new(); // This is a symmetric relation

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
        let intersections_percentages: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(intersections_percentages));

        patterns.par_iter().try_for_each(|pattern| -> Result<(), GenericError> {

            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
            // let MAX_PATTERN_INTERSECTIONS = 6;
            let mut pattern_intersections_percentages: HashMap<u32, f64> = HashMap::new();
            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            let self_overlappings = overlappings.get(&pattern.identifier);

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself
                
                match self_overlappings {
                    None => continue, // This pattern doesnt overlap any other pattern
                    Some(self_overlappings) => {
                        if !self_overlappings.contains(&other_pattern.identifier) { continue; } // These two do not overlap
                    },
                };

                // Here we know that pattern and other_pattern overlap

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

                if !intersection_indices.is_empty() { // There are intersections between pattern and other_pattern
                    let intersection_percentage = intersection_indices.len() as f64 / other_pattern.size as f64;
                    
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                    pattern_intersections_percentages.insert(other_pattern.identifier, intersection_percentage);
                }else{
                    unreachable!("There should be at least one intersection");
                }
            }

            if !pattern_intersections.is_empty(){ // This pattern has intersections with other patterns
                intersections_indices.lock()
                    .as_mut()
                    .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
                    .insert(pattern.identifier, pattern_intersections);
            }

            // if pattern_intersections_percentages.len() > MAX_PATTERN_INTERSECTIONS{
            //     // This truncates pattern_intersections_percentages up to (MAX_PATTERN_INTERSECTIONS - 1) elements
            //     // and inserts the sum of the excess elements on it after
            //     let mut sorted_pattern_intersections_indices: Vec<u32> = pattern_intersections_percentages.keys().cloned().collect();
            //     sorted_pattern_intersections_indices.sort_by(|a, b| { // Decreasing order, based on the intersection percentage
            //         pattern_intersections_percentages.get(b).partial_cmp(&pattern_intersections_percentages.get(a)).unwrap()});

            //     let excess_indices = sorted_pattern_intersections_indices.split_off(MAX_PATTERN_INTERSECTIONS - 1);
            //     let excess_percentages_sum = excess_indices.iter()
            //         .map(|index| pattern_intersections_percentages.get(index).unwrap())
            //         .sum::<f64>();

            //     // Retain in pattern_intersections_percentages only the entries in which the key is in sorted_pattern_intersections_indices
            //     pattern_intersections_percentages.retain(|key, _| sorted_pattern_intersections_indices.contains(key));
            //     pattern_intersections_percentages.insert(0, excess_percentages_sum);
            // }
            
            let total_intersection_percentage = all_intersection_indices.len() as f64 / pattern.size as f64;
            let untouched_percentage = 1.0 - total_intersection_percentage;
            if untouched_percentage < 0.0 || untouched_percentage > 1.0 {
                unreachable!("Untouched percentage should be between 0 and 1 but it is: {}", untouched_percentage);
            }
            pattern_intersections_percentages.insert(pattern.identifier, total_intersection_percentage);
            
            intersections_percentages.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
                .insert(pattern.identifier, pattern_intersections_percentages);

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

        let pattern_intersections_percentages = IntersectionsPercentages::new(intersections_percentages.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
            .clone());

        return Ok((prediction_matrix, untouched_rss_s, intersections_indices, pattern_intersections_percentages));
    }
}