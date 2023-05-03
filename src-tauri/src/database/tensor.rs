#![allow(non_snake_case)]
use ndarray::{array, ArrayD};

pub struct Tensor{
    pub path: String,
    pub dims_values: ArrayD<f64>,
    pub size: Vec<usize>,
    pub dimension: u32,
    pub density: f64,
}

impl Default for Tensor {
    fn default() -> Self { 
        return Tensor{
            path: "".to_owned(),
            density: 0.0,
            dimension: 0,
            size: vec![0],
            dims_values: array![0.0].into_dyn(), 
        };
    }
}

impl Tensor{
    pub fn new(path: &String, dims_values: ArrayD<f64>, size: &Vec<usize>, dimension: &u32, density: &f64) -> Self{
        return Tensor{
            path: path.to_owned(),
            density: *density,
            dimension: *dimension,
            size: size.clone(),
            dims_values: dims_values, 
        };
    }
}