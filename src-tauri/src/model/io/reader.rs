#![allow(non_snake_case)]
use std::{fs::{self, File}, num::{ParseFloatError, ParseIntError}};
use std::io::{prelude::*, BufReader};

use crate::common::generic_error::GenericError;

pub struct Reader{}

impl Reader{
    pub fn readRawLines(file_path:&String) -> Result<Vec<String>, GenericError>{
        let lines: Vec<String> = fs::read_to_string(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?
            .split("\n")
            .map(|i| i.to_owned().trim().to_lowercase())
            .filter(|i| !i.trim().is_empty())
            .collect();
            
        match lines.get(0){ // Checks if file is empty
            Some(i) => i,
            None => return Err(GenericError::new(format!("File {} is empty", file_path).as_str(), file!(), &line!())),
        };
        
        return Ok(lines);
    }

    pub fn readRawFirstLine(file_path:&String) -> Result<String, GenericError>{
        let file = File::open(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?;

        let reader = BufReader::new(file);
        let mut first_line = "".to_owned();

        for line in reader.lines() {
            first_line = line
                .map_err(|_| GenericError::new(format!("Could not read first line of file ({})", file_path).as_str(), file!(), &line!()))?;
            break;
        }
        
        return Ok(first_line);
    }

    pub fn preProcessLine(line: &String) -> Vec<String> {
        let line_columns: Vec<String> = line.split(" ")
                .map(|s| s.to_owned())
                .collect();

        let mut processed_line_columns: Vec<String> = Vec::new();
        for column in line_columns {
            if column == ":"{ break; }

            processed_line_columns.push(column);
        }

        return processed_line_columns;
    }

    fn tryGetDensity(vector: &Vec<String>) -> Result<Option<f64>, GenericError>{
        let mut density: Option<f64> = None;
        let vector_length = vector.len();
        for (i, dim_values_str) in vector.iter().enumerate() {
    
            if i == vector_length - 1 { // Only tests on the last element
                let dim_values_str = dim_values_str.replace("\r", "");
                let density_test_1: Result<f64, ParseFloatError> = dim_values_str.parse::<f64>(); // Tries to parse to float
                if density_test_1.is_ok(){ // Can be density or a single dimension
                    
                    let density_test_2: Result<u32, ParseIntError> = dim_values_str.parse(); // Tries to parse to int
                    if density_test_2.is_err(){ // Then its the true density
                        density = Some(density_test_1.expect("Density test 1 should be ok"));
                    }
                }
            }
        }
        return Ok(density);
    }

    pub fn fileHasDensity(file_path: &String) -> Result<bool, GenericError> {
        let file = File::open(file_path)
            .map_err(|_| GenericError::new(format!("Could not open file ({})", file_path).as_str(), file!(), &line!()))?;

        let reader = BufReader::new(file);

        for line in reader.lines() { // Line per line
            let current_line = line
                .map_err(|_| GenericError::new(format!("Could not read line of file ({})", file_path).as_str(), file!(), &line!()))?;
            
            let dimensions: Vec<String> = Reader::preProcessLine(&current_line);

            let density = Reader::tryGetDensity(&dimensions)?;
            if density.is_some(){ return Ok(true); }
        }

        return Ok(false);
    }
}