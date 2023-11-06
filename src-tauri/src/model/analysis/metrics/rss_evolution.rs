use ndarray::{ArrayD, Dim, IxDynImpl};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::iter::Iterator;
use crate::common::progress_bar;
use crate::database::pattern::{Pattern, self};
use crate::model::analysis::ordered_pair::OrderedPair;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::metric::Metric;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct PredictionMatrix {
    value: HashMap<Dim<IxDynImpl>, f64>,
}

impl PredictionMatrix{
    fn new() -> PredictionMatrix{
        return PredictionMatrix{
            value: HashMap::new(),
        }
    }

    fn insert(&mut self, index: Dim<IxDynImpl>, value: f64){
        self.value.insert(index, value);
    }

    fn get(&self, index: &Dim<IxDynImpl>) -> Option<&f64>{
        return self.value.get(index);
    }

    fn get_mut(&mut self, index: &Dim<IxDynImpl>) -> Option<&mut f64>{
        return self.value.get_mut(index);
    }
}

#[derive(Clone)]
struct IntersectionsIndices{
    value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>,
}

impl IntersectionsIndices {
    fn new() -> IntersectionsIndices{
        return IntersectionsIndices{
            value: HashMap::new(),
        }
    }

    fn insert(&mut self, identifier: u32, intersections: HashMap<u32, Vec<Dim<IxDynImpl>>>){
        self.value.insert(identifier, intersections);
    }

    fn get(&self, identifier: &u32) -> Option<&HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get(identifier);
    }
}

#[derive(Clone)]
struct UntouchedRss{
    value: HashMap<u32, (u32, f64)>,
}

impl UntouchedRss{
    fn new() -> UntouchedRss{
        return UntouchedRss{
            value: HashMap::new(),
        }
    }

    fn insert(&mut self, identifier: u32, size_rss: (u32, f64)){
        self.value.insert(identifier, size_rss);
    }

    fn get(&self, identifier: &u32) -> Option<&(u32, f64)>{
        return self.value.get(identifier);
    }
}

pub struct RssEvolution{
    value: Vec<(u32, f64)>,
    truncated_value: Vec<(u32, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<Vec<(u32, f64)>> for RssEvolution{
    fn get(&self) -> &Vec<(u32, f64)> {
        return &self.value;
    }
}

impl RssEvolution{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, empty_model_rss: &EmptyModelRss, 
        patterns: &Vec<&Pattern>) -> RssEvolution{

        println!("  RssEvolution...");
        
        let rss_evolution = RssEvolution::calculate(identifier_mapper, tensor, empty_model_rss, patterns);
        return RssEvolution{
            value: rss_evolution.clone(),
            truncated_value: rss_evolution,
        }
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn updateRssAtIndex(tensor_matrix: &ArrayD<f64>, total_rss: &f64, index: &Dim<IxDynImpl>, old_prediction: &f64, 
                        new_prediction: &f64, prediction_matrix: &mut PredictionMatrix) -> f64{
        
        prediction_matrix.insert(index.clone(), *new_prediction);
        drop(prediction_matrix);

        let actual_value = tensor_matrix.get(index).unwrap();

        let new_prediction_rss = RssEvolution::calculateRss(actual_value, new_prediction);
        let old_prediction_rss = RssEvolution::calculateRss(actual_value, old_prediction);

        let total_rss = total_rss - old_prediction_rss + new_prediction_rss;
        
        return total_rss;
    }

    fn createControlStructures(tensor: &Tensor, patterns: &Vec<&Pattern>, identifier_mapper: &IdentifierMapper) 
            -> (PredictionMatrix, UntouchedRss, IntersectionsIndices) {
        let prediction_matrix = PredictionMatrix::new();
        let untouched_rss_s = UntouchedRss::new();
        let intersections_indices = IntersectionsIndices::new();
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();

        for pattern in patterns {
            let node = identifier_mapper.getRepresentation(&pattern.identifier).asDagNode();

            for other_pattern in patterns {
                let other_node = identifier_mapper.getRepresentation(&other_pattern.identifier).asDagNode();
                
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
        
        let prediction_matrix: Arc<Mutex<PredictionMatrix>> = Arc::new(Mutex::new(prediction_matrix));
        let untouched_rss_s: Arc<Mutex<UntouchedRss>> = Arc::new(Mutex::new(untouched_rss_s));
        let intersections_indices: Arc<Mutex<IntersectionsIndices>> = Arc::new(Mutex::new(intersections_indices));
        
        patterns.par_iter().for_each(|pattern| {
            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself

                let self_overlappings = overlappings.get(&pattern.identifier);
                if self_overlappings.is_none() { continue; } // This pattern doesnt overlap any other pattern
                if !self_overlappings.unwrap().contains(&other_pattern.identifier) { continue; } // These two do not overlap

                let intersection_indices: Vec<Dim<IxDynImpl>> = pattern.intersection(other_pattern)
                    .into_iter()
                    .map(|index| Dim(index))
                    .collect();

                for index in intersection_indices.iter() {
                    all_intersection_indices.insert(index.clone());
                    prediction_matrix.lock().unwrap().insert(index.clone(), tensor.density);
                }

                if !intersection_indices.is_empty() { // There are intersections
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                }
            }

            if !pattern_intersections.is_empty(){ // This pattern has intersections with other patterns
                intersections_indices.lock().unwrap().insert(pattern.identifier, pattern_intersections);
            }

            let prediction = &pattern.density;
            let mut untouched_size: u32 = 0;
            let untouched_rss: f64 = pattern.indices_as_dims.clone().into_iter()
                .filter(|index| !all_intersection_indices.contains(index))
                .map(|index| {
                    let actual_value = tensor.dims_values.get(&index).unwrap();

                    let prediction_rss = RssEvolution::calculateRss(actual_value, prediction);
                    let lambda_0_rss = RssEvolution::calculateRss(actual_value, &tensor.density);
                    let delta_rss = prediction_rss - lambda_0_rss;

                    untouched_size += 1;
                    delta_rss
                })
                .sum();

            untouched_rss_s.lock().unwrap().insert(pattern.identifier, (untouched_size, untouched_rss));
        });

        let prediction_matrix = prediction_matrix.lock().unwrap().clone();
        let untouched_rss_s = untouched_rss_s.lock().unwrap().clone();
        let intersections_indices = intersections_indices.lock().unwrap().clone();

        return (prediction_matrix, untouched_rss_s, intersections_indices);
    }

    fn updatePredictionMatrix(prediction_matrix: &mut PredictionMatrix, intersections_indices: &IntersectionsIndices,
                            candidate_pattern: &Pattern){
        
        let all_intersections_indices = intersections_indices.get(&candidate_pattern.identifier);

        if all_intersections_indices.is_none() { return; } // No intersection
        let all_intersections_indices = all_intersections_indices.unwrap();  
            
        for (_, intersection_indices) in all_intersections_indices {
            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.get_mut(intersection_index).unwrap();

                let max_prediction = f64::max(candidate_pattern.density, *previous_prediction);

                if max_prediction > *previous_prediction { // Then change to the new prediction
                    *previous_prediction = max_prediction;
                }
            }
        }
    }

    fn calculateCandidateModelRss(current_model_rss: &f64, candidate_pattern: &Pattern,
            tensor: &Tensor,
            identifier_mapper: &IdentifierMapper,
            untouched_delta_rss_s: &UntouchedRss,
            prediction_matrix: &mut PredictionMatrix,
            intersections_indices: &IntersectionsIndices,
            seen_candidates: &Vec<u32>) -> f64{

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss_s.get(&candidate_pattern.identifier).unwrap().1;
            
        let candidate_intersections = intersections_indices.get(&candidate_pattern.identifier);
        if candidate_intersections.is_none(){ return candidate_model_rss; } // No intersection
        
        // Here we can also have indices with no intersection
        let candidate_intersections = candidate_intersections.unwrap();
        let candidate_prediction = candidate_pattern.density;

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let ignore_intersector = !seen_candidates.contains(intersector);
            
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)
                .asPattern().density;

            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.get(intersection_index).unwrap();
                let previous_prediction_copy = previous_prediction.clone();

                if ignore_intersector == true { // Intersector is not in the submodel, act as if the candidate is not intersected
                    candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                        &candidate_model_rss, 
                        intersection_index, 
                        &previous_prediction_copy, 
                        &candidate_prediction,
                        prediction_matrix);
                    
                    continue;
                }

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction,
                    prediction_matrix);
            }
        }
        return candidate_model_rss;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>) 
        -> Vec<(u32, f64)>{
        
        let pattern_nb = patterns.len();

        let (
            mut prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices) = 
                RssEvolution::createControlStructures(tensor, &patterns, identifier_mapper);

        let mut current_model_rss = *empty_model_rss.get();
        let mut rss_evolution: Vec<(u32, f64)> = vec![(0, current_model_rss)];
        let mut seen_candidates: Vec<u32> = vec![];
        
        let bar = progress_bar::new(pattern_nb as u64, "    Submodels calculated");
        for (_, pattern) in patterns.iter().enumerate(){

            let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
                &current_model_rss, 
                pattern, 
                tensor, 
                identifier_mapper, 
                &untouched_delta_rss, 
                &mut prediction_matrix,
                &intersections_indices, 
                &seen_candidates);

            current_model_rss = candidate_model_rss;
            rss_evolution.push((pattern.identifier, current_model_rss));
            seen_candidates.push(pattern.identifier);
            RssEvolution::updatePredictionMatrix(&mut prediction_matrix, &intersections_indices, pattern);
            bar.inc(1);
        }

        bar.finish();
        return rss_evolution;
    }

    pub fn truncate(&mut self, new_size: &u32){
        let full_rss_evolution: Vec<(u32, f64)> = self.value.clone();
        
        // retain the first k + 1 elements, where k is the new size
        let truncated_rss_evolution: Vec<(u32, f64)> = full_rss_evolution.into_iter()
            .take(*new_size as usize + 1)
            .map(|(pattern_identifier, rss)| (pattern_identifier, rss))
            .collect();

        self.truncated_value = truncated_rss_evolution;
    }

    pub fn getTruncated(&self) -> &Vec<(u32, f64)>{
        return &self.truncated_value;
    }
}