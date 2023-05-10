#![allow(non_snake_case)]
use ndarray::{array, ArrayD};

#[derive(Debug)]
pub enum TensorType {
    FullFuzzy,
    FullBoolean, // Most time expensive
    PartialExplicit, // Most time expensive
    PartialImplicit,
}

impl TensorType{
    pub fn hasDensity(&self) -> bool {
        match self {
            TensorType::FullFuzzy => true,
            TensorType::FullBoolean => true,
            TensorType::PartialExplicit => true,
            TensorType::PartialImplicit => false,
        }
    }
}
pub struct Tensor{
    pub path: String,
    pub dims_values: ArrayD<f64>,
    pub size: Vec<usize>,
    pub dimension: u32,
    pub density: f64,
    pub tensor_type: TensorType 
}

impl Tensor{
    pub fn new(path: &String, dims_values: ArrayD<f64>, size: &Vec<usize>, dimension: &u32, density: &f64, tensor_type: TensorType) -> Self{
        return Tensor{
            path: path.to_owned(),
            density: *density,
            dimension: *dimension,
            size: size.clone(),
            dims_values: dims_values, 
            tensor_type: tensor_type
        };
    }
}