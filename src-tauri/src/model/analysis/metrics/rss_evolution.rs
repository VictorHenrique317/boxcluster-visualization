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

        // dbg!(&untouched_pattern_indices);
        // dbg!(&intersections);

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

            // let mut untouched_delta_rss = 0.0;
            // let mut untouched_size = 0;

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

            // untouched_delta_rss_s.insert(identifier, (untouched_size, untouched_delta_rss));
        }

        dbg!(&untouched_delta_rss_s);
        dbg!(&intersections);

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

    fn calculateIntersections(patterns: &Vec<&Pattern>) -> HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> {
        let mut intersections: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();

        for pattern in patterns {
            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; }

                let intersection_indices = pattern.intersection(other_pattern);
                if intersection_indices.len() == 0 { continue; }
                let intersection_indices: Vec<Dim<IxDynImpl>> = intersection_indices.into_iter()
                    .map(|index| Dim(index))
                    .collect();

                pattern_intersections.insert(other_pattern.identifier, intersection_indices);
            }

            intersections.insert(pattern.identifier, pattern_intersections);
        }

        return intersections;
    }

    // fn calculateRssAtIntersections(total_rss: &mut f64, pattern: &Pattern, 
    //     prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, 
    //     mut accounted_indices: HashSet<Dim<IxDynImpl>>,
    //     tensor_matrix: &ArrayD<f64>,
    //     lambda_0: &f64,
    //     patterns_identifiers: &Vec<u32>) -> HashSet<Dim<IxDynImpl>> {

    //     for index in pattern.indices_as_dims.iter() {
    //         let predictions = prediction_matrix.get(&index);
    //         if predictions.is_none() { continue; } // This index is covered by one pattern
    //         let predictions = predictions.unwrap();

    //         // There are more than one prediction, retain the ones that are in this submodel
    //         // and keep the maximum value

    //         if accounted_indices.contains(index) { continue; } // This index has already been accounted for
    //         accounted_indices.insert(index.clone());

    //         let mut max_prediction = f64::MIN;
    //         for (identifier, prediction) in predictions {
    //             if patterns_identifiers.contains(identifier) && prediction > &mut max_prediction {
    //                 max_prediction = *prediction;
    //             }
    //         }
    //         *total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, lambda_0, index, &max_prediction);
    //     }

    //     return accounted_indices;
    // }



    // fn calculateModelRss(tensor: &Tensor, empty_model_rss: &EmptyModelRss, previous_patterns: &Vec<&Pattern>, 
    //     new_pattern: &Pattern, prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, 
    //     untouched_delta_rss_s: &HashMap<u32, (u32, f64)>) -> f64{
        
    //     let mut total_rss = *empty_model_rss.get();
    //     let tensor_matrix: &ArrayD<f64> = &tensor.dims_values;

    //     let mut patterns_identifiers: Vec<u32> = previous_patterns.iter()
    //         .map(|p| p.identifier)
    //         .collect();
    //     patterns_identifiers.push(new_pattern.identifier);

    //     let mut accounted_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

    //     // println!("{:?}", previous_patterns.len());

    //     for pattern in previous_patterns {
    //         let untouched_rss = untouched_delta_rss_s.get(&pattern.identifier);
    //         if untouched_rss.is_some() {
    //             let untouched_rss = untouched_rss.unwrap();
    //             if pattern.size == untouched_rss.0 {  // All indices of this pattern are touched
    //                 continue;
    //             }
    //         }

    //         // let patterns_rss_s = touched_delta_rss_s.get(&pattern.identifier);

    //         // for (pattern, delta_rss) in patterns_rss_s.unwrap() {
    //         //     let mut maximum_de
    //         //     if patterns_identifiers.contains(pattern) { // This pattern is in the submodel
                    
    //         //     } 
    //         // }
            
    //         accounted_indices = RssEvolution::calculateRssAtIntersections(&mut total_rss, pattern, prediction_matrix, accounted_indices, 
    //             tensor_matrix, &tensor.density, &patterns_identifiers);
    //     }

    //     let untouched_rss = untouched_delta_rss_s.get(&new_pattern.identifier);
    //     if untouched_rss.is_some() {
    //         let untouched_rss = untouched_rss.unwrap();
    //         if new_pattern.size != untouched_rss.0 {
    //             accounted_indices = RssEvolution::calculateRssAtIntersections(&mut total_rss, new_pattern, prediction_matrix, accounted_indices, 
    //                 tensor_matrix, &tensor.density, &patterns_identifiers);
    //         }
    //     }

    //     // println!("Ended");

    //     for pattern in patterns_identifiers {
    //         let untouched_rss = untouched_delta_rss_s.get(&pattern).unwrap();
    //         total_rss += untouched_rss.1;
    //     }

        
        
    //     return total_rss;
    // }

    fn updatePredictionMatrix(prediction_matrix: &mut HashMap<Dim<IxDynImpl>, f64>, 
            intersections: &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>, selected_pattern: &Pattern){
        
        let all_intersections_indices = intersections.get(&selected_pattern.identifier);

        if all_intersections_indices.is_none() { return; } // No intersection
        let all_intersections_indices = all_intersections_indices.unwrap();  
            
        for (pattern, intersection_indices) in all_intersections_indices {
            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.get_mut(intersection_index).unwrap();

                let max_prediction = f64::max(selected_pattern.density, *previous_prediction);

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
            intersections: &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>) -> f64{

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss_s.get(&candidate_pattern.identifier).unwrap().1;
            
        let candidate_intersections = intersections.get(&candidate_pattern.identifier);
        if candidate_intersections.is_none(){ return candidate_model_rss; } // No intersection 
        
        dbg!(&current_model_rss);
        dbg!(&candidate_model_rss);
        dbg!(&candidate_pattern.identifier);
        // dbg!(&candidate_intersections);
        

        // Here we can also have indices with no intersection
        let candidate_intersections = candidate_intersections.unwrap();
        let candidate_prediction = candidate_pattern.density;
        // let mut all_intersection_indices = HashSet::new();
        // let mut candidate_model_rss = *current_model_rss;

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)
                .asPattern().density;

            for intersection_index in intersection_indices {
                // all_intersection_indices.insert(intersection_index);

                let previous_prediction = prediction_matrix.get(intersection_index).unwrap();
                let previous_prediction_copy = previous_prediction.clone();

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                dbg!(*previous_prediction);
                dbg!(&intersector_prediction);
                dbg!(&candidate_prediction);
                dbg!(max_prediction);


                // if max_prediction > *previous_prediction { // Then change to the new prediction
                //     *previous_prediction = max_prediction;
                // }

                dbg!(&candidate_model_rss);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction);

                dbg!(&candidate_model_rss);
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

        return candidate_model_rss;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, 
        intersections_predictions: &IntersectionsPredictions) -> Vec<(u32, f64)>{
        // 100 => 5s, 3s, 1s
        // all => ??, 
    
        let mut patterns: Vec<&Pattern> = identifier_mapper.getRepresentations().iter()
                .map(|r| r.asPattern())
                .collect();
        let pattern_nb = patterns.len();

        // Control structures
        let (untouched_delta_rss_s, 
            intersections) = RssEvolution::analysePatternsRelations(&patterns, tensor, identifier_mapper);
        
        let prediction_matrix = RssEvolution::initializePredictionMatrix(&intersections, tensor);
        dbg!(&prediction_matrix);
        let prediction_matrix = Arc::new(Mutex::new(prediction_matrix));
        // Control structures
        
        let mut minimum_rss_value = *empty_model_rss.get();
        let mut rss_evolution: Vec<f64> = vec![minimum_rss_value];
        let mut sorted_patterns: Vec<&Pattern> = Vec::new();
        
        let bar = progress_bar::new(pattern_nb as u64, "  Orderered patterns");
        while sorted_patterns.len() < pattern_nb { // Sorts all patterns in the patterns vector
            // println!("      Sorting one...");
            let minimum_temp_model_rss = Arc::new(Mutex::new(f64::MAX));
            let minimum_temp_model_pattern: Arc<Mutex<Option<&Pattern>>> = Arc::new(Mutex::new(None));
            let minimum_temp_model_pattern_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));

            // patterns.par_iter().enumerate().for_each(|(index, pattern)|{

            //     let mut prediction_matrix = prediction_matrix.lock().unwrap();

            //     let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
            //         &minimum_rss_value, 
            //         pattern, tensor, 
            //         identifier_mapper, 
            //         &untouched_delta_rss_s, 
            //         &mut prediction_matrix, 
            //         &intersections);

            //     drop(prediction_matrix);

            //     let mut minimum_temp_model_rss = minimum_temp_model_rss.lock().unwrap();
            //     if candidate_model_rss <= *minimum_temp_model_rss {
            //         let mut minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
            //         let mut minimum_temp_model_pattern_index = minimum_temp_model_pattern_index.lock().unwrap();
                    
            //         *minimum_temp_model_rss = candidate_model_rss;
            //         *minimum_temp_model_pattern = Some(pattern);
            //         *minimum_temp_model_pattern_index = index;
            //     }
            // });
            
            let mut prediction_matrix = prediction_matrix.lock().unwrap();

            for (index, pattern) in patterns.iter().enumerate(){
                let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
                    &minimum_rss_value, 
                    pattern, tensor, 
                    identifier_mapper, 
                    &untouched_delta_rss_s, 
                    &prediction_matrix, 
                    &intersections);


                let mut minimum_temp_model_rss = minimum_temp_model_rss.lock().unwrap();
                if candidate_model_rss <= *minimum_temp_model_rss {
                    let mut minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
                    let mut minimum_temp_model_pattern_index = minimum_temp_model_pattern_index.lock().unwrap();
                    
                    *minimum_temp_model_rss = candidate_model_rss;
                    *minimum_temp_model_pattern = Some(pattern);
                    *minimum_temp_model_pattern_index = index;
                }
            }

            let minimum_temp_model_rss = *minimum_temp_model_rss.lock().unwrap();
            let minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
            let minimum_temp_model_pattern_index = *minimum_temp_model_pattern_index.lock().unwrap();

            if minimum_temp_model_rss <= minimum_rss_value { // Its worth to add a pattern, add it
                let selected_pattern = minimum_temp_model_pattern.unwrap();

                minimum_rss_value = minimum_temp_model_rss;
                sorted_patterns.push(selected_pattern);
                rss_evolution.push(minimum_rss_value);

                patterns.remove(minimum_temp_model_pattern_index);
                RssEvolution::updatePredictionMatrix(&mut prediction_matrix, &intersections, selected_pattern);


            }else{ break; }// No pattern decreases the rss

            bar.inc(1);
        }

        bar.finish();

        let mut patterns_with_rss: Vec<(u32, f64)> = vec![(0, *empty_model_rss.get())];
        for (i, pattern) in sorted_patterns.iter().enumerate() {
            patterns_with_rss.push((pattern.identifier, rss_evolution[i + 1]));
        }

        return patterns_with_rss;
        
    }

}