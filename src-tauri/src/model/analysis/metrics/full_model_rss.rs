// #![allow(non_snake_case)]
// use std::{collections::{HashMap, HashSet, LinkedList}, time::Instant};
// use indicatif::{ProgressBar, ProgressStyle};
// use ndarray::{Dim, IxDynImpl, indices, Dimension, ShapeBuilder};
// use crate::{tensor::tensor::Tensor, pattern::{pattern_mapper::PatternMapper, pattern::Pattern}, utils::{ordered_pair::OrderedPair}};
// use super::{metric::Metric, intersections_predictions::IntersectionsPredictions};

// pub struct FullModelRss{
//     pub untouched_rss_s: HashMap<u32, f64>,
//     pub intersection_rss_s: HashMap<OrderedPair, f64>, // {Overlapped, {overlapper, intersection_rss}}
//     pub model_edges_rss: f64,

//     value: f64,
// }

// #[allow(non_camel_case_types)]
// impl Metric<f64> for FullModelRss{
//     fn get(&self) -> &f64{
//         return &self.value;
//     }
// }

// impl FullModelRss{
//     pub fn new(pattern_mapper: &PatternMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions) ->  FullModelRss{
//         let all_rss = FullModelRss::calculateAll(pattern_mapper, tensor, intersections_predictions);
        
//         return FullModelRss { 
//             untouched_rss_s: all_rss.0,
//             intersection_rss_s: all_rss.1,
//             model_edges_rss: all_rss.2,
//             value: all_rss.3,
//         }
//     }

//     fn getEdgeIndices(pattern_mapper: &PatternMapper, tensor: &Tensor) ->  Vec<Dim<IxDynImpl>>{
//         let mut non_edge_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();
//         let mut edge_indices: Vec<Dim<IxDynImpl>> = Vec::new();
        
//         for pattern in pattern_mapper.getPatterns(){
//             for index in pattern.indices.iter(){
//                 non_edge_indices.insert(index.clone());
//             }
//         }

//         for index in tensor.dims_values.indexed_iter(){
//             let index = index.0;
//             if non_edge_indices.contains(&index) { continue ;}
//             edge_indices.push(index);
//         }

//         return edge_indices;
//     }

//     fn calculatePatternRss(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>){

//         let intersections_predictions: &HashMap<Dim<IxDynImpl>, &Pattern> = intersections_predictions.get();                                                                                                          
//         let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
//         let mut intersection_rss_s: HashMap<OrderedPair, f64> = HashMap::new();

//         for pattern in pattern_mapper.getPatterns(){
//             let mut pattern_untouched_rss = 0.0;

//             for index in pattern.indices.iter(){
//                 let actual_value = tensor.dims_values.get(index).unwrap();

//                 let possible_overlapper = intersections_predictions.get(&index);
//                 if possible_overlapper.is_some() { // Index IS touched by another pattern
//                     let overlapper = *possible_overlapper.unwrap();
//                     if overlapper.density == pattern.density{ // Current pattern was determined to be the overlapper previously
//                         continue;
//                     }

                    
//                     let overlapper_contribution = (actual_value - overlapper.density).powi(2);

//                     let pair = OrderedPair::new(&pattern.identifier, &overlapper.identifier);
//                     let intersection_rss = intersection_rss_s.get_mut(&pair);
                    
//                     if intersection_rss.is_some(){
//                         *intersection_rss.unwrap() += overlapper_contribution; // This pair has a previous RSS value, sum new
//                     }else{
//                         intersection_rss_s.insert(pair, overlapper_contribution); // This pair hasnt a previous RSS value
//                     }
                    
//                     continue;
//                 }
                
//                 // Index IS NOT touched by another pattern
//                 pattern_untouched_rss += (actual_value - pattern.density).powi(2);
//             }

//             untouched_rss_s.insert(pattern.identifier, pattern_untouched_rss);
//         }

//         return (untouched_rss_s, intersection_rss_s);
//     }

//     fn calculateModelEdgesRss(tensor: &Tensor, edge_indices: Vec<Dim<IxDynImpl>>) -> f64{
//         let mut model_edges_rss = 0.0;
//         for edge_index in edge_indices{
//             let actual_value = tensor.dims_values.get(edge_index).unwrap();
//             model_edges_rss += (actual_value - tensor.density).powi(2);
//         }
//         return model_edges_rss;
//     }

//     fn calculateFullModelRss(untouched_rss_s: &HashMap<u32, f64>,
//                             intersection_rss_s: &HashMap<OrderedPair, f64>,
//                             model_edges_rss: &f64) -> f64 {

//         let mut full_model_rss = *model_edges_rss;

//         for (_, untouched_rss) in untouched_rss_s {
//             full_model_rss += *untouched_rss;
//         }

//         for (_, intersection_rss) in intersection_rss_s {
//             full_model_rss += *intersection_rss;
//         }

//         return full_model_rss;
//     }

//     fn calculateAll(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>, f64, f64){
        
//         let (untouched_rss_s, intersection_rss_s) = 
//         FullModelRss::calculatePatternRss(
//             pattern_mapper, 
//             tensor,
//             intersections_predictions);
        
//         println!("  Model edges RSS...");
//         let model_edges_rss: f64 = FullModelRss::calculateModelEdgesRss(
//             tensor, 
//             FullModelRss::getEdgeIndices(pattern_mapper, tensor));

//         let full_model_rss: f64 = FullModelRss::calculateFullModelRss(
//             &untouched_rss_s, 
//             &intersection_rss_s, 
//             &model_edges_rss);

//         return (untouched_rss_s, intersection_rss_s, model_edges_rss, full_model_rss);
//     }
// }
    
    