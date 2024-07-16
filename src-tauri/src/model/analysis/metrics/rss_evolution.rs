use ndarray::{ArrayD, Dim, IxDynImpl};
use std::iter::Iterator;
use crate::common::generic_error::GenericError;
use crate::common::progress_bar;
use crate::database::pattern::Pattern;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::intersection::intersections_indices::IntersectionsIndices;
use super::intersection::prediction_matrix::PredictionMatrix;
use super::intersection::untouched_delta_rss::UntouchedDeltaRss;
use super::metric::Metric;

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
        patterns: &Vec<&Pattern>, prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) -> Result<RssEvolution, GenericError>{

        println!("  RssEvolution...");
        
        let rss_evolution = RssEvolution::calculate(identifier_mapper, tensor, 
            empty_model_rss, patterns, prediction_matrix, untouched_delta_rss, intersections_indices)?;
        return Ok(
            RssEvolution{
                value: rss_evolution.clone(),
                truncated_value: rss_evolution,
            }
        );
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn updateRssAtIndex(tensor_matrix: &ArrayD<f64>, total_rss: &f64, index: &Dim<IxDynImpl>, old_prediction: &f64, 
                        new_prediction: &f64, prediction_matrix: &mut PredictionMatrix) -> Result<f64, GenericError>{
        
        prediction_matrix.insert(index.clone(), *new_prediction);
        // drop(prediction_matrix);

        let actual_value = tensor_matrix.get(index)
            .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

        let new_prediction_rss = RssEvolution::calculateRss(actual_value, new_prediction);
        let old_prediction_rss = RssEvolution::calculateRss(actual_value, old_prediction);

        let total_rss = total_rss - old_prediction_rss + new_prediction_rss;
        
        return Ok(total_rss);
    }

    fn updatePredictionMatrix(prediction_matrix: &mut PredictionMatrix, intersections_indices: &IntersectionsIndices,
                            candidate_pattern: &Pattern) -> Result<(), GenericError>{
        
        let all_intersections_indices = intersections_indices.getValue(&candidate_pattern.identifier);
        let all_intersections_indices = match all_intersections_indices{
            None => { return Ok(()); } // No intersection
            Some(all_intersections_indices) => { all_intersections_indices },
        };
            
        for (_, intersection_indices) in all_intersections_indices {
            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getMutValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let max_prediction = f64::max(candidate_pattern.density, *previous_prediction);

                if max_prediction > *previous_prediction { // Then change to the new prediction
                    *previous_prediction = max_prediction;
                }
            }
        }

        return Ok(());
    }

    fn calculateCandidateModelRss(current_model_rss: &f64, candidate_pattern: &Pattern,
            tensor: &Tensor,
            identifier_mapper: &IdentifierMapper,
            untouched_delta_rss: &UntouchedDeltaRss,
            prediction_matrix: &mut PredictionMatrix,
            intersections_indices: &IntersectionsIndices,
            seen_candidates: &Vec<u32>) -> Result<f64, GenericError>{

        // let prediction_matrix = prediction_matrix.get();
        // let untouched_delta_rss = untouched_delta_rss.get();
        // let intersections_indices = intersections_indices.get();

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss.getValue(&candidate_pattern.identifier)
            .ok_or(GenericError::new(
                &format!("Untouched delta rss for pattern {} not found", candidate_pattern.identifier), file!(), &line!()))?
            .1;
            
        let candidate_intersections = intersections_indices.getValue(&candidate_pattern.identifier);
        let candidate_intersections = match candidate_intersections {
            None => { return Ok(candidate_model_rss); } // No intersection
            Some(candidate_intersections) => { candidate_intersections },
        };
        
        // Here we can also have indices with no intersection
        let candidate_prediction = candidate_pattern.density;

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let ignore_intersector = !seen_candidates.contains(intersector);
            
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)?
                .asPattern()?.density;

            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let previous_prediction_copy = previous_prediction.clone();

                if ignore_intersector == true { // Intersector is not in the submodel, act as if the candidate is not intersected
                    candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                        &candidate_model_rss, 
                        intersection_index, 
                        &previous_prediction_copy, 
                        &candidate_prediction,
                        prediction_matrix)?;
                    
                    continue;
                }

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction,
                    prediction_matrix)?;
            }
        }
        return Ok(candidate_model_rss);
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
        prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) 
        -> Result<Vec<(u32, f64)>, GenericError>{
        
        let pattern_nb = patterns.len();

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
                prediction_matrix,
                &intersections_indices, 
                &seen_candidates)?;

            current_model_rss = candidate_model_rss;
            rss_evolution.push((pattern.identifier, current_model_rss));
            seen_candidates.push(pattern.identifier);
            RssEvolution::updatePredictionMatrix(prediction_matrix, &intersections_indices, pattern)?;
            bar.inc(1);
        }

        bar.finish();
        return Ok(rss_evolution);
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