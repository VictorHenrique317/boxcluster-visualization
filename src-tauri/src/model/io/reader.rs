#![allow(non_snake_case)]
use std::{fs::{self, File}, num::{ParseFloatError, ParseIntError}, error::Error};
use std::io::{prelude::*, BufReader};

pub struct Reader{}

impl Reader{
    pub fn readRawLines(file_path:&String) -> Vec<String>{
        let lines: Vec<String> = fs::read_to_string(file_path)
            .expect(&format!("File not found: {}", &file_path))
            .split("\n")
            .map(|i| i.to_owned().trim().to_lowercase())
            .filter(|i| !i.trim().is_empty())
            .collect();
            
        match lines.get(0){ // Checks if file is empty
            Some(i) => i,
            None => panic!("File is empty"),
        };
        
        return lines;
    }

    pub fn readRawFirstLine(file_path:&String) -> String{
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut first_line = "".to_owned();

        for line in reader.lines() {
            first_line = line.unwrap();
            break;
        }
        
        return first_line;
    }

    fn tryGetDensity(vector: &Vec<String>) -> Result<Option<f64>, Box<dyn Error>>{
        let mut density: Option<f64> = None;
        let vector_length = vector.len();
        for (i, dim_values_str) in vector.iter().enumerate() {
    
            if i == vector_length - 1 { // Only tests on the last element
                let dim_values_str = dim_values_str.replace("\r", "");
                let density_test_1: Result<f64, ParseFloatError> = dim_values_str.parse::<f64>(); // Tries to parse to float
                if density_test_1.is_ok(){ // Can be density or a single dimension
                    
                    let density_test_2: Result<u32, ParseIntError> = dim_values_str.parse(); // Tries to parse to int
                    if density_test_2.is_err(){ // Then its the true density
                        density = Some(density_test_1?);
                    }
                }
            }
        }
        return Ok(density);
    }

    pub fn fileHasDensity(file_path: &String) -> Result<bool, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() { // Line per line
            let current_line = line?;
            let dimensions: Vec<String> = current_line.split(" ")
                .map(|i| i.to_owned())
                .collect();

            let density = Reader::tryGetDensity(&dimensions)?;
            if density.is_some(){ return Ok(true); }
        }

        return Ok(false);
    }
}