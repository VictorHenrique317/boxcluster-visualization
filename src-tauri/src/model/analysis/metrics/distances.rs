#![allow(non_snake_case)]
use std::{collections::{HashMap}, sync::{Arc, Mutex}};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};
use crate::{model::{common::{identifier_mapper::IdentifierMapper, ordered_pair::OrderedPair, progress_bar, intersections_predictions::IntersectionsPredictions}}, database::{tensor::Tensor, subtensor::Subtensor, pattern::Pattern}};
use super::metric::Metric;

#[derive(Default)]
pub struct Distances{
    value: HashMap<u32, HashMap<u32, f64>>, 
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, HashMap<u32, f64>>> for Distances{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

impl Distances{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions) -> Distances{
        println!("  Distances...");
        return Distances { 
            value: Distances::calculate(identifier_mapper, tensor, intersections_predictions),
        };
    }

    fn calculatePairRss(tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, pair: &OrderedPair) -> (HashMap<u32, f64>, f64) {
        let intersections_predictions = intersections_predictions.get();
        let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
        let mut intersection_rss = 0.0;

        let mut saw_pair_overlapping = false;
        for pattern in pair.get(){
            let mut untouched_rss = 0.0;

            for index in pattern.indices.iter(){
                let actual_value = tensor.dims_values.get(index.as_slice()).unwrap();
    
                let possible_overlapper = match saw_pair_overlapping {
                    false => intersections_predictions.get(index),
                    true => None,
                };
                if possible_overlapper.is_some(){ // Maybe intersects the pair pattern

                    let possible_overlapper = *possible_overlapper.unwrap();
                    if possible_overlapper == pair.getOther(pattern) { // Here there is intersection with the pair
                        let overlapper = possible_overlapper;
                        let overlapper_contribution = (actual_value - overlapper.density).powi(2);
        
                        intersection_rss += overlapper_contribution;
                        continue;
                    }
                }
                
                untouched_rss += (actual_value - pattern.density).powi(2);
            }

            saw_pair_overlapping = true;
            untouched_rss_s.insert(pattern.identifier, untouched_rss);
        }

        return (untouched_rss_s, intersection_rss);
    }

    fn getXUYDimsValues(x: &Pattern, y: &Pattern) -> Vec<Vec<usize>> {
        let mut xuy_dims_values: Vec<Vec<usize>> = vec![Vec::new(); x.dims_values.len()];

        for (i, dim_values) in x.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i).unwrap();

            for value in dim_values{
                // if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        for (i, dim_values) in y.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i).unwrap();

            for value in dim_values{
                if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        return xuy_dims_values;
    }

    
    fn getXUY(tensor:&Tensor, x: &Pattern, y: &Pattern) -> Subtensor{
        let xuy_dims_values = Distances::getXUYDimsValues(x, y);
        let xuy = Subtensor::new(tensor, &xuy_dims_values); // Expensive
        return xuy;
    }

    fn getCoveredXUYRss(tensor:&Tensor, xuy: &Subtensor, x: &Pattern, y: &Pattern) -> f64{
        let mut xuy_rss = 0.0;

        let interested_indices: Vec<Vec<usize>> = x.union(y);
        for index in interested_indices.iter(){
            let actual_value = tensor.dims_values.get(index.as_slice()).unwrap();
            xuy_rss += (actual_value - xuy.density).powi(2);
        }   

        return xuy_rss;
    }

    fn normalize(x: &Pattern, y: &Pattern, raw_distance: &f64) -> f64{
        let mut dimensions_sum_area = 1.0;
        for i in 0..x.dims_values.len() {
            let ith_x_dimension_size = x.dims_values.get(i).unwrap().len() as f64;
            let ith_y_dimension_size = y.dims_values.get(i).unwrap().len() as f64;
            dimensions_sum_area *= ith_x_dimension_size + ith_y_dimension_size;
        }
        let mut xuy_reference_density = (x.size as f64 * x.density) + (y.size as f64 * y.density);
        xuy_reference_density /= dimensions_sum_area;

        let mut denominator = x.size as f64 * (x.density - xuy_reference_density).powi(2);
        denominator += y.size as f64 * (y.density - xuy_reference_density).powi(2);  

        let normalized_distance = raw_distance / denominator;
        return (10000.0 * normalized_distance).round() / 10000.0;
    }

    fn insertIntoDistancesMatrix(distances: &mut HashMap<u32, HashMap<u32, f64>>, x: &Pattern, y: &Pattern, distance: &f64){
        if !distances.contains_key(&x.identifier){
            distances.insert(x.identifier, HashMap::new());
        }

        let distances_from_x = distances.get_mut(&x.identifier).unwrap();
        distances_from_x.insert(y.identifier, *distance);
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, intersections_predictions: &IntersectionsPredictions) -> HashMap<u32, HashMap<u32, f64>>{
        // 58s, 30s, 46s, 39s, 37s, 19s, 16s, 5s, 3s
        let distances = Arc::new(Mutex::new(HashMap::new()));
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedRepresentations().iter()
            .map(|r| r.asPattern())
            .collect();

        let total_distances = (identifier_mapper.length().pow(2) as u32 / 2) - identifier_mapper.length() as u32;
        let total_distances = total_distances as u64;
        let bar = progress_bar::new(total_distances, "  Calculated distances");

        // let distances_vec = Arc::new(Mutex::new(Vec::new())); // Create an Arc<Mutex<Vec<f64>>>

        patterns.par_iter().enumerate().for_each(|(row, x)|{

            if row != 0 {
                for (col, y) in patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let xuy = Distances::getXUY(tensor, x, y);
                        let covered_xuy_rss = Distances::getCoveredXUYRss(tensor, &xuy, x, y);
                        
                        let pair = OrderedPair::new(x, y);
                        let (untouched_rss, x_y_intersection_rss) = Distances::calculatePairRss(tensor, intersections_predictions, &pair);
        
                        let untouched_rss_x = *untouched_rss.get(&x.identifier).unwrap();
                        let untouched_rss_y = *untouched_rss.get(&y.identifier).unwrap();
        
                        let raw_distance = covered_xuy_rss - untouched_rss_x - untouched_rss_y - x_y_intersection_rss;
                        let normalized_distance = Distances::normalize(x, y, &raw_distance);

                        // distances_vec.clone().lock().unwrap().push(normalized_distance); // Clone the Arc<Mutex<Vec<f64>>> to move into closure
                        
                        let mut distances = distances.lock().unwrap();
                        Distances::insertIntoDistancesMatrix(&mut distances, &x, &y, &normalized_distance);
                        Distances::insertIntoDistancesMatrix(&mut distances, &y, &x, &normalized_distance);
                        bar.inc(1);
                    }
                }
            }
        });
    
        bar.finish();
        // let distances_vec = distances_vec.lock().unwrap();
        // let distances_mean = mean(&distances_vec);
        // let distances_median = median(&distances_vec);
        // let distances_std_deviance = standard_deviation(&distances_vec, None);

        // dbg!(distances_mean);
        // dbg!(distances_median);
        // dbg!(distances_std_deviance);
        let distances = distances.lock().unwrap().clone();
        return distances;
    }
}