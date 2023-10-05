// https://www.sheshbabu.com/posts/rust-module-system/
#![allow(non_snake_case)]
pub mod common;
pub mod controller;
pub mod services;
pub mod model;
pub mod database;

use std::{fs::File, collections::{HashMap, HashSet}, time::Instant};

use ndarray::{Dim, IxDynImpl};
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

    // let tensor_path = "tests/test_data/tensors/synth_co1.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/synth_co1_truncated_100.txt".to_owned();

    let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();
    let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_X_patterns.txt".to_owned();

    let mut application_manager = ApplicationService::new(&tensor_path, &patterns_path);
    application_manager.init();

    dbg!(application_manager.getRssEvolution().iter().map(|(identifier, _)| identifier.clone()).collect::<Vec<u32>>());
    // Starts at pattern 11
    // let mut test: HashSet<Dim<IxDynImpl>> = HashSet::new(); // 2.264
}

// == RIGHT

// IxDynImpl(Inline(3, [13, 52, 80, 0])): 0.644945, 
// IxDynImpl(Inline(3, [13, 8, 80, 0])): 0.644945, 
// IxDynImpl(Inline(3, [80, 52, 64, 0])): 0.652714, 
// IxDynImpl(Inline(3, [13, 52, 64, 0])): 0.652714, 
// IxDynImpl(Inline(3, [13, 8, 80, 0])): 0.644945, 
// IxDynImpl(Inline(3, [13, 52, 64, 0])): 0.652714, 
// IxDynImpl(Inline(3, [80, 52, 64, 0])): 0.652714, 
// IxDynImpl(Inline(3, [13, 52, 80, 0])): 0.644945, 

// == WRONG
// [13, 52, 80]: 0.644945,
// [13, 8, 80]: 0.644945, 
// [80, 52, 64]: 0.652714, 
// [13, 52, 64]: 0.652714, 
// [13, 8, 80]: 0.644945, 
// [13, 52, 64]: 0.652714, 
// [80, 52, 64]: 0.652714, 
// [13, 52, 80]: 0.644945,
// ====================

