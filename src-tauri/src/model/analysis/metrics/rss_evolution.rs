use ndarray::{ArrayD, Dim, IxDynImpl};

use crate::common::progress_bar;
use crate::database::pattern::Pattern;
use crate::model::analysis::intersections_predictions::IntersectionsPredictions;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::metric::Metric;
use std::collections::HashMap;

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

    fn calculateRssAtIndex(tensor_matrix: &ArrayD<f64>, index: &Dim<IxDynImpl>, prediction: &f64) -> f64{
        let actual_value = tensor_matrix.get(index).unwrap();
        let rss = (actual_value - prediction).powi(2);
        return rss;
    }

    fn calculateModelRss(tensor: &Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
            intersections_predictions: &IntersectionsPredictions) -> f64{
        // TODO: USAR INTERSECTION PREDICTIONS, SE DER CERTO RETIRAR PATTERN.INDICES_AS_DIMS
        
        let mut total_rss = *empty_model_rss.get();
        let intersections_predictions = intersections_predictions.get();
        let tensor_matrix: &ArrayD<f64> = &tensor.dims_values;
        let mut prediction_matrix: HashMap<&Dim<IxDynImpl>, f64> = HashMap::new();
    
        for pattern in patterns {

            // let current_prediction = &pattern.density;
            // for index in pattern.indices_as_dims.iter() {

            //     let previous_prediction = prediction_matrix.get_mut(&index);
                
            //     if previous_prediction.is_some(){
            //         let previous_prediction = previous_prediction.unwrap();

            //         if current_prediction > previous_prediction{ // Prediction is the maximum value
            //             *previous_prediction = *current_prediction;
            //         }
    
            //     } else { // Prediction is not in the matrix
            //         prediction_matrix.insert(index, *current_prediction);
            //     }
            // }
            for index in pattern.indices.iter(){
                let intersection_prediction = intersections_predictions.get(index);
                let mut prediction = pattern.density;
                
                if intersection_prediction.is_some(){
                    prediction = intersection_prediction.unwrap().density;
                }

                let index = Dim(index.clone());
                
                let prediction_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, &index, &prediction);
                let lambda0_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, &index, &tensor.density);

                total_rss -= lambda0_rss;
                total_rss += prediction_rss;
            }
            
        }

        // for (index, &prediction) in &prediction_matrix {
        //     let prediction_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &prediction);
        //     let lambda0_rss = RssEvolution::calculateRssAtIndex(tensor_matrix, index, &tensor.density);

        //     total_rss -= lambda0_rss;
        //     total_rss += prediction_rss;
        // }

        return total_rss;
    }

    // fn calculateModelRss(lambda_0: &f64, base_rss: &f64, pattern: &Pattern) -> f64{
    //     let pattern_contribution =pattern.size as f64 * (pattern.density-lambda_0).powi(2);
    //     let rss =  base_rss - pattern_contribution;
    //     return rss;
    // }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, 
        intersections_predictions: &IntersectionsPredictions) -> Vec<(u32, f64)>{
        // 22s, 15s
    
        let mut patterns: Vec<&Pattern> = identifier_mapper.getRepresentations().iter()
                .map(|r| r.asPattern())
                .collect();

        let pattern_nb = patterns.len();

        // let mut minimum_rss_value = f64::MAX;
        let mut minimum_rss_value = *empty_model_rss.get();

        let mut sorted_patterns: Vec<&Pattern> = Vec::new();
        let mut rss_evolution: Vec<f64> = vec![minimum_rss_value];
        
        let bar = progress_bar::new(pattern_nb as u64, "  Orderered patterns");
        while sorted_patterns.len() < pattern_nb { // Sorts all patterns in the patterns vector
            let mut minimum_temp_model_rss = f64::MAX;
            let mut minimum_temp_model_pattern: Option<&Pattern> = None;
            let mut minimum_temp_model_pattern_index: usize = usize::MAX;

            for (index, pattern) in patterns.iter().enumerate() {
                let mut temp_patterns: Vec<&Pattern> = sorted_patterns.clone();
                temp_patterns.push(pattern);

                let temp_model_rss = RssEvolution::calculateModelRss(tensor, empty_model_rss, &temp_patterns, intersections_predictions);
                // let temp_model_rss = RssEvolution::calculateModelRss(&tensor.density, &minimum_rss_value, &pattern);

                if temp_model_rss <= minimum_temp_model_rss {
                    minimum_temp_model_rss = temp_model_rss;
                    minimum_temp_model_pattern = Some(pattern);
                    minimum_temp_model_pattern_index = index;
                }
            }

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