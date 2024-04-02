// https://www.sheshbabu.com/posts/rust-module-system/
#![allow(non_snake_case)]
pub mod common;
pub mod controller;
pub mod services;
pub mod model;
pub mod database;
pub mod commands;
use std::{collections::HashMap, hash::Hash};
use common::generic_error::GenericError;
use nalgebra::{DMatrix, DVector, SVD};

use model::{analysis::metrics::distances::DistancesTrait, io::pattern_reader::PatternReader};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use rand::{Rng, thread_rng};
use serde_json::Error as SerdeJsonError;


use services::application::application_service::ApplicationService;

pub fn main() {
    testDag();
    return;
    // let distances: DMatrix<f64> = DMatrix::from_row_slice(10, 10, &[
    //     0.0, 587.0, 1212.0, 701.0, 1936.0, 604.0, 748.0, 2139.0, 2182.0, 543.0,
    //     587.0, 0.0, 920.0, 940.0, 1745.0, 1188.0, 713.0, 1858.0, 1737.0, 597.0,
    //     1212.0, 920.0, 0.0, 879.0, 831.0, 1726.0, 1631.0, 949.0, 1021.0, 1494.0,
    //     701.0, 940.0, 879.0, 0.0, 1374.0, 968.0, 1420.0, 1645.0, 1891.0, 1220.0,
    //     1936.0, 1745.0, 831.0, 1374.0, 0.0, 2339.0, 2451.0, 347.0, 959.0, 2300.0,
    //     604.0, 1188.0, 1726.0, 968.0, 2339.0, 0.0, 1092.0, 2594.0, 2734.0, 923.0,
    //     748.0, 713.0, 1631.0, 1420.0, 2451.0, 1092.0, 0.0, 2571.0, 2408.0, 205.0,
    //     2139.0, 1858.0, 949.0, 1645.0, 347.0, 2594.0, 2571.0, 0.0, 678.0, 2442.0,
    //     2182.0, 1737.0, 1021.0, 1891.0, 959.0, 2734.0, 2408.0, 678.0, 0.0, 2329.0,
    //     543.0, 597.0, 1494.0, 1220.0, 2300.0, 923.0, 205.0, 2442.0, 2329.0, 0.0]);

    // // let distances: DMatrix<f64> = hashmap_to_dmatrix(distances);

    // let result = testMds(distances, 2);

    // for i in 0..result.nrows() {
    //     let point: Vec<f64> = result.row(i).iter().cloned().collect();
    //     println!("Point {}: {:?}", i, point);
    // }
        
}

fn printMatrix(matrix: &HashMap<u32, HashMap<u32, f64>>) {
    // Collect and sort the keys
    let mut keys: Vec<&u32> = matrix.keys().collect();
    keys.sort();

    // Print column names
    print!("{:10}", "");
    for &column_name in &keys {
        print!("{:10}", column_name);
    }
    println!();

    // Print rows
    for &row_name in &keys {
        // Print row name
        print!("{:10}", row_name);

        // Print values in the row
        for &column_name in &keys {
            if let Some(value) = matrix.get(row_name).and_then(|row| row.get(column_name)) {
                print!("{:10}", value);
            } else {
                print!("{:10}", "");
            }
        }
        println!();
    }
}

fn testDag(){
    // let path = "../tests/test_data/real1.txt".to_owned(); 
    // let path = "../tests/test_data/4k-big-patterns.txt".to_owned(); 
    // let path = "../tests/test_data/9k-small-patterns.txt".to_owned();
    // let path = "../tests/test_data/simple-msuper.txt".to_owned();
    // let path = "../tests/test_data/simple-msub-2.txt".to_owned();
    // // let path = "../tests/test_data/synth-2.txt".to_owned();
    // let path = "../tests/test_data/paf-1.txt".to_owned();
    // let path = "../tests/test_data/paf-1.processed".to_owned();
    // let path = "../tests/test_data/real-1.txt".to_owned();
    // let path = "../tests/test_data/dataset-co16.fuzzy_tensor".to_owned();
    
    // let tensor_path = "../tests/test_data/tensors/4k-big-patterns-fuzzytensor.txt".to_owned();
    // let patterns_path = "../tests/test_data/4k-big-patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/a.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/a_patterns.txt".to_owned();
    
    // let tensor_path = "../tests/test_data/distance_test/b.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/b_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/c.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/c_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/dataset-co16.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/synth-100-3d-co16.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/retweets-sparser.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test_patterns/158-retweets-sparser.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/primary-school.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/paf-1.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets3d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets3d_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();
    
    let patterns_path = "tests/test_data/dag_test_patterns/complex-msub.txt".to_owned();
    let tensor_path = "tests/test_data/dag_test_patterns/complex-msub.txt".to_owned();

    // let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();

    let mut application_manager = ApplicationService::default();
    application_manager.init(&tensor_path, &patterns_path).unwrap();

}