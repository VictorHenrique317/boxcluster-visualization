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

    fn updateRssAtIndex(total_rss: f64, tensor_matrix: &ArrayD<f64>, lambda_0: &f64, index: &Dim<IxDynImpl>, prediction: &f64) -> f64{
        let actual_value = tensor_matrix.get(index).unwrap();
        let rss = (actual_value - prediction).powi(2);
        let lambda0_rss = (actual_value - lambda_0).powi(2);

        let total_rss = total_rss - lambda0_rss + rss;
        return total_rss;
    }

    fn calculateRssAtIndex(tensor_matrix: &ArrayD<f64>, index: &Dim<IxDynImpl>, prediction: &f64) -> f64{
        let actual_value = tensor_matrix.get(index).unwrap();
        let rss = (actual_value - prediction).powi(2);
        return rss;
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

    fn filterPredictionMatrix(prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>, patterns: &Vec<&Pattern>) -> HashMap<Dim<IxDynImpl>, f64>{
        let patterns_identifiers: Vec<u32> = patterns.iter()
            .map(|p| p.identifier)
            .collect();

        let mut filtered_prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();

        return filtered_prediction_matrix;
    }

    // fn calculateModelRss(tensor: &Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
    //     prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>) -> f64{
        
    //     let patterns_identifiers: Vec<u32> = patterns.iter()
    //         .map(|p| p.identifier)
    //         .collect();
    //     let tensor_matrix: &ArrayD<f64> = &tensor.dims_values;
    //     let mut total_rss = *empty_model_rss.get();
    //     let mut accounted_indices: HashSet<&Dim<IxDynImpl>> = HashSet::new();
    
    //     for pattern in patterns {
    //         for index in pattern.indices_as_dims.iter() {

    //             let predictions = prediction_matrix.get(&index).unwrap();

    //             if predictions.len() == 1 { // There is only one prediction, no intersection
    //                 let prediction = predictions.first().unwrap().1;
    //                 // total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, &tensor.density, index, &prediction);
    //                 let prediction_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &prediction);
    //                 let lambda0_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &tensor.density);

    //                 total_rss -= lambda0_rss;
    //                 total_rss += prediction_rss;

    //                 continue;
    //             }

    //             if accounted_indices.contains(index) { continue; } // This index has already been accounted
    //             accounted_indices.insert(index);

    //             // There are more than one prediction, retain the ones that are in this submodel
    //             // and keep the maximum value
    //             let mut max_prediction = f64::MIN;
    //             for (identifier, prediction) in predictions {
    //                 if patterns_identifiers.contains(identifier) && prediction > &max_prediction {
    //                     max_prediction = *prediction;
    //                 }
    //             }

    //             // total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, &tensor.density, index, &max_prediction);
    //             let prediction_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &max_prediction);
    //             let lambda0_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &tensor.density);

    //             total_rss -= lambda0_rss;
    //             total_rss += prediction_rss;
    //         }
    //     }

    //     return total_rss;
    // }

    fn calculateModelRss(tensor: &Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
        prediction_matrix: &HashMap<Dim<IxDynImpl>, Vec<(u32, f64)>>) -> f64{
        
        let mut total_rss = *empty_model_rss.get();
        let tensor_matrix: &ArrayD<f64> = &tensor.dims_values;

        let patterns_identifiers: Vec<u32> = patterns.iter()
            .map(|p| p.identifier)
            .collect();
        let mut accounted_indices: HashSet<&Dim<IxDynImpl>> = HashSet::new();

        for pattern in patterns {
            for index in pattern.indices_as_dims.iter() {
                let predictions = prediction_matrix.get(&index).unwrap();

                if predictions.len() == 1 { // There is only one prediction, no intersection
                    let prediction = predictions.first().unwrap().1;
                    total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, &tensor.density, index, &prediction);
                    continue;
                }

                // There are more than one prediction, retain the ones that are in this submodel
                // and keep the maximum value

                if accounted_indices.contains(index) { continue; } // This index has already been accounted for
                accounted_indices.insert(index);

                let mut max_prediction = f64::MIN;
                for (identifier, prediction) in predictions {
                    if patterns_identifiers.contains(identifier) && prediction > &mut max_prediction {
                        max_prediction = *prediction;
                    }
                }
                total_rss = RssEvolution::updateRssAtIndex(total_rss, tensor_matrix, &tensor.density, index, &max_prediction);

            }
        }
        
        return total_rss;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, 
        intersections_predictions: &IntersectionsPredictions) -> Vec<(u32, f64)>{
        // 5s, 2s
    
        let mut patterns: Vec<&Pattern> = identifier_mapper.getRepresentations().iter()
                .map(|r| r.asPattern())
                .collect();

        let prediction_matrix = RssEvolution::calculatePredictionMatrix(&patterns);
        let pattern_nb = patterns.len();

        // let mut minimum_rss_value = f64::MAX;
        let mut minimum_rss_value = *empty_model_rss.get();

        let mut sorted_patterns: Vec<&Pattern> = Vec::new();
        let mut rss_evolution: Vec<f64> = vec![minimum_rss_value];
        
        let bar = progress_bar::new(pattern_nb as u64, "  Orderered patterns");
        while sorted_patterns.len() < pattern_nb { // Sorts all patterns in the patterns vector
            let minimum_temp_model_rss = Arc::new(Mutex::new(f64::MAX));
            let minimum_temp_model_pattern: Arc<Mutex<Option<&Pattern>>> = Arc::new(Mutex::new(None));
            let minimum_temp_model_pattern_index: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
            
            patterns.par_iter().enumerate().for_each(|(index, pattern)|{
                let mut temp_patterns: Vec<&Pattern> = sorted_patterns.clone();
                temp_patterns.push(pattern);
                // sorted_patterns.push(&pattern);

                // let temp_model_rss = RssEvolution::calculateModelRss(tensor, empty_model_rss, &sorted_patterns, &prediction_matrix);
                let temp_model_rss = RssEvolution::calculateModelRss(tensor, empty_model_rss, &temp_patterns, &prediction_matrix);

                let mut minimum_temp_model_rss = minimum_temp_model_rss.lock().unwrap();
                if temp_model_rss <= *minimum_temp_model_rss {
                    let mut minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
                    let mut minimum_temp_model_pattern_index = minimum_temp_model_pattern_index.lock().unwrap();
                    
                    *minimum_temp_model_rss = temp_model_rss;
                    *minimum_temp_model_pattern = Some(pattern);
                    *minimum_temp_model_pattern_index = index;
                }

                // sorted_patterns.pop();
            });

            let minimum_temp_model_rss = *minimum_temp_model_rss.lock().unwrap();
            let minimum_temp_model_pattern = minimum_temp_model_pattern.lock().unwrap();
            let minimum_temp_model_pattern_index = *minimum_temp_model_pattern_index.lock().unwrap();

            if minimum_temp_model_rss <= minimum_rss_value { // Its worth to add a pattern
                minimum_rss_value = minimum_temp_model_rss;
                sorted_patterns.push(minimum_temp_model_pattern.unwrap());
                rss_evolution.push(minimum_rss_value);

                patterns.remove(minimum_temp_model_pattern_index);

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
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55563.46775797215
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55548.724098351566
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55534.15132468542
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55519.63568524107
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55505.50823457207
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55492.355429111805
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55479.48738446582
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55466.79016959554
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55454.64362154556
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55442.557853373495
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55431.07190785551
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55421.0835804755
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55411.099244773846
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55401.1960104198
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55391.29305496085
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55381.64723069241
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55372.00527927974
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55362.658900330054
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55353.33608175768
// [src/model/analysis/metrics/rss_evolution.rs:139] minimum_temp_model_rss = 55344.05621114673