use ndarray::{ArrayD, Dim, IxDynImpl, Dimension};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};

use crate::common::progress_bar;
use crate::database::pattern::{Pattern, self};
use crate::model::analysis::intersections_predictions::IntersectionsPredictions;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::metric::Metric;
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

    fn updateRssAtIndex(total_rss: &f64, tensor_matrix: &ArrayD<f64>, lambda_0: &f64, index: &Dim<IxDynImpl>, prediction: &f64) -> f64{
        let actual_value = tensor_matrix.get(index).unwrap();

        let rss = RssEvolution::calculateRss(actual_value, prediction);
        let lambda0_rss = RssEvolution::calculateRss(actual_value, lambda_0);

        let total_rss = total_rss - lambda0_rss + rss;
        return total_rss;
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn calculatePredictionMatrix(patterns: &Vec<&Pattern>) -> HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>> {
        let mut prediction_matrix: HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>> = HashMap::new();
        for pattern in patterns {

            let current_prediction = &pattern.density;
            for index in pattern.indices_as_dims.iter() {

                let previous_predictions = prediction_matrix.get_mut(&index);

                if previous_predictions.is_none(){ // Prediction is not in the matrix
                    prediction_matrix.insert(index.clone(), vec![(pattern.identifier, pattern.density)]);
                    continue;
                }
                
                // Prediction is in the matrix
                let previous_predictions = previous_predictions.unwrap();
                if !previous_predictions.contains(&(pattern.identifier, *current_prediction)) { // Does not contain current prediction
                    previous_predictions.push((pattern.identifier, *current_prediction));
                }
            }
        }
        return prediction_matrix;
    }

    fn extractUntouchedDeltaRss(prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, tensor: &Tensor) -> HashMap<u32, f64> {
        let mut untouched_delta_rss_s: HashMap<u32, f64> = HashMap::new();

        for (index, pattern_prediction) in prediction_matrix {
            if pattern_prediction.len() == 1 { // There is only one prediction, no intersection, majority of cases
                let pattern_identifier = pattern_prediction.first().unwrap().0;
                let pattern_prediction = pattern_prediction.first().unwrap().1;
                let actual_value = tensor.dims_values.get(index).unwrap();

                let prediction_rss = RssEvolution::calculateRss(actual_value, &pattern_prediction);
                let lambda_0_rss = RssEvolution::calculateRss(actual_value, &tensor.density);
                let delta_rss = prediction_rss - lambda_0_rss;

                let untouched_rss = untouched_delta_rss_s.get_mut(&pattern_identifier);

                match untouched_rss{
                    Some(rss) => {
                        *rss += delta_rss;
                    }

                    None => {
                        untouched_delta_rss_s.insert(pattern_identifier, delta_rss);
                    }
                }
            }
        }
        return untouched_delta_rss_s;
    }

    fn calculateRssAtIntersections(total_rss: &mut f64, pattern: &Pattern, 
        prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, 
        mut accounted_indices: HashSet<Dim<IxDynImpl>>,
        tensor_matrix: &ArrayD<f64>,
        lambda_0: &f64,
        patterns_identifiers: &Vec<u32>) -> HashSet<Dim<IxDynImpl>>{
        
        for index in pattern.indices_as_dims.iter() {
            let predictions = prediction_matrix.get(&index);
            if predictions.is_none() { continue; } // This index is covered by one pattern
            let predictions = predictions.unwrap();

            // There are more than one prediction, retain the ones that are in this submodel
            // and keep the maximum value

            if accounted_indices.contains(index) { continue; } // This index has already been accounted for
            accounted_indices.insert(index.clone());

            let mut max_prediction = f64::MIN;
            for (identifier, prediction) in predictions {
                if patterns_identifiers.contains(identifier) && prediction > &mut max_prediction {
                    max_prediction = *prediction;
                }
            }
            *total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, lambda_0, index, &max_prediction);
        }

        return accounted_indices;
    }

    fn calculateModelRss(tensor: &Tensor, empty_model_rss: &EmptyModelRss, previous_patterns: &Vec<&Pattern>, 
        new_pattern: &Pattern, prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, 
        untouched_delta_rss_s: &HashMap<u32, f64>) -> f64{
        
        let mut total_rss = *empty_model_rss.get();
        let tensor_matrix: &ArrayD<f64> = &tensor.dims_values;

        let mut patterns_identifiers: Vec<u32> = previous_patterns.iter()
            .map(|p| p.identifier)
            .collect();
        patterns_identifiers.push(new_pattern.identifier);

        let mut accounted_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

        // println!("{:?}", previous_patterns.len());

        for pattern in previous_patterns {
            accounted_indices = RssEvolution::calculateRssAtIntersections(&mut total_rss, pattern, prediction_matrix, accounted_indices, 
                tensor_matrix, &tensor.density, &patterns_identifiers);
        }

        accounted_indices = RssEvolution::calculateRssAtIntersections(&mut total_rss, new_pattern, prediction_matrix, accounted_indices, 
            tensor_matrix, &tensor.density, &patterns_identifiers);

        // println!("Ended");

        for pattern in patterns_identifiers {
            let untouched_rss = untouched_delta_rss_s.get(&pattern).unwrap();
            total_rss += untouched_rss;
        }

        
        
        return total_rss;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, 
        intersections_predictions: &IntersectionsPredictions) -> Vec<(u32, f64)>{
        // 100 => 5s, 3s, 1s
        // all => ??, 
    
        let mut patterns: Vec<&Pattern> = identifier_mapper.getRepresentations().iter()
                .map(|r| r.asPattern())
                .collect();
        let pattern_nb = patterns.len();

        let prediction_matrix = RssEvolution::calculatePredictionMatrix(&patterns);
        let untouched_delta_rss_s = RssEvolution::extractUntouchedDeltaRss(&prediction_matrix, tensor);

        // Only keep the predictions that have more than one prediction
        let prediction_matrix: HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>> = prediction_matrix.into_iter()
            .filter(|(_, predictions)| predictions.len() > 1)
            .collect();

        // let mut minimum_rss_value = f64::MAX;
        let mut minimum_rss_value = *empty_model_rss.get();

        let mut sorted_patterns: Vec<&Pattern> = Vec::new();
        let mut rss_evolution: Vec<f64> = vec![minimum_rss_value];
        
        // let bar = progress_bar::new(pattern_nb as u64, "  Orderered patterns");
        while sorted_patterns.len() < pattern_nb { // Sorts all patterns in the patterns vector
            // println!("      Sorting one...");
            let minimum_temp_model_rss = Arc::new(Mutex::new(f64::MAX));
            let minimum_temp_model_pattern: Arc<Mutex<Option<&Pattern>>> = Arc::new(Mutex::new(None));
            let minimum_temp_model_pattern_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
            
            // patterns.par_iter().enumerate().for_each(|(index, pattern)|{

            //     let temp_model_rss = RssEvolution::calculateModelRss(tensor, empty_model_rss, &sorted_patterns, 
            //         pattern, &prediction_matrix, &untouched_delta_rss_s);

            //     let mut minimum_temp_model_rss = minimum_temp_model_rss.lock().unwrap();
            //     if temp_model_rss <= *minimum_temp_model_rss {
            //         let mut minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
            //         let mut minimum_temp_model_pattern_index = minimum_temp_model_pattern_index.lock().unwrap();
                    
            //         *minimum_temp_model_rss = temp_model_rss;
            //         *minimum_temp_model_pattern = Some(pattern);
            //         *minimum_temp_model_pattern_index = index;
            //     }
            // });

            for (index, pattern) in patterns.iter().enumerate(){
                let temp_model_rss = RssEvolution::calculateModelRss(tensor, empty_model_rss, &sorted_patterns, 
                    pattern, &prediction_matrix, &untouched_delta_rss_s);

                let mut minimum_temp_model_rss = minimum_temp_model_rss.lock().unwrap();
                if temp_model_rss <= *minimum_temp_model_rss {
                    let mut minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
                    let mut minimum_temp_model_pattern_index = minimum_temp_model_pattern_index.lock().unwrap();
                    
                    *minimum_temp_model_rss = temp_model_rss;
                    *minimum_temp_model_pattern = Some(pattern);
                    *minimum_temp_model_pattern_index = index;
                }
            }

            let minimum_temp_model_rss = *minimum_temp_model_rss.lock().unwrap();
            let minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
            let minimum_temp_model_pattern_index = *minimum_temp_model_pattern_index.lock().unwrap();

            if minimum_temp_model_rss <= minimum_rss_value { // Its worth to add a pattern
                minimum_rss_value = minimum_temp_model_rss;
                sorted_patterns.push(minimum_temp_model_pattern.unwrap());
                rss_evolution.push(minimum_rss_value);

                patterns.remove(minimum_temp_model_pattern_index);

            }else{ break; }// No pattern decreases the rss

            // bar.inc(1);
        }

        // bar.finish();

        let mut patterns_with_rss: Vec<(u32, f64)> = vec![(0, *empty_model_rss.get())];
        for (i, pattern) in sorted_patterns.iter().enumerate() {
            patterns_with_rss.push((pattern.identifier, rss_evolution[i + 1]));
        }

        return patterns_with_rss;
        
    }

}