use ndarray::{ArrayD, Dim, IxDynImpl, Dimension};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};

use crate::common::progress_bar;
use crate::database::pattern::{Pattern, self};
use crate::model::analysis::intersections_predictions::IntersectionsPredictions;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::metric::Metric;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct RssEvolution{
    value: Vec<(u32, f64)>, 
}

#[allow(non_camel_case_types)]
impl Metric<Vec<(u32, f64)>> for RssEvolution{
    fn get(&self) -> &Vec<(u32, f64)> {
        return &self.value;
    }
}

impl RssEvolution{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, empty_model_rss: &EmptyModelRss,
        intersections_predictions: &IntersectionsPredictions) -> RssEvolution{

        println!("  RssEvolution...");

        return RssEvolution{
            value: RssEvolution::calculate(identifier_mapper, tensor, empty_model_rss, intersections_predictions),
        }
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn updateRssAtIndex(tensor_matrix: &ArrayD<f64>, total_rss: &f64, index: &Dim<IxDynImpl>, old_prediction: &f64, new_prediction: &f64) -> f64{
        let actual_value = tensor_matrix.get(index).unwrap();

        let new_prediction_rss = RssEvolution::calculateRss(actual_value, new_prediction);
        let old_prediction_rss = RssEvolution::calculateRss(actual_value, old_prediction);

        let total_rss = total_rss - old_prediction_rss + new_prediction_rss;
        return total_rss;
    }

    fn getUntouchedAndIntersections(patterns: &Vec<&Pattern>) 
        -> (HashMap<u32, Vec<Dim<IxDynImpl>>>, HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>){
        
        let mut untouched_pattern_indices: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
        let mut intersections: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();

        for pattern in patterns {
            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();

            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself

                let intersection_indices = pattern.intersection(other_pattern);
                let intersection_indices: Vec<Dim<IxDynImpl>> = intersection_indices.into_iter()
                    .map(|index| Dim(index))
                    .collect();

                for index in intersection_indices.iter() { // Add intersections to all intersection indices
                    all_intersection_indices.insert(index.clone());
                }

                if !intersection_indices.is_empty() { // There are intersections
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                }
            }

            let untouched_indices: Vec<Dim<IxDynImpl>> = pattern.indices_as_dims.clone().into_iter()
                .filter(|index| !all_intersection_indices.contains(index))
                .collect();
            untouched_pattern_indices.insert(pattern.identifier, untouched_indices);

            if !pattern_intersections.is_empty(){ // This pattern has intersections
                intersections.insert(pattern.identifier, pattern_intersections);
            }
        }

        return (untouched_pattern_indices, intersections);
    }

    fn analysePatternsRelations(patterns: &Vec<&Pattern>, tensor: &Tensor, identifier_mapper: &IdentifierMapper) 
        -> (HashMap<u32, (u32, f64)>, HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>) {
        
        let (untouched_pattern_indices, 
            intersections) = RssEvolution::getUntouchedAndIntersections(patterns);

        let mut untouched_delta_rss_s: HashMap<u32, (u32, f64)> = HashMap::new();
        for (identifier, untouched_indices) in untouched_pattern_indices {
            let pattern_density = identifier_mapper.getRepresentation(&identifier)
                    .asPattern().density;

            for index in untouched_indices {
                let actual_value = tensor.dims_values.get(&index).unwrap();

                let prediction_rss = RssEvolution::calculateRss(actual_value, &pattern_density);
                let lambda_0_rss = RssEvolution::calculateRss(actual_value, &tensor.density);
                let delta_rss = prediction_rss - lambda_0_rss;

                let untouched_rss = untouched_delta_rss_s.get_mut(&identifier);

                match untouched_rss{
                    None => {
                        untouched_delta_rss_s.insert(identifier, (1, delta_rss));
                    }

                    Some(size_rss) => {
                        let size = size_rss.0 + 1;
                        let rss = size_rss.1 + delta_rss;

                        *size_rss = (size, rss);
                    }
                }
            }
        }

        return (untouched_delta_rss_s, intersections);
    }

    fn initializePredictionMatrix(intersections: &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>, tensor: &Tensor) 
            -> HashMap<Dim<IxDynImpl>, f64> {
        let mut prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();
        let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

        for (_, patterns_intersections_indices) in intersections {
            for (_, intersection_indices) in patterns_intersections_indices {
                for index in intersection_indices {
                    all_intersection_indices.insert(index.clone());
                }
            }
        }

        for index in all_intersection_indices {
            prediction_matrix.insert(index, tensor.density);
        }

        return prediction_matrix;
    }

    fn updatePredictionMatrix(prediction_matrix: &mut HashMap<Dim<IxDynImpl>, f64>, 
            intersections: &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>, candidate_pattern: &Pattern){
        
        let all_intersections_indices = intersections.get(&candidate_pattern.identifier);

        if all_intersections_indices.is_none() { return; } // No intersection
        let all_intersections_indices = all_intersections_indices.unwrap();  
            
        for (pattern, intersection_indices) in all_intersections_indices {
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
            untouched_delta_rss_s: &HashMap<u32, (u32, f64)>,
            prediction_matrix: &HashMap<Dim<IxDynImpl>, f64>,
            intersections: &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>,
            seen_candidates: &Vec<u32>) -> f64{

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss_s.get(&candidate_pattern.identifier).unwrap().1;
            
        let candidate_intersections = intersections.get(&candidate_pattern.identifier);
        if candidate_intersections.is_none(){ return candidate_model_rss; } // No intersection
        
        // Here we can also have indices with no intersection
        let candidate_intersections = candidate_intersections.unwrap();
        let candidate_prediction = candidate_pattern.density;
        let mut seen_candidate_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();
        // let mut all_intersection_indices = HashSet::new();
        // let mut candidate_model_rss = *current_model_rss;

        // dbg!(candidate_pattern.identifier);

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let ignore_intersector = !seen_candidates.contains(intersector);
            
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)
                .asPattern().density;

            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.get(intersection_index).unwrap();
                let previous_prediction_copy = previous_prediction.clone();

                // dbg!(*previous_prediction);

                TODO: Predicition matrix has to be changed after each updatRssAtIndex call
                
                if ignore_intersector == true { // Intersector is not in the submodel, act as if the candidate is not intersected
                    // dbg!(&candidate_prediction);
                    candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                        &candidate_model_rss, 
                        intersection_index, 
                        &previous_prediction_copy, 
                        &candidate_prediction);
                    
                    continue;
                }

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction);

                // dbg!(&candidate_model_rss);
            }

            

        }
    
        // // For last deal with the untouched indices
        // let untouched_indices: Vec<Dim<IxDynImpl>> = candidate_pattern.indices_as_dims.clone();
        // let untouched_indices: Vec<&Dim<IxDynImpl>> = untouched_indices.iter()
        //         .filter(|index| !all_intersection_indices.contains(index))
        //         .collect();

        // dbg!(untouched_indices.len());

        // let old_prediction = tensor.density;
        // let new_prediction = candidate_pattern.density;
        // for untouched_index in untouched_indices {
        //     candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
        //         &candidate_model_rss, 
        //         untouched_index, 
        //         &old_prediction, 
        //         &new_prediction);
        // }
        
        // dbg!(&candidate_model_rss);
        // dbg!("===========================================");
        return candidate_model_rss;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, 
        intersections_predictions: &IntersectionsPredictions) -> Vec<(u32, f64)>{
    
        let mut patterns: Vec<&Pattern> = identifier_mapper.getOrderedRepresentations().iter()
                .map(|r| r.asPattern())
                .collect();
        
        let pattern_nb = patterns.len();

        // Control structures
        let (untouched_delta_rss_s, 
            intersections) = RssEvolution::analysePatternsRelations(&patterns, tensor, identifier_mapper);
        
        let mut prediction_matrix = RssEvolution::initializePredictionMatrix(&intersections, tensor);
        // Control structures
        
        let mut current_model_rss = *empty_model_rss.get();
        // dbg!(&empty_model_rss.get());
        let mut rss_evolution: Vec<(u32, f64)> = vec![(0, current_model_rss)];
        let mut seen_candidates: Vec<u32> = vec![];
        
        let bar = progress_bar::new(pattern_nb as u64, "  Submodels calculated...");
        for (_, pattern) in patterns.iter().enumerate(){

            let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
                &current_model_rss, 
                pattern, 
                tensor, 
                identifier_mapper, 
                &untouched_delta_rss_s, 
                &prediction_matrix, 
                &intersections,
                &seen_candidates);

            current_model_rss = candidate_model_rss;
            rss_evolution.push((pattern.identifier, current_model_rss));
            seen_candidates.push(pattern.identifier);
            RssEvolution::updatePredictionMatrix(&mut prediction_matrix, &intersections, pattern);
            bar.inc(1);
        }

        bar.finish();
        return rss_evolution;
        
    }

}