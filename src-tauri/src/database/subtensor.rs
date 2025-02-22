#![allow(non_snake_case)]


use itertools::Itertools;


use crate::common::generic_error::GenericError;

use super::tensor::Tensor;

pub struct Subtensor{
    pub dims_values: Vec<Vec<usize>>,
    pub density: f64,
    pub size: u32,
    pub indices: Vec<Vec<usize>>,
}

impl Subtensor {
    pub fn new(tensor: &Tensor,  dims_values: &Vec<Vec<usize>>) -> Result<Subtensor, GenericError> {
        let (indices, density, size) = Subtensor::iterateOver(tensor, &dims_values)?;

        return Ok(
            Subtensor {
                dims_values: dims_values.clone(),
                density: density,
                size: size,
                indices: indices,
            }
        );
    }

    fn calculateSize(dims_values: &Vec<Vec<usize>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
    }

    fn iterateOver(tensor: &Tensor, dims_values: &Vec<Vec<usize>>) -> Result<(Vec<Vec<usize>>, f64, u32), GenericError> {
        let mut sum = 0.0;
        let subtensor_size = Subtensor::calculateSize(&dims_values);
        let mut indices: Vec<Vec<usize>> = Vec::with_capacity(subtensor_size.clone() as usize);
        
        for index in dims_values.iter().cloned().multi_cartesian_product(){
            sum += *tensor.dims_values.get(index.as_slice())
                .ok_or(GenericError::new(&format!("Tensor index {:?} not found", index), file!(), &line!()))? 
                as f64;

            indices.push(index);
        }

        let density = sum / subtensor_size as f64;
        return Ok((indices, density, subtensor_size as u32));
    }
}
