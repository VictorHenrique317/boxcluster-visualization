#![allow(non_snake_case)]
use std::{num::{ParseFloatError, ParseIntError}, collections::HashSet};

use debug_print::debug_println;
use ndarray::{ArrayD, Array, Dim, IxDynImpl};


use crate::database::tensor::{Tensor, TensorType};

use super::{translator::Translator, reader::Reader};

pub struct TensorReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
}

impl TensorReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> TensorReader<'a> {
        return TensorReader {
            file_path: file_path.clone(),
            translator: translator,
        };
    }

    fn calculateDimension(&self) -> u32{
        let first_line: String = Reader::readRawFirstLine(&self.file_path);
        let first_line: Vec<&str> = first_line
            .split(" ")
            .collect();

        return first_line.len() as u32 - 1;
    }

    fn getTensorSize(&self) -> Vec<usize>{
        return self.translator.getSize();
    }

    fn createEmptySizedMatrix(&self, size: &Vec<usize>) -> ArrayD<f64>{
        let matrix: ArrayD<f64> = Array::zeros(Dim(size.clone())).into_dyn();
        return matrix;
    }

    fn defineTensorType(&self, lines_dims: &Vec<Vec<String>>) -> TensorType {
        let mut last_values: HashSet<u32> = HashSet::new();
        for line_dims in lines_dims {
            let last_value = line_dims.last().unwrap();
            
            let float_parse_test: Result<f64, ParseFloatError> = last_value.parse::<f64>(); // Tries to parse to float
            if float_parse_test.is_ok() { // Can be int or float
                
                let int_parse_test: Result<u32, ParseIntError> = last_value.parse::<u32>(); // Tries to parse to int
                if int_parse_test.is_err(){ // Then its number with floating points
                    // 100% of full fuzzy tensors will be identified here, but the others will be identified later (exaustively)
                    return TensorType::FullFuzzy;
                }
            }

            if float_parse_test.is_err() { // Then its a string (dimension)
                return TensorType::PartialImplicit; // There can be partial implicits where the last dimension IS NOT a string
            }

            // Here the tensor can be PartialImplicit, PartialExplicit or FullBoolean
            // Last value is for sure an integer

            let last_value = last_value.parse::<u32>().unwrap();
            if last_value != 0 && last_value != 1 {
                // 100% of partial implicits will be identified here, even if they pass the previous test
                return TensorType::PartialImplicit;
            }

            // Here the tensor can be PartialExplicit or FullBoolean
            // To determine which one it is, we need to iterate through all lines
            last_values.insert(last_value);
        }

        // Here the tensor can be PartialExplicit or FullBoolean
        if last_values.contains(&0) {
            // Then its full boolean
            return TensorType::FullBoolean;
        }

        return TensorType::PartialExplicit;
    }

    fn processFile(&self, tensor_size: &Vec<usize>) -> (ArrayD<f64>, TensorType){
        debug_println!("    Processing tensor file ...");
        let lines = Reader::readRawLines(&self.file_path);
        let lines_dims: Vec<Vec<String>> = lines.into_iter()
            .map(|line| line.split(" ").map(|i| i.to_owned()).collect())
            .collect();

        let mut dims_values_matrix: ArrayD<f64> = self.createEmptySizedMatrix(tensor_size);
        let tensor_type = self.defineTensorType(&lines_dims);

        for line_dims in lines_dims{
            let mut line_dims =  line_dims;
            let mut density = 1.0;
            if tensor_type.hasDensity() {
                density = line_dims.pop().unwrap().parse::<f64>().unwrap();
            }
            
            let translated_line = self.translator.translateLineDims(&line_dims);
            let dims_values: Vec<usize> = translated_line
                .iter()
                .map(|v| *v.get(0).unwrap() as usize)
                .collect();

            let index: Dim<IxDynImpl> = Dim(dims_values);
            let matrix_value = dims_values_matrix.get_mut(index).unwrap();
            *matrix_value = density;
        }
        debug_println!("    Done");
        return (dims_values_matrix, tensor_type);
    }

    fn calculateDensity(&self, dims_values: &ArrayD<f64>, size: &Vec<usize>) -> f64{
        let mut area = 1.0;

        for dim_size in size{
            area *= *dim_size as f64;
        }

        return dims_values.sum() as f64 / area;
    }

    pub fn read(self) -> Tensor{
        let dimension = self.calculateDimension();
        let tensor_size = self.getTensorSize();
        let (dims_values, tensor_type) = self.processFile(&tensor_size);
        let density = self.calculateDensity(&dims_values, &tensor_size);
        return Tensor::new(&self.file_path, dims_values, &tensor_size, &dimension, &density, tensor_type);
    }
    
}