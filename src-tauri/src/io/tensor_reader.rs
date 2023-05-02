#![allow(non_snake_case)]
use debug_print::debug_println;
use ndarray::{ArrayD, Array, Dim, IxDynImpl};

use crate::tensor::tensor::Tensor;

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

    fn readDimsValues(&self, tensor_size: &Vec<usize>) -> ArrayD<f64>{
        debug_println!("    Reading dims values...");
        let lines = Reader::readRawLines(&self.file_path);
        let mut dims_values_matrix: ArrayD<f64> = self.createEmptySizedMatrix(tensor_size);
        let mut density: f64 = 1.0; // FULL tensors always have densities
        let file_has_densities = Reader::fileHasDensity(&self.file_path);

        for line in lines{
            let mut line_dims: Vec<String> = line.split(" ").map(|i| i.to_owned()).collect();

            if file_has_densities {
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
        return dims_values_matrix;
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
        let dims_values = self.readDimsValues(&tensor_size);
        let density = self.calculateDensity(&dims_values, &tensor_size);

        return Tensor::new(&self.file_path, dims_values, &tensor_size, &dimension, &density);
    }
    
}