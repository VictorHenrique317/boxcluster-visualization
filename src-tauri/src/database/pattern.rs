#![allow(non_snake_case)]
use std::collections::HashSet;
use debug_print::{debug_println, debug_print};
use itertools::Itertools;
use ndarray::{IxDynImpl, Dim};
use serde::{Serialize, ser::SerializeStruct};
use std::hash::{Hash, Hasher};

use crate::common::generic_error::GenericError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Relation {
    NotRelatable,
    Overlaps,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32, // Starts at 1
    pub dims_values: Vec<Vec<usize>>,
    pub density: f64,
    pub size: u32,
    pub indices_as_dims: Vec<Dim<IxDynImpl>>,
    pub indices: Vec<Vec<usize>>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.dims_values == other.dims_values {
            return true;
        }
        return false;
    }
}

impl Eq for Pattern {}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Maybe can cause problems? Ideally we should hash dims_values (and not identifier) 
        self.identifier.hash(state);
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("Pattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}

impl Pattern {
    pub fn new<'a>(identifier: u32, dims_values: Vec<Vec<usize>>, density: f64) -> Pattern {
        let size = Pattern::calculateSize(&dims_values);
        let indices = Pattern::getIndices(&dims_values);

        return Pattern {
            identifier: identifier,
            dims_values: Pattern::sortDimsValues(&dims_values),
            density: density,
            size: size,
            indices_as_dims: Pattern::getIndicesAsDims(&indices),
            indices: indices
        };
    }

    fn calculateSize(dims_values: &Vec<Vec<usize>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
    }

    fn getIndices(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        return dims_values.iter()
            .cloned()
            .multi_cartesian_product()
            .collect();
    }

    fn getIndicesAsDims(indices: &Vec<Vec<usize>>) -> Vec<Dim<IxDynImpl>> {
        let mut indices_as_dims: Vec<Dim<IxDynImpl>> = Vec::new();

        for index in indices{
            indices_as_dims.push(Dim(index.clone()));
        }

        return indices_as_dims;
    }

    fn sortDimsValues(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        let mut dims_values: Vec<Vec<usize>> = dims_values.clone();

        for dim_values in dims_values.iter_mut(){
            dim_values.sort();
        }
        return dims_values;
    }

    fn intersectionPercentage(vector: &Vec<usize>, base: &Vec<usize>) -> f64 { // Only works for sorted vectors
        let reference_area = vector.len() as f64;
        let mut used_vector = vector;
        let mut used_base = base;

        if vector.len() > base.len(){
            // One dimension of possible sub 'vector' is larger than the corresponding dim on base, so its not contained in base
            used_vector = base;
            used_base = vector;
            // Switches the vectors of place so that vector is always smaller than base
            // panic!("Wrong use of intersection method");
        }

        let mut current_index = 0;
        let mut contained_values_sum = 0;
        let mut stop = false;

        for element in used_vector{
            loop{
                let base_element = used_base.get(current_index);

                let base_element = match base_element {
                    None => {
                        stop = true;
                        break;
                    }

                    Some(base_element) => { base_element },
                };

                if base_element > element { // If the vector is sorted the value will not be found anymore
                    break;
                }

                current_index += 1; // Element is lesser or equal than base element, can change index

                if element == base_element{
                    contained_values_sum += 1;
                    break;
                }
            }

            if stop{
                break;
            }

        }

        return contained_values_sum as f64 / reference_area; // Percetange of intersection on VECTOR
    }

    pub fn selfRelationTo(&self, pattern: &Pattern) -> Result<Relation, GenericError> {
        debug_print!("    Comparing patterns {} to {}: ", &self.identifier, &pattern.identifier);
        if self.identifier == pattern.identifier{
            debug_println!("{:?} (Identical patterns)", Relation::NotRelatable);
            return Ok(Relation::NotRelatable);
        }  
        
        // Relation of the actual pattern
        let self_dims_values = self.dims_values.iter();
        let mut other_dims_values = pattern.dims_values.iter();
        let mut full_intersection = true;

        for self_dims_value in self_dims_values{
            let other_dims_value = other_dims_values.next()
                .ok_or(GenericError::new(
                    &format!("Pattern {} has less dimensions than pattern {}", &self.identifier, &pattern.identifier),
                    file!(), &line!()))?; 

            let intersection_percentage: f64;

            if self.size > pattern.size{ // Self is possible super
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value);
            }
            else if pattern.size > self.size{ // Pattern is possible super
                intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);
            }
            else{ // No one is super but there may be an overlap
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value); // Doesn't matter the order
            }

            // intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);

            if intersection_percentage == 0.0{
                debug_println!("{:?}", Relation::NotRelatable);
                return Ok(Relation::NotRelatable);
            }

            if intersection_percentage < 1.0{
                full_intersection = false;
            }
        }

        if full_intersection == false {
            debug_println!("{:?}", Relation::Overlaps);
            return Ok(Relation::Overlaps);
        }

        // Here all dimensions have 100% intersection

        if self.size > pattern.size{
            debug_println!("{:?}", Relation::SuperPattern);
            return Ok(Relation::SuperPattern);
        }

        if self.size < pattern.size{
            debug_println!("{:?}", Relation::SubPattern);
            return Ok(Relation::SubPattern);
        }

        // Its the same pattern if the execution reaches here, duplicated patterns exist in the input file
        return Err(GenericError::new(
            &format!("Duplicated patterns detected in input file: {} and {}", &self.identifier, &pattern.identifier),
            file!(), &line!()));
    }

    pub fn intersection(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let intersections = indices
            .intersection(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
    
        return intersections;
    }

    pub fn dimIntersection(&self, other: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut intersections: Vec<Vec<usize>> = Vec::new();

        for (dim, self_dim) in self.dims_values.iter().enumerate(){
            let other_dim = other.dims_values.get(dim)
                .ok_or(GenericError::new(&format!("Pattern {} has less dimensions than pattern {}", self.identifier, other.identifier), file!(), &line!()))?;

            let mut intersection: Vec<usize> = Vec::new();

            for self_value in self_dim.iter(){
                if other_dim.contains(self_value){
                    intersection.push(*self_value);
                }
            }

            if intersection.is_empty(){ return Ok(Vec::new()); } // Intersection has to occur in every dim

            intersections.push(intersection);
        }

        return Ok(intersections);
    }

    pub fn union(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let unions = indices
            .union(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
        
        return unions;
    }
}