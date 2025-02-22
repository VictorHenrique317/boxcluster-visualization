#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};
use crate::{common::{generic_error::GenericError, progress_bar}, database::{pattern::{Pattern, Relation}, subtensor::Subtensor, tensor::Tensor}, model::{analysis::ordered_pair::OrderedPair, identifier_mapper::IdentifierMapper}};
use super::{intersections_predictions::IntersectionsPredictions, metric::Metric};

pub trait DistancesTrait {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>;
}

pub struct DistancesView {
    view: HashMap<u32, HashMap<u32, f64>>,
    // mapping: HashMap<u32, u32>,

}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for DistancesView{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

impl DistancesTrait for DistancesView {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

#[allow(non_camel_case_types)]
impl DistancesView {
    fn new(view: &HashMap<u32, HashMap<u32, f64>> , mapping: HashMap<u32, u32>) -> DistancesView{
        mapping.len();
        return DistancesView { 
            view: view.clone(),
            // mapping: mapping,
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Clone)]
pub struct Distances{
    value: HashMap<u32, HashMap<u32, f64>>, 
}

impl DistancesTrait for Distances {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, HashMap<u32, f64>>> for Distances{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

impl Distances{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<Distances, GenericError>{
                
        println!("  Distances...");
        return Ok(
            Distances { 
                value: Distances::calculate(identifier_mapper, tensor, intersections_predictions, visible_identifiers)?,
            }
        );
    }

    fn calculatePairRss(tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, pair: &OrderedPair) 
            -> Result<(HashMap<u32, f64>, f64), GenericError> {

        let intersections_predictions = intersections_predictions.get();
        let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
        let mut intersection_rss = 0.0;
        let mut seen_intersection_indices: HashSet<&Vec<usize>> = HashSet::new();

        for pattern in pair.get(){
            let mut untouched_rss = 0.0;

            for index in pattern.indices.iter(){
                let actual_value = *tensor.dims_values.get(index.as_slice())
                    .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;
    
                let possible_overlapper = intersections_predictions.get(index);

                match possible_overlapper {
                None => { }, // No overlapper
                Some(possible_overlapper) => {
                    if *possible_overlapper == pair.getOther(pattern) { // Here there is intersection with the pair
                        if seen_intersection_indices.contains(index){ continue; } // Avoid double counting

                        let overlapper = possible_overlapper;
                        let overlapper_contribution = (actual_value - overlapper.density).powi(2);
                        
                        intersection_rss += overlapper_contribution;
                        seen_intersection_indices.insert(index);
                        continue;
                    }}
                }

                untouched_rss += (actual_value - pattern.density).powi(2);
            }

            untouched_rss_s.insert(pattern.identifier, untouched_rss);
        }

        return Ok((untouched_rss_s, intersection_rss));
    }

    fn getXUYDimsValues(x: &Pattern, y: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut xuy_dims_values: Vec<HashSet<usize>> = vec![HashSet::new(); x.dims_values.len()];

        for (i, dim_values) in x.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;
            
            for value in dim_values{
                // if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.insert(*value);
            }
        }

        for (i, dim_values) in y.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;

            for value in dim_values{
                xuy_dim_values.insert(*value);
            }
        }

        let xuy_dims_values: Vec<Vec<usize>> = xuy_dims_values.iter()
            .map(|set| set.iter().cloned().collect())
            .collect();

        return Ok(xuy_dims_values);
    }
    
    fn getXUY(tensor:&Tensor, x: &Pattern, y: &Pattern) -> Result<Subtensor, GenericError>{
        let xuy_dims_values = Distances::getXUYDimsValues(x, y)?;
        let xuy = Subtensor::new(tensor, &xuy_dims_values); // Expensive
        return xuy;
    }

    fn getCoveredXUYRss(tensor:&Tensor, xuy: &Subtensor, x: &Pattern, y: &Pattern) -> Result<f64, GenericError>{
        let mut xuy_rss = 0.0;

        let interested_indices: Vec<Vec<usize>> = x.union(y);
        for index in interested_indices.iter(){
            let actual_value = *tensor.dims_values.get(index.as_slice())
                .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;

            xuy_rss += (actual_value - xuy.density).powi(2);
        }   

        // for index in xuy.indices.iter(){
        //     let actual_value = *tensor.dims_values.get(index.as_slice())
        //         .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;

        //     xuy_rss += (actual_value - xuy.density).powi(2);
        // }  

        return Ok(xuy_rss);
    }

    fn normalize(x: &Pattern, y: &Pattern, raw_distance: &f64) -> Result<f64, GenericError>{
        let mut dimensions_sum_area = 1.0;
        for i in 0..x.dims_values.len() {
            let ith_x_dimension_size = x.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            let ith_y_dimension_size = y.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            dimensions_sum_area *= ith_x_dimension_size + ith_y_dimension_size;
        }
        let mut xuy_reference_density = (x.size as f64 * x.density) + (y.size as f64 * y.density);
        xuy_reference_density /= dimensions_sum_area;

        let mut denominator = x.size as f64 * (x.density - xuy_reference_density).powi(2);
        denominator += y.size as f64 * (y.density - xuy_reference_density).powi(2);  

        let normalized_distance = raw_distance / denominator;
        return Ok((10000.0 * normalized_distance).round() / 10000.0);
    }

    fn insertIntoDistancesMatrix(distances: &mut HashMap<u32, HashMap<u32, f64>>, x: &Pattern, y: &Pattern, distance: &f64)
            -> Result<(), GenericError>{

        if !distances.contains_key(&x.identifier){
            distances.insert(x.identifier, HashMap::new());
        }

        let distances_from_x = distances.get_mut(&x.identifier)
            .ok_or(GenericError::new(&format!("Distances from {} not found", &x.identifier), file!(), &line!()))?;

        distances_from_x.insert(y.identifier, *distance);

        return Ok(());
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<HashMap<u32, HashMap<u32, f64>>, GenericError>{
        // 58s, 30s, 46s, 39s, 37s, 19s, 16s, 5s, 3s
        let distances = Arc::new(Mutex::new(HashMap::new()));
        let visible_patterns: Result<Vec<&Pattern>, GenericError> = visible_identifiers.iter()
                .map(|identifier| identifier_mapper.getRepresentation(identifier)?.asPattern())
                .collect();

        let visible_patterns = visible_patterns?;

        let mut total_distances = 0;
        // println!("Visible patterns: {:?}", visible_patterns);
        if visible_identifiers.len() > 1 {
            total_distances = (visible_identifiers.len().pow(2) as u32 / 2) - visible_identifiers.len() as u32
        }

        let total_distances = total_distances as u64;
        let bar = progress_bar::new(total_distances, "  Calculated distances");

        visible_patterns.par_iter().enumerate().try_for_each(|(row, x)| 
                -> Result<(), GenericError>{

            if row != 0 {
                for (col, y) in visible_patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        
                        let relation = x.selfRelationTo(y)?;
                        if relation == Relation::SubPattern || relation == Relation::SuperPattern {
                            let mut distances = distances.lock()
                            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                            Distances::insertIntoDistancesMatrix(&mut distances, &x, &y, &0.0)?;
                            Distances::insertIntoDistancesMatrix(&mut distances, &y, &x, &0.0)?;
                            bar.inc(1);

                            continue;
                        }

                        let xuy = Distances::getXUY(tensor, x, y)?;
                        let covered_xuy_rss: f64 = Distances::getCoveredXUYRss(tensor, &xuy, x, y)?;
                        
                        let pair = OrderedPair::new(x, y);
                        let (untouched_rss, x_y_intersection_rss) = Distances::
                            calculatePairRss(tensor, intersections_predictions, &pair)?;
                        
                        let untouched_rss_x = *untouched_rss.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &x.identifier), file!(), &line!()))?;

                        let untouched_rss_y = *untouched_rss.get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &y.identifier), file!(), &line!()))?;
                        
                        let raw_distance = covered_xuy_rss - untouched_rss_x - untouched_rss_y - x_y_intersection_rss;
                        
                        let normalized_distance = Distances::normalize(x, y, &raw_distance)?;

                        let mut distances = distances.lock()
                            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances, &x, &y, &normalized_distance)?;
                        Distances::insertIntoDistancesMatrix(&mut distances, &y, &x, &normalized_distance)?;
                        bar.inc(1);
                    }
                }
            }

            return Ok(());
        })?;
    
        bar.finish();
        let distances = distances.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();

        // dbg!(&distances);

        return Ok(distances);
    }

    pub fn getView(&self, identifier_mapper: &IdentifierMapper, identifiers: &Vec<u32>) -> Result<DistancesView, GenericError>{
        let mut distances_view: HashMap<u32, HashMap<u32, f64>> = HashMap::new();

        if identifiers.len() == 1 {
            let identifier = *identifiers.get(0).expect("If len() == 1 then should have the first element");
            
            let mut self_distance: HashMap<u32, f64> = HashMap::new();
            self_distance.insert(identifier, 0.0);

            distances_view.insert(identifier, self_distance);
            return Ok(DistancesView::new(&distances_view, HashMap::new()));
        }

        let mut patterns: Vec<&Pattern> = Vec::new();
        // Maps the identifier of the pattern INSIDE the view to the REAL identifier
        let mut mapping: HashMap<u32, u32> = HashMap::new();

        for (i, real_identifier) in identifiers.iter().enumerate(){
            let view_identifier = (i + 1) as u32; // Because i starts at zero
            let representation = identifier_mapper.getRepresentation(real_identifier)?;
            let pattern = representation.asPattern()?;
            
            patterns.push(pattern);
            mapping.insert(view_identifier, *real_identifier);
        }
        
        for (row, x) in patterns.iter().enumerate(){
            if row != 0 {
                for (col, y) in patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let distance = self.value.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} not found", &x.identifier), file!(), &line!()))?
                            .get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} to {} not found", &x.identifier, &y.identifier), file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances_view, &x, &y, distance)?;    
                        Distances::insertIntoDistancesMatrix(&mut distances_view, &y, &x, distance)?;    
                    }
                }
            }
        }

        return Ok(DistancesView::new(&distances_view, mapping));
    }
}