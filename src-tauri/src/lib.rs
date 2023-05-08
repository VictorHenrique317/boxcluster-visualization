// https://www.sheshbabu.com/posts/rust-module-system/
#![allow(non_snake_case)]
pub mod common;
pub mod controller;
pub mod services;
pub mod model;
pub mod database;

use std::{fs::File, collections::HashMap};

use services::application::application_service::ApplicationService;

pub fn main() {
    testDag();
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
    
    // let tensor_path = "../tests/test_data/tensors/primary-school.txt".to_owned();
    // let patterns_path = "../tests/test_data/paf-1.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/a.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/a_patterns.txt".to_owned();
    
    // let tensor_path = "../tests/test_data/distance_test/b.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/b_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/c.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/c_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/dataset-co16.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/synth-100-3d-co16.txt".to_owned();

    let tensor_path = "../tests/test_data/tensors/retweets-sparser.txt".to_owned();
    let patterns_path = "../tests/test_data/distance_test_patterns/158-retweets-sparser.txt".to_owned();

    // let tensor_path = "retweets-sparser.txt".to_owned();
    // let patterns_path = "158-retweets-sparser.txt".to_owned();

    let mut application_manager = ApplicationService::new(&tensor_path, &patterns_path);
    application_manager.init();
}

