### DIRECTORY src-tauri/src/ FOLDER STRUCTURE ###
/
    flamegraph.svg
    lib.rs
    main.rs
commands/
    application_commands.rs
    mod.rs
    paginator_commands.rs
common/
    generic_error.rs
    mod.rs
    progress_bar.rs
controller/
    application_controller.rs
    dynamic_paginator_controller.rs
    mod.rs
    states/
        mod.rs
        states.rs
database/
    dag.rs
    dag_node.rs
    datapoint.rs
    intersections_details.rs
    mod.rs
    pattern.rs
    raw_pattern.rs
    subtensor.rs
    tensor.rs
model/
    identifier_mapper.rs
    identifier_representation.rs
    mod.rs
    analysis/
        mod.rs
        ordered_pair.rs
        metrics/
            coordinates.rs
            distances.rs
            empty_model_rss.rs
            full_model_rss.rs
            intersections_predictions.rs
            metric.rs
            mod.rs
            rss_evolution.rs
            intersection/
                intersections_indices.rs
                intersections_percentages.rs
                intersection_metrics.rs
                mod.rs
                prediction_matrix.rs
                untouched_delta_rss.rs
    io/
        mod.rs
        pattern_reader.rs
        reader.rs
        tensor_reader.rs
        translator.rs
services/
    datapoint_service.rs
    dynamic_paginator_service.rs
    io_service.rs
    metrics_service.rs
    mod.rs
    plot_service.rs
    application/
        application_service.rs
        application_state_service.rs
        mod.rs
    dag/
        dag_arranger_service.rs
        dag_creator_service.rs
        dag_service.rs
        mod.rs
temp/
    retweets.png
    synth-100-3d-co16.png
### DIRECTORY src-tauri/src/ FOLDER STRUCTURE ###

### DIRECTORY src-tauri/src/ FLATTENED CONTENT ###
### src-tauri/src/flamegraph.svg BEGIN ###
<?xml version="1.0" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg version="1.1" width="1200" height="60" onload="init(evt)" viewBox="0 0 1200 60" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:fg="http://github.com/jonhoo/inferno"><!--Flame graph stack visualization. See https://github.com/brendangregg/FlameGraph for latest version, and http://www.brendangregg.com/flamegraphs.html for examples.--><!--NOTES: --><text x="50.0000%" y="24.00">ERROR: No valid input provided to flamegraph</text></svg>
### src-tauri/src/flamegraph.svg END ###

### src-tauri/src/lib.rs BEGIN ###
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

// fn printMatrix(matrix: &DMatrix<f64>) {
//     // Collect and sort the keys
//     let mut keys: Vec<&u32> = matrix.keys().collect();
//     keys.sort();

//     // Print column names
//     print!("{:10}", "");
//     for &column_name in &keys {
//         print!("{:10}", column_name);
//     }
//     println!();

//     // Print rows
//     for &row_name in &keys {
//         // Print row name
//         print!("{:10}", row_name);

//         // Print values in the row
//         for &column_name in &keys {
//             if let Some(value) = matrix.get(row_name).and_then(|row| row.get(column_name)) {
//                 print!("{:10}", value);
//             } else {
//                 print!("{:10}", "");
//             }
//         }
//         println!();
//     }
// }

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
    
    let tensor_path = "tests/test_data/dissimilarity_matrix/8.tensor".to_owned();
    let patterns_path = "tests/test_data/dissimilarity_matrix/8.patterns".to_owned();

    // let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();

    let mut application_manager = ApplicationService::default();
    application_manager.init(&tensor_path, &patterns_path).unwrap();

}
### src-tauri/src/lib.rs END ###

### src-tauri/src/main.rs BEGIN ###
#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::controller::states::states::*;
use boxcluster_visualization::commands::paginator_commands::*;
use boxcluster_visualization::commands::application_commands::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .manage(ApplicationServiceState(Default::default()))
        .manage(PaginatorServiceState(Default::default()))
        .invoke_handler(tauri::generate_handler![ 
            getSoundingPattern,
            refreshPageSize,
            goToPage,
            goToFirstPage,
            goToLastPage,
            nextPage,
            previousPage,

            initApplication,
            changePatterns,
            ascendDag,
            descendDag,
            truncateModel,
            getFullRssEvolution,
            getTruncatedRssEvolution,
            getDataPoints,
            getAllSubPatternsIdentifiers,
            getPattern,
            getIntersectionsPercentages,
            getIntersectionDetails,
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}

### src-tauri/src/main.rs END ###

### src-tauri/src/commands/application_commands.rs BEGIN ###
use std::collections::HashMap;

use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::{ApplicationServiceState, PaginatorServiceState}, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, raw_pattern::RawPattern}};

#[tauri::command]
pub fn initApplication(application_service: State<ApplicationServiceState>, tensor_path: String, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changeTensor...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.init(&tensor_path, &patterns_path);
}

#[tauri::command]
pub fn changePatterns(application_service: State<ApplicationServiceState>, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changePatterns...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.changePatterns(&patterns_path);
}

#[tauri::command]
pub fn truncateModel(application_service: State<ApplicationServiceState>, new_size: u32) 
        -> Result<Vec<(f32, f32)>, GenericError>{
    println!("Calling truncateModel...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.truncateModel(&new_size);
}

#[tauri::command]
pub fn getFullRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getFullRssEvolution...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getFullRssEvolution();
}

#[tauri::command]
pub fn getTruncatedRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getTruncatedRssEvolution...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getTruncatedRssEvolution();
}

#[tauri::command]
pub fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) -> Result<(), GenericError>{
    println!("Calling descendDag...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.descendDag(&next_identifier);
}

#[tauri::command]
pub fn ascendDag(application_service: State<ApplicationServiceState>) -> Result<(), GenericError>{
    println!("Calling ascendDag...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.ascendDag();
}

#[tauri::command]
pub fn getDataPoints(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDataPoints...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getDataPoints();
}

#[tauri::command]
pub fn getAllSubPatternsIdentifiers(application_service: State<ApplicationServiceState>) -> Result<Vec<u32>, GenericError> {
    println!("Calling getAllSubPatternsIdentifiers...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getAllSubPatternsIdentifiers();
}

#[tauri::command]
pub fn getPattern(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<RawPattern, GenericError> {
    println!("Calling getPattern...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getRawPattern(&identifier);
}

#[tauri::command]
pub fn getIntersectionsPercentages(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<HashMap<u32, f64>, GenericError> {
    println!("Calling getIntersectionsPercentages...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getIntersectionsPercentages(&identifier);
}

#[tauri::command]
pub fn getIntersectionDetails(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<IntersectionsDetails, GenericError> {
    println!("Calling getIntersectionDetails...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getIntersectionDetails(&identifier);
}
### src-tauri/src/commands/application_commands.rs END ###

### src-tauri/src/commands/mod.rs BEGIN ###
pub mod paginator_commands;
pub mod application_commands;
### src-tauri/src/commands/mod.rs END ###

### src-tauri/src/commands/paginator_commands.rs BEGIN ###
use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::{ApplicationServiceState, PaginatorServiceState}, database::raw_pattern::RawPattern};

#[tauri::command]
pub fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<RawPattern, GenericError> {

    println!("Calling getSoundingPattern...");

    let paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.getSoundingPattern(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn refreshPageSize(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_size: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {
    
    println!("Calling refreshPageSize...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.refreshPageSize(identifier_mapper, application_service.getTranslator(), page_size);
}

#[tauri::command]
pub fn goToPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_index: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &page_index);
}

#[tauri::command]
pub fn goToFirstPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToFirstPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    let first_page = paginator_service.first_page.clone();
    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &first_page);
}

#[tauri::command]
pub fn goToLastPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToLastPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let last_page = paginator_service.last_page.clone();
    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &last_page);
}

#[tauri::command]
pub fn nextPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling nextPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.nextPage(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn previousPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling previousPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.previousPage(identifier_mapper, application_service.getTranslator());
}
### src-tauri/src/commands/paginator_commands.rs END ###

### src-tauri/src/common/generic_error.rs BEGIN ###
use std::sync::{PoisonError, MutexGuard};
use colored::*;

#[derive(Debug, thiserror::Error)]
pub enum GenericError {
    #[error(transparent)]
    Tauri(tauri::Error),

    #[error("Failed to acquire lock due to a poisoned mutex.")]
    MutexPoisonError,

    // Add a new variant for a custom error message
    #[error("ERROR in file {file} at line {line}: {message}")]
    Custom {
        message: String,
        file: String,
        line: u32,
    },
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for GenericError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        GenericError::MutexPoisonError
    }
}

impl serde::Serialize for GenericError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {   
        serializer.serialize_str(self.getErrorMessage().as_str())
    }
}

impl GenericError {
    pub fn new(message: &str, file: &str, line: &u32) -> GenericError {
        GenericError::Custom { message: message.to_string(), file: file.to_string(), line: *line }
    }

    pub fn getErrorMessage(&self) -> String {
        match self {
            GenericError::MutexPoisonError => {
                format!("Failed to acquire lock due to a poisoned mutex.")
            },
            GenericError::Custom { message, file, line } => {
                format!("ERROR in file {} at line {}: {}", file, line, message)
            },
            GenericError::Tauri(err) => {
              format!("Tauri error: {}", err)
            },
        }
    }

    pub fn print(&self) {
        println!("{}", self.getErrorMessage().red());
    }

    pub fn from<T, E: std::fmt::Debug>(result: Result<T, E>, message: &str, file: &str, line: &u32) -> Result<T, GenericError> {
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                let error = GenericError::new(message, file, line);
                Err(error)
            }
        }
    }
}

### src-tauri/src/common/generic_error.rs END ###

### src-tauri/src/common/mod.rs BEGIN ###
pub mod progress_bar;
pub mod generic_error;
### src-tauri/src/common/mod.rs END ###

### src-tauri/src/common/progress_bar.rs BEGIN ###
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

pub fn new(total: u64, message: &str) -> ProgressBar {
    let hide_progress = env::var("HIDE_PROGRESS").is_ok();
    let bar = if hide_progress {
        ProgressBar::hidden()
    } else {
        let bar = ProgressBar::new(total);
        bar.set_message(message.to_owned());
        bar.set_style(ProgressStyle::default_bar()
            .template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} | Elapsed time: {elapsed} | Estimated time:{eta}").unwrap()
            .progress_chars("=>-"));
        bar
    };

    return bar;
}

### src-tauri/src/common/progress_bar.rs END ###

### src-tauri/src/controller/application_controller.rs BEGIN ###

### src-tauri/src/controller/application_controller.rs END ###

### src-tauri/src/controller/dynamic_paginator_controller.rs BEGIN ###
// #![allow(non_snake_case)]
// use crate::pattern::pattern::Pattern;
// use crate::states::states::{DagState, PaginatorState};
// use tauri::State;

// #[tauri::command]
// pub fn getSoundingPattern(dag_state:State<DagState>, paginator_state:State<PaginatorState>) -> Pattern{
//     let dag = dag_state.0.lock().unwrap();
//     let paginator = paginator_state.0.lock().unwrap();

//     return paginator.getSoundingPattern();
// }

// #[tauri::command]
// pub fn refreshPageSize(pageSize: u32, dag_state:State<DagState>, paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let dag = dag_state.0.lock().unwrap();
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.refreshPageSize(pageSize);
// }

// #[tauri::command]
// pub fn goToPage(page_index: u32, paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.goToPage(&page_index);
// }

// #[tauri::command]
// pub fn goToFirstPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();
//     let first_page = paginator.first_page.clone();

//     return paginator.goToPage(&first_page);
// }

// #[tauri::command]
// pub fn goToLastPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();
//     let last_page = paginator.last_page.clone();

//     return paginator.goToPage(&last_page);
// }

// #[tauri::command]
// pub fn nextPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.nextPage();
// }

// #[tauri::command]
// pub fn previousPage(paginator_state:State<PaginatorState>) -> (Vec<Pattern>, u32, u32){
//     let mut paginator = paginator_state.0.lock().unwrap();

//     return paginator.previousPage();
// }
### src-tauri/src/controller/dynamic_paginator_controller.rs END ###

### src-tauri/src/controller/mod.rs BEGIN ###
pub mod states;
// pub mod application_controller;
// pub mod dynamic_paginator_controller;
### src-tauri/src/controller/mod.rs END ###

### src-tauri/src/controller/states/mod.rs BEGIN ###
pub mod states;
### src-tauri/src/controller/states/mod.rs END ###

### src-tauri/src/controller/states/states.rs BEGIN ###
use std::sync::Mutex;
use crate::services::{dynamic_paginator_service::DynamicPaginatorService, application::application_service::ApplicationService};

pub struct ApplicationServiceState(pub Mutex<ApplicationService>);
pub struct PaginatorServiceState(pub Mutex<DynamicPaginatorService>);
### src-tauri/src/controller/states/states.rs END ###

### src-tauri/src/database/dag.rs BEGIN ###
#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use crate::common::generic_error::GenericError;

use super::{dag_node::DagNode, pattern::Pattern};

#[derive(Default)]
pub struct Dag {
    nodes: HashMap<u32, DagNode>,
}

impl Dag {
    pub fn new(patterns: &Vec<&Pattern>) -> Self {
        return Dag { 
            nodes: Dag::createNodes(patterns),
        };
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        return self.nodes.keys().map(|i| *i).collect();
    }


    pub fn getNumberOfNodes(&self) -> u32 {
        return self.nodes.len() as u32;
    }

    pub fn isEdge(&self, node: &u32) -> Result<bool, GenericError> {
        let node_p = self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?;
        
        return Ok(node_p.subs.len() == 0);
    }

    pub fn isFont(&self, node: &u32) -> Result<bool, GenericError> {
        return Ok(
            self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?
            .supers.len() == 0
        );
    }

    pub fn hasSubs(&self, node: &u32) -> Result<bool, GenericError> {
        return Ok(
            self.nodes.get(node)
            .ok_or(GenericError::new(&format!("Node {} not found", node), file!(), &line!()))?
            .subs.len() != 0
        );
    }

    pub fn getOverllapings(&self) -> HashMap<u32, HashSet<u32>>{
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (id, node) in self.nodes.iter(){
            overlappings.insert(*id, node.overlappings.clone());
        }
        
        return overlappings;
    }
    
    pub fn extractNodes(self) -> HashMap<u32, DagNode>{
        return self.nodes;
    }

    fn createNodes(_patterns: &[&Pattern]) -> HashMap<u32, DagNode> {
        todo!()
    }

}

### src-tauri/src/database/dag.rs END ###

### src-tauri/src/database/dag_node.rs BEGIN ###
use std::collections::HashSet;


pub struct DagNode{
    pub identifier: u32,
    pub supers: Vec<u32>,
    pub subs: Vec<u32>,

    // This pattern is overlapped by these ones, here only appear the patterns that overlaps AND 
    // have greater density.
    pub overlappings: HashSet<u32>, 
}

impl DagNode{
    pub fn new(identifier: &u32) -> DagNode{
        return DagNode { 
            identifier: *identifier,
            supers: Vec::new(), 
            subs: Vec::new(), 
            overlappings: HashSet::new() };
    }
}
### src-tauri/src/database/dag_node.rs END ###

### src-tauri/src/database/datapoint.rs BEGIN ###
use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub identifier: u32,
    pub size: f32,
    pub pattern_size: u32,
    pub density: f32,
    pub stroke_width: u32,

    pub x: f32,
    pub y: f32,

    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: f32,
}

impl Serialize for DataPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("DataPoint", 8)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("pattern_size", &self.pattern_size)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("stroke_width", &self.stroke_width)?;
    
 
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
 
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;

       state.end()
    }
 }

impl DataPoint {
    pub fn new(identifier: &u32, size: &f32, pattern_size: &u32, density: &f32, stroke_width: &u32, x: &f32, y: &f32, r: &u32, g: &u32, b: &u32, a: &f32) -> DataPoint { 
        return DataPoint { identifier: *identifier, 
            x: *x, 
            y: *y , 
            size: *size, 
            pattern_size: *pattern_size, 
            density: *density, 
            stroke_width:*stroke_width, 
            r: *r,  
            g: *g,
            b: *b,
            a: *a,
            };
    }
}
### src-tauri/src/database/datapoint.rs END ###

### src-tauri/src/database/intersections_details.rs BEGIN ###
use serde::{ser::SerializeStruct, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IntersectionsDetails{
    pub identifier: u32,
    pub total_untouched_percentage: f64,
    pub total_intersection_percentage: f64,

    pub intersections: HashMap<u32, (f64, Vec<Vec<String>>)>, // Identifier, (percentage, raw_dims)
}

impl IntersectionsDetails{
    pub fn new(identifier: u32, total_untouched_percentage: f64, total_intersection_percentage: f64, 
                intersections: HashMap<u32, (f64, Vec<Vec<String>>)>) -> IntersectionsDetails{
        
        return IntersectionsDetails{
            identifier: identifier,
            total_untouched_percentage: total_untouched_percentage,
            total_intersection_percentage: total_intersection_percentage,
            intersections: intersections,
        };
    }
}

impl Serialize for IntersectionsDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("IntersectionsDetails", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("total_untouched_percentage", &self.total_untouched_percentage)?;
        state.serialize_field("total_intersection_percentage", &self.total_intersection_percentage)?;
        state.serialize_field("intersections", &self.intersections)?;
        state.end()
    }
}
### src-tauri/src/database/intersections_details.rs END ###

### src-tauri/src/database/mod.rs BEGIN ###
pub mod dag_node;
pub mod dag;
pub mod datapoint;
pub mod pattern;
pub mod raw_pattern;
pub mod subtensor;
pub mod tensor;
pub mod intersections_details;
### src-tauri/src/database/mod.rs END ###

### src-tauri/src/database/pattern.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashSet;
use debug_print::{debug_println, debug_print};
use itertools::Itertools;
use ndarray::{IxDynImpl, Dim};
use serde::{Serialize, ser::SerializeStruct};
use std::hash::{Hash, Hasher};

use crate::common::generic_error::GenericError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Relation {
    NotRelatable,
    Overlaps,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32, // Starts at 1
    pub dims_values: Vec<Vec<usize>>,
    pub density: f64,
    pub size: u32,
    pub indices_as_dims: Vec<Dim<IxDynImpl>>,
    pub indices: Vec<Vec<usize>>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.dims_values == other.dims_values {
            return true;
        }
        return false;
    }
}

impl Eq for Pattern {}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Maybe can cause problems? Ideally we should hash dims_values (and not identifier) 
        self.identifier.hash(state);
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("Pattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}

impl Pattern {
    pub fn new<'a>(identifier: u32, dims_values: Vec<Vec<usize>>, density: f64) -> Pattern {
        let size = Pattern::calculateSize(&dims_values);
        let indices = Pattern::getIndices(&dims_values);

        return Pattern {
            identifier: identifier,
            dims_values: Pattern::sortDimsValues(&dims_values),
            density: density,
            size: size,
            indices_as_dims: Pattern::getIndicesAsDims(&indices),
            indices: indices
        };
    }

    fn calculateSize(dims_values: &Vec<Vec<usize>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
    }

    fn getIndices(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        return dims_values.iter()
            .cloned()
            .multi_cartesian_product()
            .collect();
    }

    fn getIndicesAsDims(indices: &Vec<Vec<usize>>) -> Vec<Dim<IxDynImpl>> {
        let mut indices_as_dims: Vec<Dim<IxDynImpl>> = Vec::new();

        for index in indices{
            indices_as_dims.push(Dim(index.clone()));
        }

        return indices_as_dims;
    }

    fn sortDimsValues(dims_values: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        let mut dims_values: Vec<Vec<usize>> = dims_values.clone();

        for dim_values in dims_values.iter_mut(){
            dim_values.sort();
        }
        return dims_values;
    }

    fn intersectionPercentage(vector: &Vec<usize>, base: &Vec<usize>) -> f64 { // Only works for sorted vectors
        let reference_area = vector.len() as f64;
        let mut used_vector = vector;
        let mut used_base = base;

        if vector.len() > base.len(){
            // One dimension of possible sub 'vector' is larger than the corresponding dim on base, so its not contained in base
            used_vector = base;
            used_base = vector;
            // Switches the vectors of place so that vector is always smaller than base
            // panic!("Wrong use of intersection method");
        }

        let mut current_index = 0;
        let mut contained_values_sum = 0;
        let mut stop = false;

        for element in used_vector{
            loop{
                let base_element = used_base.get(current_index);

                let base_element = match base_element {
                    None => {
                        stop = true;
                        break;
                    }

                    Some(base_element) => { base_element },
                };

                if base_element > element { // If the vector is sorted the value will not be found anymore
                    break;
                }

                current_index += 1; // Element is lesser or equal than base element, can change index

                if element == base_element{
                    contained_values_sum += 1;
                    break;
                }
            }

            if stop{
                break;
            }

        }

        return contained_values_sum as f64 / reference_area; // Percetange of intersection on VECTOR
    }

    pub fn selfRelationTo(&self, pattern: &Pattern) -> Result<Relation, GenericError> {
        debug_print!("    Comparing patterns {} to {}: ", &self.identifier, &pattern.identifier);
        if self.identifier == pattern.identifier{
            debug_println!("{:?} (Identical patterns)", Relation::NotRelatable);
            return Ok(Relation::NotRelatable);
        }  
        
        // Relation of the actual pattern
        let self_dims_values = self.dims_values.iter();
        let mut other_dims_values = pattern.dims_values.iter();
        let mut full_intersection = true;

        for self_dims_value in self_dims_values{
            let other_dims_value = other_dims_values.next()
                .ok_or(GenericError::new(
                    &format!("Pattern {} has less dimensions than pattern {}", &self.identifier, &pattern.identifier),
                    file!(), &line!()))?; 

            let intersection_percentage: f64;

            if self.size > pattern.size{ // Self is possible super
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value);
            }
            else if pattern.size > self.size{ // Pattern is possible super
                intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);
            }
            else{ // No one is super but there may be an overlap
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value); // Doesn't matter the order
            }

            // intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);

            if intersection_percentage == 0.0{
                debug_println!("{:?}", Relation::NotRelatable);
                return Ok(Relation::NotRelatable);
            }

            if intersection_percentage < 1.0{
                full_intersection = false;
            }
        }

        if full_intersection == false {
            debug_println!("{:?}", Relation::Overlaps);
            return Ok(Relation::Overlaps);
        }

        // Here all dimensions have 100% intersection

        if self.size > pattern.size{
            debug_println!("{:?}", Relation::SuperPattern);
            return Ok(Relation::SuperPattern);
        }

        if self.size < pattern.size{
            debug_println!("{:?}", Relation::SubPattern);
            return Ok(Relation::SubPattern);
        }

        // Its the same pattern if the execution reaches here, duplicated patterns exist in the input file
        return Err(GenericError::new(
            &format!("Duplicated patterns detected in input file: {} and {}", &self.identifier, &pattern.identifier),
            file!(), &line!()));
    }

    pub fn intersection(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let intersections = indices
            .intersection(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
    
        return intersections;
    }

    pub fn dimIntersection(&self, other: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut intersections: Vec<Vec<usize>> = Vec::new();

        for (dim, self_dim) in self.dims_values.iter().enumerate(){
            let other_dim = other.dims_values.get(dim)
                .ok_or(GenericError::new(&format!("Pattern {} has less dimensions than pattern {}", self.identifier, other.identifier), file!(), &line!()))?;

            let mut intersection: Vec<usize> = Vec::new();

            for self_value in self_dim.iter(){
                if other_dim.contains(self_value){
                    intersection.push(*self_value);
                }
            }

            if intersection.is_empty(){ return Ok(Vec::new()); } // Intersection has to occur in every dim

            intersections.push(intersection);
        }

        return Ok(intersections);
    }

    pub fn union(&self, pattern: &Pattern) -> Vec<Vec<usize>> {
        let indices: HashSet<Vec<usize>> = self.indices.iter().cloned().collect();
        let unions = indices
            .union(&pattern.indices.iter().cloned().collect())
            .map(|i| i.clone())
            .collect();
        
        return unions;
    }
}
### src-tauri/src/database/pattern.rs END ###

### src-tauri/src/database/raw_pattern.rs BEGIN ###
use serde::{ser::SerializeStruct, Serialize};

#[derive(Clone, Debug)]
pub struct RawPattern {
    pub identifier: u32,
    pub dims_values: Vec<Vec<String>>,
    pub density: f64,
    pub size: u32,
}

impl RawPattern {
    pub fn new<'a>(identifier: &u32, dims_values: &Vec<Vec<String>>, density: &f64, size: &u32) -> RawPattern {

        return RawPattern {
            identifier: *identifier,
            dims_values: dims_values.clone(),
            density: *density,
            size: *size,
        };
    }
}

impl Serialize for RawPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("RawPattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}
### src-tauri/src/database/raw_pattern.rs END ###

### src-tauri/src/database/subtensor.rs BEGIN ###
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

### src-tauri/src/database/subtensor.rs END ###

### src-tauri/src/database/tensor.rs BEGIN ###
#![allow(non_snake_case)]
use ndarray::ArrayD;

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
### src-tauri/src/database/tensor.rs END ###

### src-tauri/src/model/identifier_mapper.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{database::{pattern::Pattern, dag_node::DagNode, datapoint::DataPoint}, common::generic_error::GenericError};

use super::identifier_representation::IdentifierRepresentation;

pub struct IdentifierMapper{
    mapping: HashMap<u32, IdentifierRepresentation>, // WARNING: ID's start at 1
}

impl IdentifierMapper{
    pub fn new(pattern_representations: Vec<Pattern>) -> IdentifierMapper{
        return IdentifierMapper { 
            mapping: IdentifierMapper::createInitialMapping(pattern_representations),
        };
    }

    fn createInitialMapping(pattern_representations: Vec<Pattern>) -> HashMap<u32, IdentifierRepresentation>{
        let mut mapping: HashMap<u32, IdentifierRepresentation> = HashMap::new();

        for pattern_representation in pattern_representations {
            mapping.insert(pattern_representation.identifier, IdentifierRepresentation::new(pattern_representation));
        }

        return mapping;
    }

    fn removeAllDagNodeRepresentations(&mut self){
        for identifier_representation in self.mapping.values_mut() {
            identifier_representation.removeDagNodeRepresentation();
        }
    }

    fn removeAllDatapointRepresentations(&mut self){
        for identifier_representation in self.mapping.values_mut() {
            identifier_representation.removeDatapointRepresentation();
        }
    }

    pub fn insertDagNodeRepresentations(&mut self, dag_nodes_representations: Vec<DagNode>) -> Result<(), GenericError>{
        self.removeAllDagNodeRepresentations();

        let dag_nodes_representations: HashMap<u32, DagNode> = dag_nodes_representations.into_iter()
            .map(|dag_node| (dag_node.identifier, dag_node))
            .collect();

        for (identifier, dag_nodes_representation) in dag_nodes_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDagNodeRepresentation(dag_nodes_representation);
        }

        return Ok(());
    }

    pub fn insertDataPointRepresentations(&mut self, data_point_representations: Vec<DataPoint>) -> Result<(), GenericError>{
        self.removeAllDatapointRepresentations();

        let data_point_representations: HashMap<u32, DataPoint> = data_point_representations.into_iter()
            .map(|data_point| (data_point.identifier, data_point))
            .collect();
        
        for (identifier, data_point_representation) in data_point_representations {
            let identifier_representation = self.mapping.get_mut(&identifier)
                .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()))?;

            identifier_representation.insertDataPointRepresentation(data_point_representation);
        }

        return Ok(());
    }

    pub fn getRepresentation(&self, identifier: &u32) -> Result<&IdentifierRepresentation, GenericError>{
        return self.mapping.get(identifier)
            .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()));
    }

    pub fn getRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        return self.mapping.values().collect();
    }

    pub fn getRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        return identifiers.iter()
            .filter_map(|identifier| self.getRepresentation(identifier).ok())
            .collect();
    }

    pub fn getOrderedRepresentationsFrom(&self, identifiers: &Vec<u32>) -> Vec<&IdentifierRepresentation>{
        let mut identifiers = identifiers.clone();
        identifiers.sort();

        let representations = self.getRepresentationsFrom(&identifiers);
        // Representations will be naturally ordered
        return representations;
    }

    pub fn getIdentifier(&self, identifier: &u32) -> Result<&IdentifierRepresentation, GenericError>{
        return self.mapping.get(identifier)
            .ok_or(GenericError::new("Could not get identifier representation", file!(), &line!()));
    }

    pub fn getIdentifiers(&self) -> Vec<u32>{
        let mut keys: Vec<u32> = self.mapping.keys().cloned().collect();
        keys.sort();
        return keys;
    }

    pub fn getMapping(&self) -> &HashMap<u32, IdentifierRepresentation>{
        return &self.mapping;
    }

    pub fn getOrderedRepresentations(&self) -> Vec<&IdentifierRepresentation>{
        let keys: Vec<u32> = self.getIdentifiers();

        let values: Vec<&IdentifierRepresentation> = keys.iter()
            .map(|k| self.mapping.get(k)
                .expect("Should have gotten identifier representation"))
            .collect();
        return values;
    }

    pub fn getOrderedPatterns(&self) -> Vec<&Pattern> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedPatternsFrom(&self, identifiers: &Vec<u32>) -> Vec<&Pattern> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asPattern()
                .expect("Should have gotten pattern representation"))
            .collect();
    }

    pub fn getOrderedDataPoints(&self) -> Vec<&DataPoint> {
        return self.getOrderedRepresentations().iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn getOrderedDataPointsFrom(&self, identifiers: &Vec<u32>) -> Vec<&DataPoint> {
        return self.getOrderedRepresentationsFrom(identifiers).iter()
            .map(|representation| representation.asDataPoint()
                .expect("Should have gotten data point representation"))
            .collect();
    }

    pub fn length(&self) -> u32{
        return self.mapping.len() as u32;
    }
}
### src-tauri/src/model/identifier_mapper.rs END ###

### src-tauri/src/model/identifier_representation.rs BEGIN ###
use crate::{database::{dag_node::DagNode, datapoint::DataPoint, pattern::Pattern, raw_pattern::RawPattern}, common::generic_error::GenericError};

use super::io::translator::Translator;


pub struct IdentifierRepresentation {
    pattern_representation: Option<Pattern>,
    dag_node_representation: Option<DagNode>,
    data_point_representation: Option<DataPoint>,
}

impl IdentifierRepresentation {
    pub fn new(pattern_representation: Pattern) -> IdentifierRepresentation {
        return IdentifierRepresentation { 
            pattern_representation: Some(pattern_representation), 
            dag_node_representation: None, 
            data_point_representation: None 
        };
    }

    pub fn insertDagNodeRepresentation(&mut self, dag_node_representation: DagNode){
        self.dag_node_representation = Some(dag_node_representation);
    }

    pub fn insertDataPointRepresentation(&mut self, data_point_representation: DataPoint){
        self.data_point_representation = Some(data_point_representation);
    }

    pub fn removeDagNodeRepresentation(&mut self){
        self.dag_node_representation = None;
    }

    pub fn removeDatapointRepresentation(&mut self){
        self.data_point_representation = None;
    }

    pub fn asPattern(&self) -> Result<&Pattern, GenericError> {
        return self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()));
    }

    pub fn asRawPattern(&self, translator: &Translator) -> Result<RawPattern, GenericError> {
        let pattern = self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()))?;

        let raw_dims_values = translator.untranslateLineDims(&pattern.dims_values)?;
        let raw_dims_values: Vec<Vec<String>> = raw_dims_values.iter()
            .map(|raw_dim_values| raw_dim_values.split(",").map(|s| s.to_string()).collect())
            .collect();

        let raw_pattern = RawPattern::new(&pattern.identifier, &raw_dims_values, &pattern.density, &pattern.size);

        return Ok(raw_pattern);
    }

    pub fn asDagNode(&self) -> Result<&DagNode, GenericError> {
        return self.dag_node_representation.as_ref()
            .ok_or(GenericError::new("Could not get dag node representation", file!(), &line!()));
    }

    pub fn asDataPoint(&self) -> Result<&DataPoint, GenericError> {
        return self.data_point_representation.as_ref()
            .ok_or(GenericError::new("Could not get data point representation", file!(), &line!()));
    }
}
### src-tauri/src/model/identifier_representation.rs END ###

### src-tauri/src/model/mod.rs BEGIN ###
pub mod analysis;
pub mod io;
pub mod identifier_mapper;
pub mod identifier_representation;
### src-tauri/src/model/mod.rs END ###

### src-tauri/src/model/analysis/mod.rs BEGIN ###
pub mod metrics;
pub mod ordered_pair;
### src-tauri/src/model/analysis/mod.rs END ###

### src-tauri/src/model/analysis/ordered_pair.rs BEGIN ###
use std::hash::Hash;
use crate::database::pattern::Pattern;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OrderedPair <'a>{
    pub x: &'a Pattern,
    pub y: &'a Pattern,
}

impl OrderedPair <'_> {
    pub fn new<'a>(x: &'a Pattern, y: &'a Pattern) -> OrderedPair<'a> {
        let mut pair = vec![x, y];
        pair.sort_by_key(|obj| obj.identifier);

        return OrderedPair {
            x: *pair.get(0).expect("Should have gotten first element of ordered pair"),
            y: *pair.get(1).expect("Should have gotten second element of ordered pair"),
        };
    }

    pub fn getOther(&self, current: &Pattern) -> &Pattern {
        if current == self.x { 
            return self.y;
        }
        return self.x;
    }

    pub fn get(&self) -> Vec<&Pattern> {
        return vec![self.x, self.y];
    }
}
### src-tauri/src/model/analysis/ordered_pair.rs END ###

### src-tauri/src/model/analysis/metrics/coordinates.rs BEGIN ###
#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::{DMatrix, SVD};
use ndarray::{IxDynImpl, Dim, ArrayD, Array};
use rayon::{iter::IndexedParallelIterator, prelude::{IntoParallelRefIterator, ParallelIterator}};
use crate::{common::generic_error::GenericError, model::identifier_mapper::IdentifierMapper};
use super::{metric::Metric, distances::DistancesTrait};

pub struct Coordinates {
    value: HashMap<u32, (f64, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, (f64, f64)>> for Coordinates{
    fn get(&self) -> &HashMap<u32, (f64, f64)> {
        return &self.value;
    }
}

impl Coordinates {
    pub fn new<T: DistancesTrait>(distances: &T) -> Result<Coordinates, GenericError>{
        println!("  Coordinates...");
        return Ok(
            Coordinates { 
                value: Coordinates::calculate(distances)?,
            }
        );
    }

    fn printMatrix(matrix: &DMatrix<f64>){
        println!("Printing matrix:");
        for i in 0..matrix.nrows(){
            for j in 0..matrix.ncols(){
                print!("{:.2} ", matrix[(i, j)]);
            }
            println!("");
        }
        println!("");
    }

    fn buildDissimilarityMatrix<T: DistancesTrait>(distances: &T, n: usize) -> Result<DMatrix<f64>, GenericError> {
        let size: Vec<usize> = vec![n, n];
        let distance_matrix: Arc<Mutex<ArrayD<f64>>> = Arc::new(Mutex::new(Array::zeros(Dim(size.clone())).into_dyn()));

        let mut visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        visible_identifiers.sort();
        let visible_identifiers2: Vec<u32> = visible_identifiers.clone();
        
        let distances: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(distances.get().clone()));
        visible_identifiers.par_iter().enumerate().try_for_each(|(i, &identifier_1)| -> Result<(), GenericError> {
            
            for (j, identifier_2) in visible_identifiers2.iter().enumerate(){
                let distances_lock = distances.lock()
                    .map_err(|_| GenericError::new("Error while getting distances thread lock", file!(), &line!()))?;
                
                let mut distance: f64 = 0.0;
                if identifier_1 != *identifier_2 {
                    distance = distances_lock.get(&identifier_1)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_1), file!(), &line!()))?
                    .get(identifier_2)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_2), file!(), &line!()))?.clone();
                }
                
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                
                let mut distance_matrix_lock = distance_matrix.lock()
                    .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                let matrix_value = distance_matrix_lock.get_mut(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;
                
                *matrix_value = distance;
            }

            return Ok(());
        })?;

        let distance_matrix = distance_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();
        
        let mut dissimilarity_matrix = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                let matrix_value = distance_matrix.get(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;

                dissimilarity_matrix[(i, j)] = *matrix_value;
            }
        }

        return Ok(dissimilarity_matrix);
    }

    fn mds(dissimilarity_matrix: DMatrix<f64>, dimensions: usize) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        // Returns a hashmap of the points in the new space, the indices DO NOT represent the identifiers
        // dbg!(&dissimilarity_matrix);
        let mut m = dissimilarity_matrix.map(|x| -0.5 * x.powi(2));

        // double centre the rows/columns
        let row_means = m.row_mean();
        let col_means = m.column_mean();
        let total_mean = row_means.mean();

        for i in 0..m.nrows() {
            for j in 0..m.ncols() {
                m[(i, j)] += total_mean - row_means[i] - col_means[j];
            }
        }

        // dbg!(&m);

        // take the SVD of the double centred matrix, and return the
        // points from it
        let svd = SVD::new(m, true, true);
        let eigen_values = svd.singular_values.map(|x| x.sqrt());

        let u = svd.u
            .ok_or(GenericError::new("Error getting U matrix from SVD", file!(), &line!()))?;

        let mut result = DMatrix::zeros(u.nrows(), dimensions);
        // dbg!(&eigen_values);
        // dbg!(&result);

        for i in 0..u.nrows() {
            for j in 0..dimensions {
                result[(i, j)] = u[(i, j)] * eigen_values[j];
            }
        }

        // Convert result to hashmap
        let n_rows = result.nrows();
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n_rows {
            let x = result[(i, 0)];
            let y = result[(i, 1)];
            xys.insert(i as u32, (x, y));
        }

        return Ok(xys);
    }

    fn calculate<T: DistancesTrait>(distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if distances.get().len() == 0{ // Only one datapoint, no need to calculate MDS
            let mut xys = HashMap::new();
            xys.insert(1, (0.0, 0.0));
            return Ok(xys);
        }

        println!("  Applying Multi Dimensional Scaling...");
        let n: usize = distances.get().len();
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n)?;
        let xys: HashMap<u32, (f64, f64)> = Coordinates::mds(dissimilarity_matrix, 2)?;

        let mut visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        visible_identifiers.sort();

        let mut result: HashMap<u32, (f64, f64)> = HashMap::new();
        for entry in xys.iter(){
            let i = entry.0;
            let identifier = visible_identifiers.get(*i as usize)
                .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?;

            result.insert(*identifier, *entry.1);
        }

        return Ok(result);
    }
}


### src-tauri/src/model/analysis/metrics/coordinates.rs END ###

### src-tauri/src/model/analysis/metrics/distances.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};
use crate::{common::{generic_error::GenericError, progress_bar}, database::{pattern::Pattern, subtensor::Subtensor, tensor::Tensor}, model::{analysis::ordered_pair::OrderedPair, identifier_mapper::IdentifierMapper}};
use super::{intersections_predictions::IntersectionsPredictions, metric::Metric};

pub trait DistancesTrait {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>;
}

pub struct DistancesView {
    view: HashMap<u32, HashMap<u32, f64>>,
    mapping: HashMap<u32, u32>,

}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for DistancesView{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

impl DistancesTrait for DistancesView {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.view;
    }
}

#[allow(non_camel_case_types)]
impl DistancesView {
    fn new(view: &HashMap<u32, HashMap<u32, f64>> , mapping: HashMap<u32, u32>) -> DistancesView{
        return DistancesView { 
            view: view.clone(),
            mapping: mapping,
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Distances{
    value: HashMap<u32, HashMap<u32, f64>>, 
}

impl DistancesTrait for Distances {
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, HashMap<u32, f64>>> for Distances{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>> {
        return &self.value;
    }
}

impl Distances{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<Distances, GenericError>{
                
        println!("  Distances...");
        return Ok(
            Distances { 
                value: Distances::calculate(identifier_mapper, tensor, intersections_predictions, visible_identifiers)?,
            }
        );
    }

    fn calculatePairRss(tensor: &Tensor, intersections_predictions: &IntersectionsPredictions, pair: &OrderedPair) 
            -> Result<(HashMap<u32, f64>, f64), GenericError> {

        let intersections_predictions = intersections_predictions.get();
        let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
        let mut intersection_rss = 0.0;
        let mut seen_intersection_indices: HashSet<&Vec<usize>> = HashSet::new();

        for pattern in pair.get(){
            let mut untouched_rss = 0.0;

            for index in pattern.indices.iter(){
                let actual_value = *tensor.dims_values.get(index.as_slice())
                    .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;
    
                let possible_overlapper = intersections_predictions.get(index);

                match possible_overlapper {
                None => { }, // No overlapper
                Some(possible_overlapper) => {
                    if *possible_overlapper == pair.getOther(pattern) { // Here there is intersection with the pair
                        if seen_intersection_indices.contains(index){ continue; } // Avoid double counting

                        let overlapper = possible_overlapper;
                        let overlapper_contribution = (actual_value - overlapper.density).powi(2);
                        
                        intersection_rss += overlapper_contribution;
                        seen_intersection_indices.insert(index);
                        continue;
                    }}
                }

                untouched_rss += (actual_value - pattern.density).powi(2);
            }

            untouched_rss_s.insert(pattern.identifier, untouched_rss);
        }

        return Ok((untouched_rss_s, intersection_rss));
    }

    fn getXUYDimsValues(x: &Pattern, y: &Pattern) -> Result<Vec<Vec<usize>>, GenericError> {
        let mut xuy_dims_values: Vec<Vec<usize>> = vec![Vec::new(); x.dims_values.len()];

        for (i, dim_values) in x.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;

            for value in dim_values{
                // if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        for (i, dim_values) in y.dims_values.iter().enumerate(){
            let xuy_dim_values = xuy_dims_values.get_mut(i)
                .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?;

            for value in dim_values{
                if xuy_dim_values.contains(value){ continue; }
                xuy_dim_values.push(*value);
            }
        }

        return Ok(xuy_dims_values);
    }
    
    fn getXUY(tensor:&Tensor, x: &Pattern, y: &Pattern) -> Result<Subtensor, GenericError>{
        let xuy_dims_values = Distances::getXUYDimsValues(x, y)?;
        let xuy = Subtensor::new(tensor, &xuy_dims_values); // Expensive
        return xuy;
    }

    fn getCoveredXUYRss(tensor:&Tensor, xuy: &Subtensor, x: &Pattern, y: &Pattern) -> Result<f64, GenericError>{
        let mut xuy_rss = 0.0;

        let interested_indices: Vec<Vec<usize>> = x.union(y);
        for index in interested_indices.iter(){
            let actual_value = *tensor.dims_values.get(index.as_slice())
                .ok_or(GenericError::new("Index not found", file!(), &line!()))? as f64;

            xuy_rss += (actual_value - xuy.density).powi(2);
        }   

        return Ok(xuy_rss);
    }

    fn normalize(x: &Pattern, y: &Pattern, raw_distance: &f64) -> Result<f64, GenericError>{
        let mut dimensions_sum_area = 1.0;
        for i in 0..x.dims_values.len() {
            let ith_x_dimension_size = x.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            let ith_y_dimension_size = y.dims_values.get(i)
            .ok_or(GenericError::new(&format!("Index {} not found", i), file!(), &line!()))?
            .len() as f64;

            dimensions_sum_area *= ith_x_dimension_size + ith_y_dimension_size;
        }
        let mut xuy_reference_density = (x.size as f64 * x.density) + (y.size as f64 * y.density);
        xuy_reference_density /= dimensions_sum_area;

        let mut denominator = x.size as f64 * (x.density - xuy_reference_density).powi(2);
        denominator += y.size as f64 * (y.density - xuy_reference_density).powi(2);  

        let normalized_distance = raw_distance / denominator;
        return Ok((10000.0 * normalized_distance).round() / 10000.0);
    }

    fn insertIntoDistancesMatrix(distances: &mut HashMap<u32, HashMap<u32, f64>>, x: &Pattern, y: &Pattern, distance: &f64)
            -> Result<(), GenericError>{

        if !distances.contains_key(&x.identifier){
            distances.insert(x.identifier, HashMap::new());
        }

        let distances_from_x = distances.get_mut(&x.identifier)
            .ok_or(GenericError::new(&format!("Distances from {} not found", &x.identifier), file!(), &line!()))?;

        distances_from_x.insert(y.identifier, *distance);

        return Ok(());
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, intersections_predictions: &IntersectionsPredictions, visible_identifiers: &Vec<u32>) 
            -> Result<HashMap<u32, HashMap<u32, f64>>, GenericError>{
        // 58s, 30s, 46s, 39s, 37s, 19s, 16s, 5s, 3s
        let distances = Arc::new(Mutex::new(HashMap::new()));
        let visible_patterns: Result<Vec<&Pattern>, GenericError> = visible_identifiers.iter()
                .map(|identifier| identifier_mapper.getRepresentation(identifier)?.asPattern())
                .collect();

        let visible_patterns = visible_patterns?;

        let mut total_distances = 0;
        if visible_identifiers.len() > 1 {
            total_distances = (visible_identifiers.len().pow(2) as u32 / 2) - visible_identifiers.len() as u32
        }

        let total_distances = total_distances as u64;
        let bar = progress_bar::new(total_distances, "  Calculated distances");

        visible_patterns.par_iter().enumerate().try_for_each(|(row, x)| 
                -> Result<(), GenericError>{

            if row != 0 {
                for (col, y) in visible_patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let xuy = Distances::getXUY(tensor, x, y)?;
                        let covered_xuy_rss = Distances::getCoveredXUYRss(tensor, &xuy, x, y)?;
                        
                        let pair = OrderedPair::new(x, y);
                        let (untouched_rss, x_y_intersection_rss) = Distances::
                            calculatePairRss(tensor, intersections_predictions, &pair)?;
                        
                        let untouched_rss_x = *untouched_rss.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &x.identifier), file!(), &line!()))?;

                        let untouched_rss_y = *untouched_rss.get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Untouched RSS for pattern {} not found", &y.identifier), file!(), &line!()))?;
                        
                        let raw_distance = covered_xuy_rss - untouched_rss_x - untouched_rss_y - x_y_intersection_rss;
                        
                        let normalized_distance = Distances::normalize(x, y, &raw_distance)?;
                        
                        let mut distances = distances.lock()
                            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances, &x, &y, &normalized_distance)?;
                        Distances::insertIntoDistancesMatrix(&mut distances, &y, &x, &normalized_distance)?;
                        bar.inc(1);
                    }
                }
            }

            return Ok(());
        })?;
    
        bar.finish();
        let distances = distances.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();

        return Ok(distances);
    }

    pub fn getView(&self, identifier_mapper: &IdentifierMapper, identifiers: &Vec<u32>) -> Result<DistancesView, GenericError>{
        let mut patterns: Vec<&Pattern> = Vec::new();
        // Maps the identifier of the pattern INSIDE the view to the REAL identifier
        let mut mapping: HashMap<u32, u32> = HashMap::new();

        for (i, real_identifier) in identifiers.iter().enumerate(){
            let view_identifier = (i + 1) as u32; // Because i starts at zero
            let representation = identifier_mapper.getRepresentation(real_identifier)?;
            let pattern = representation.asPattern()?;
            
            patterns.push(pattern);
            mapping.insert(view_identifier, *real_identifier);
        }

        let mut distances_view: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        for (row, x) in patterns.iter().enumerate(){
            if row != 0 {
                for (col, y) in patterns.iter().enumerate() { 
                    if col < row { // Iterate triangularly
                        let distance = self.value.get(&x.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} not found", &x.identifier), file!(), &line!()))?
                            .get(&y.identifier)
                            .ok_or(GenericError::new(&format!("Distance from {} to {} not found", &x.identifier, &y.identifier), file!(), &line!()))?;

                        Distances::insertIntoDistancesMatrix(&mut distances_view, &x, &y, distance)?;    
                        Distances::insertIntoDistancesMatrix(&mut distances_view, &y, &x, distance)?;    
                    }
                }
            }
        }

        return Ok(DistancesView::new(&distances_view, mapping));
    }
}
### src-tauri/src/model/analysis/metrics/distances.rs END ###

### src-tauri/src/model/analysis/metrics/empty_model_rss.rs BEGIN ###
use crate::database::tensor::Tensor;

use super::metric::Metric;

#[derive(Default)]
pub struct EmptyModelRss{
    value: f64, 
}

#[allow(non_camel_case_types)]
impl Metric<f64> for EmptyModelRss{
    fn get(&self) -> &f64 {
        return &self.value;
    }
}

impl EmptyModelRss{
    pub fn new(tensor: &Tensor) -> EmptyModelRss {
        println!("  Empty model RSS...");
        return EmptyModelRss { value: EmptyModelRss::calculate(tensor) }
    }

    fn calculate(tensor: &Tensor) -> f64{
        let mut rss = 0.0;
        for actual_value in tensor.dims_values.iter(){
            rss += (actual_value - tensor.density).powi(2);
        }
        return rss;
    }
}
    
    
### src-tauri/src/model/analysis/metrics/empty_model_rss.rs END ###

### src-tauri/src/model/analysis/metrics/full_model_rss.rs BEGIN ###
// #![allow(non_snake_case)]
// use std::{collections::{HashMap, HashSet, LinkedList}, time::Instant};
// use indicatif::{ProgressBar, ProgressStyle};
// use ndarray::{Dim, IxDynImpl, indices, Dimension, ShapeBuilder};
// use crate::{tensor::tensor::Tensor, pattern::{pattern_mapper::PatternMapper, pattern::Pattern}, utils::{ordered_pair::OrderedPair}};
// use super::{metric::Metric, intersections_predictions::IntersectionsPredictions};

// pub struct FullModelRss{
//     pub untouched_rss_s: HashMap<u32, f64>,
//     pub intersection_rss_s: HashMap<OrderedPair, f64>, // {Overlapped, {overlapper, intersection_rss}}
//     pub model_edges_rss: f64,

//     value: f64,
// }

// #[allow(non_camel_case_types)]
// impl Metric<f64> for FullModelRss{
//     fn get(&self) -> &f64{
//         return &self.value;
//     }
// }

// impl FullModelRss{
//     pub fn new(pattern_mapper: &PatternMapper, tensor: &Tensor, intersections_predictions: &IntersectionsPredictions) ->  FullModelRss{
//         let all_rss = FullModelRss::calculateAll(pattern_mapper, tensor, intersections_predictions);
        
//         return FullModelRss { 
//             untouched_rss_s: all_rss.0,
//             intersection_rss_s: all_rss.1,
//             model_edges_rss: all_rss.2,
//             value: all_rss.3,
//         }
//     }

//     fn getEdgeIndices(pattern_mapper: &PatternMapper, tensor: &Tensor) ->  Vec<Dim<IxDynImpl>>{
//         let mut non_edge_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();
//         let mut edge_indices: Vec<Dim<IxDynImpl>> = Vec::new();
        
//         for pattern in pattern_mapper.getPatterns(){
//             for index in pattern.indices.iter(){
//                 non_edge_indices.insert(index.clone());
//             }
//         }

//         for index in tensor.dims_values.indexed_iter(){
//             let index = index.0;
//             if non_edge_indices.contains(&index) { continue ;}
//             edge_indices.push(index);
//         }

//         return edge_indices;
//     }

//     fn calculatePatternRss(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>){

//         let intersections_predictions: &HashMap<Dim<IxDynImpl>, &Pattern> = intersections_predictions.get();                                                                                                          
//         let mut untouched_rss_s: HashMap<u32, f64> = HashMap::new();
//         let mut intersection_rss_s: HashMap<OrderedPair, f64> = HashMap::new();

//         for pattern in pattern_mapper.getPatterns(){
//             let mut pattern_untouched_rss = 0.0;

//             for index in pattern.indices.iter(){
//                 let actual_value = tensor.dims_values.get(index).unwrap();

//                 let possible_overlapper = intersections_predictions.get(&index);
//                 if possible_overlapper.is_some() { // Index IS touched by another pattern
//                     let overlapper = *possible_overlapper.unwrap();
//                     if overlapper.density == pattern.density{ // Current pattern was determined to be the overlapper previously
//                         continue;
//                     }

                    
//                     let overlapper_contribution = (actual_value - overlapper.density).powi(2);

//                     let pair = OrderedPair::new(&pattern.identifier, &overlapper.identifier);
//                     let intersection_rss = intersection_rss_s.get_mut(&pair);
                    
//                     if intersection_rss.is_some(){
//                         *intersection_rss.unwrap() += overlapper_contribution; // This pair has a previous RSS value, sum new
//                     }else{
//                         intersection_rss_s.insert(pair, overlapper_contribution); // This pair hasnt a previous RSS value
//                     }
                    
//                     continue;
//                 }
                
//                 // Index IS NOT touched by another pattern
//                 pattern_untouched_rss += (actual_value - pattern.density).powi(2);
//             }

//             untouched_rss_s.insert(pattern.identifier, pattern_untouched_rss);
//         }

//         return (untouched_rss_s, intersection_rss_s);
//     }

//     fn calculateModelEdgesRss(tensor: &Tensor, edge_indices: Vec<Dim<IxDynImpl>>) -> f64{
//         let mut model_edges_rss = 0.0;
//         for edge_index in edge_indices{
//             let actual_value = tensor.dims_values.get(edge_index).unwrap();
//             model_edges_rss += (actual_value - tensor.density).powi(2);
//         }
//         return model_edges_rss;
//     }

//     fn calculateFullModelRss(untouched_rss_s: &HashMap<u32, f64>,
//                             intersection_rss_s: &HashMap<OrderedPair, f64>,
//                             model_edges_rss: &f64) -> f64 {

//         let mut full_model_rss = *model_edges_rss;

//         for (_, untouched_rss) in untouched_rss_s {
//             full_model_rss += *untouched_rss;
//         }

//         for (_, intersection_rss) in intersection_rss_s {
//             full_model_rss += *intersection_rss;
//         }

//         return full_model_rss;
//     }

//     fn calculateAll(pattern_mapper: &PatternMapper, tensor: &Tensor, 
//         intersections_predictions: &IntersectionsPredictions) 
//         -> (HashMap<u32, f64>, HashMap<OrderedPair, f64>, f64, f64){
        
//         let (untouched_rss_s, intersection_rss_s) = 
//         FullModelRss::calculatePatternRss(
//             pattern_mapper, 
//             tensor,
//             intersections_predictions);
        
//         println!("  Model edges RSS...");
//         let model_edges_rss: f64 = FullModelRss::calculateModelEdgesRss(
//             tensor, 
//             FullModelRss::getEdgeIndices(pattern_mapper, tensor));

//         let full_model_rss: f64 = FullModelRss::calculateFullModelRss(
//             &untouched_rss_s, 
//             &intersection_rss_s, 
//             &model_edges_rss);

//         return (untouched_rss_s, intersection_rss_s, model_edges_rss, full_model_rss);
//     }
// }
    
    
### src-tauri/src/model/analysis/metrics/full_model_rss.rs END ###

### src-tauri/src/model/analysis/metrics/intersections_predictions.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::{HashMap, HashSet}, sync::{Mutex, Arc}};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::{database::pattern::Pattern, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper}, common::generic_error::GenericError};

pub struct IntersectionsPredictions<'a>{
    value: HashMap<Vec<usize>, &'a Pattern>,
}

#[allow(non_camel_case_types)]
impl<'a> Metric<HashMap<Vec<usize>, &'a Pattern>> for IntersectionsPredictions<'a> {
    fn get(&self) -> &HashMap<Vec<usize>, &'a Pattern> {
        return &self.value;
    }
}

impl IntersectionsPredictions<'_>{
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<IntersectionsPredictions<'a>, GenericError>{
        println!("  Intersections predictions...");
        return Ok(
            IntersectionsPredictions { 
                value: IntersectionsPredictions::calculate(identifier_mapper)?,
            }
        );
    }
    
    fn calculate<'a>(identifier_mapper: &'a IdentifierMapper) -> Result<HashMap<Vec<usize>, &'a Pattern>, GenericError> {

        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new();
        for identifier_representation in identifier_mapper.getRepresentations(){
            let node = identifier_representation.asDagNode()?;
            overlappings.insert(node.identifier, node.overlappings.clone());
        }

        let intersections_predictions: Arc<Mutex<HashMap<Vec<usize>, &Pattern>>> = Arc::new(Mutex::new(HashMap::new()));

        overlappings.par_iter().try_for_each(|(overlapped, overlappers)| -> Result<(), GenericError>{
            let overlapped: &Pattern = identifier_mapper.getRepresentation(overlapped)?.asPattern()?;

            for overlapper in overlappers{
                let overlapper: &Pattern = identifier_mapper.getRepresentation(overlapper)?.asPattern()?;
                let intersection_indices: Vec<Vec<usize>> = overlapped.intersection(overlapper);

                for intersection_index in intersection_indices {
                    let mut intersections_predictions_lock = intersections_predictions
                        .lock()
                        .map_err(|_| GenericError::new("Could not lock intersections predictions", file!(), &line!()))?;

                    let possible_previous_prediction = intersections_predictions_lock.get_mut(&intersection_index); // EXPENSIVE
                    match possible_previous_prediction{
                        None => {
                            intersections_predictions_lock.insert(intersection_index.clone(), overlapper);
                        }
                        Some(previous_prediction) => { // Multiple overlapping in one index

                            if overlapper.density > previous_prediction.density{ // Switch to current overlapper
                                *previous_prediction = overlapper;
                            }
                        }
                    };

                    
                }
            }

            return Ok(());
        })?;
            
        return Ok(
            intersections_predictions.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections predictions", file!(), &line!()))?
            .clone()
        );
    }
}
### src-tauri/src/model/analysis/metrics/intersections_predictions.rs END ###

### src-tauri/src/model/analysis/metrics/metric.rs BEGIN ###
pub trait Metric<T>{
    fn get(&self) -> &T;
}
### src-tauri/src/model/analysis/metrics/metric.rs END ###

### src-tauri/src/model/analysis/metrics/mod.rs BEGIN ###
pub mod metric;
pub mod rss_evolution;
pub mod empty_model_rss;
// pub mod full_model_rss;
pub mod intersections_predictions;
pub mod distances;
pub mod coordinates;
pub mod intersection;
### src-tauri/src/model/analysis/metrics/mod.rs END ###

### src-tauri/src/model/analysis/metrics/rss_evolution.rs BEGIN ###
use ndarray::{ArrayD, Dim, IxDynImpl};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::iter::Iterator;
use crate::common::generic_error::GenericError;
use crate::common::progress_bar;
use crate::database::pattern::Pattern;
use crate::{model::identifier_mapper::IdentifierMapper, database::tensor::Tensor};
use super::empty_model_rss::EmptyModelRss;
use super::intersection::intersections_indices::IntersectionsIndices;
use super::intersection::prediction_matrix::PredictionMatrix;
use super::intersection::untouched_delta_rss::{self, UntouchedDeltaRss};
use super::metric::Metric;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct RssEvolution{
    value: Vec<(u32, f64)>,
    truncated_value: Vec<(u32, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<Vec<(u32, f64)>> for RssEvolution{
    fn get(&self) -> &Vec<(u32, f64)> {
        return &self.value;
    }
}

impl RssEvolution{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, empty_model_rss: &EmptyModelRss, 
        patterns: &Vec<&Pattern>, prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) -> Result<RssEvolution, GenericError>{

        println!("  RssEvolution...");
        
        let rss_evolution = RssEvolution::calculate(identifier_mapper, tensor, 
            empty_model_rss, patterns, prediction_matrix, untouched_delta_rss, intersections_indices)?;
        return Ok(
            RssEvolution{
                value: rss_evolution.clone(),
                truncated_value: rss_evolution,
            }
        );
    }

    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    fn updateRssAtIndex(tensor_matrix: &ArrayD<f64>, total_rss: &f64, index: &Dim<IxDynImpl>, old_prediction: &f64, 
                        new_prediction: &f64, prediction_matrix: &mut PredictionMatrix) -> Result<f64, GenericError>{
        
        prediction_matrix.insert(index.clone(), *new_prediction);
        drop(prediction_matrix);

        let actual_value = tensor_matrix.get(index)
            .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

        let new_prediction_rss = RssEvolution::calculateRss(actual_value, new_prediction);
        let old_prediction_rss = RssEvolution::calculateRss(actual_value, old_prediction);

        let total_rss = total_rss - old_prediction_rss + new_prediction_rss;
        
        return Ok(total_rss);
    }

    fn updatePredictionMatrix(prediction_matrix: &mut PredictionMatrix, intersections_indices: &IntersectionsIndices,
                            candidate_pattern: &Pattern) -> Result<(), GenericError>{
        
        let all_intersections_indices = intersections_indices.getValue(&candidate_pattern.identifier);
        let all_intersections_indices = match all_intersections_indices{
            None => { return Ok(()); } // No intersection
            Some(all_intersections_indices) => { all_intersections_indices },
        };
            
        for (_, intersection_indices) in all_intersections_indices {
            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getMutValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let max_prediction = f64::max(candidate_pattern.density, *previous_prediction);

                if max_prediction > *previous_prediction { // Then change to the new prediction
                    *previous_prediction = max_prediction;
                }
            }
        }

        return Ok(());
    }

    fn calculateCandidateModelRss(current_model_rss: &f64, candidate_pattern: &Pattern,
            tensor: &Tensor,
            identifier_mapper: &IdentifierMapper,
            untouched_delta_rss: &UntouchedDeltaRss,
            prediction_matrix: &mut PredictionMatrix,
            intersections_indices: &IntersectionsIndices,
            seen_candidates: &Vec<u32>) -> Result<f64, GenericError>{

        // let prediction_matrix = prediction_matrix.get();
        // let untouched_delta_rss = untouched_delta_rss.get();
        // let intersections_indices = intersections_indices.get();

        let mut candidate_model_rss = *current_model_rss + untouched_delta_rss.getValue(&candidate_pattern.identifier)
            .ok_or(GenericError::new(
                &format!("Untouched delta rss for pattern {} not found", candidate_pattern.identifier), file!(), &line!()))?
            .1;
            
        let candidate_intersections = intersections_indices.getValue(&candidate_pattern.identifier);
        let candidate_intersections = match candidate_intersections {
            None => { return Ok(candidate_model_rss); } // No intersection
            Some(candidate_intersections) => { candidate_intersections },
        };
        
        // Here we can also have indices with no intersection
        let candidate_prediction = candidate_pattern.density;

        for (intersector, intersection_indices) in candidate_intersections {
            // First deal with intersection indices
            let ignore_intersector = !seen_candidates.contains(intersector);
            
            let intersector_prediction = identifier_mapper
                .getRepresentation(intersector)?
                .asPattern()?.density;

            for intersection_index in intersection_indices {
                let previous_prediction = prediction_matrix.getValue(intersection_index)
                    .ok_or(GenericError::new(&format!("Index {:?} not found", intersection_index), file!(), &line!()))?;

                let previous_prediction_copy = previous_prediction.clone();

                if ignore_intersector == true { // Intersector is not in the submodel, act as if the candidate is not intersected
                    candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                        &candidate_model_rss, 
                        intersection_index, 
                        &previous_prediction_copy, 
                        &candidate_prediction,
                        prediction_matrix)?;
                    
                    continue;
                }

                let mut max_prediction = f64::max(intersector_prediction, candidate_prediction);
                max_prediction = f64::max(max_prediction, *previous_prediction);

                candidate_model_rss = RssEvolution::updateRssAtIndex(&tensor.dims_values,
                    &candidate_model_rss, 
                    intersection_index, 
                    &previous_prediction_copy, 
                    &max_prediction,
                    prediction_matrix)?;
            }
        }
        return Ok(candidate_model_rss);
    }

    fn calculate(identifier_mapper: &IdentifierMapper, tensor:&Tensor, empty_model_rss: &EmptyModelRss, patterns: &Vec<&Pattern>,
        prediction_matrix: &mut PredictionMatrix, untouched_delta_rss: &UntouchedDeltaRss, 
        intersections_indices: &IntersectionsIndices) 
        -> Result<Vec<(u32, f64)>, GenericError>{
        
        let pattern_nb = patterns.len();

        let mut current_model_rss = *empty_model_rss.get();
        let mut rss_evolution: Vec<(u32, f64)> = vec![(0, current_model_rss)];
        let mut seen_candidates: Vec<u32> = vec![];
        
        let bar = progress_bar::new(pattern_nb as u64, "    Submodels calculated");
        for (_, pattern) in patterns.iter().enumerate(){

            let candidate_model_rss = RssEvolution::calculateCandidateModelRss(
                &current_model_rss, 
                pattern, 
                tensor, 
                identifier_mapper, 
                &untouched_delta_rss, 
                prediction_matrix,
                &intersections_indices, 
                &seen_candidates)?;

            current_model_rss = candidate_model_rss;
            rss_evolution.push((pattern.identifier, current_model_rss));
            seen_candidates.push(pattern.identifier);
            RssEvolution::updatePredictionMatrix(prediction_matrix, &intersections_indices, pattern)?;
            bar.inc(1);
        }

        bar.finish();
        return Ok(rss_evolution);
    }

    pub fn truncate(&mut self, new_size: &u32){
        let full_rss_evolution: Vec<(u32, f64)> = self.value.clone();
        
        // retain the first k + 1 elements, where k is the new size
        let truncated_rss_evolution: Vec<(u32, f64)> = full_rss_evolution.into_iter()
            .take(*new_size as usize + 1)
            .map(|(pattern_identifier, rss)| (pattern_identifier, rss))
            .collect();

        self.truncated_value = truncated_rss_evolution;
    }

    pub fn getTruncated(&self) -> &Vec<(u32, f64)>{
        return &self.truncated_value;
    }
}
### src-tauri/src/model/analysis/metrics/rss_evolution.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersections_indices.rs BEGIN ###
use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsIndices{
    value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>,
}

impl Metric<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>> for IntersectionsIndices{
    fn get(&self) -> &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>
    {
        return &self.value;
    }
}

impl IntersectionsIndices{
    pub fn new(value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>) -> IntersectionsIndices{
        return IntersectionsIndices{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersections_indices.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersections_percentages.rs BEGIN ###
use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsPercentages{
    value: HashMap<u32, HashMap<u32, f64>>,
}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for IntersectionsPercentages{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>{
        return &self.value;
    }
}

impl IntersectionsPercentages {
    pub fn new(value: HashMap<u32, HashMap<u32, f64>>) -> IntersectionsPercentages{
        return IntersectionsPercentages{
            value: value,
        };
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersections_percentages.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/intersection_metrics.rs BEGIN ###
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use ndarray::{Dim, IxDynImpl};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tauri::utils::pattern;

use crate::{common::generic_error::GenericError, database::{pattern::Pattern, tensor::Tensor}, model::identifier_mapper::IdentifierMapper};

use super::{intersections_percentages::{self, IntersectionsPercentages}, intersections_indices::{self, IntersectionsIndices}, prediction_matrix::PredictionMatrix, untouched_delta_rss::UntouchedDeltaRss};

pub struct IntersectionMetrics {}

impl IntersectionMetrics{
    fn calculateRss(actual_value: &f64, prediction: &f64) -> f64{
        return (actual_value - prediction).powi(2);
    }

    pub fn calculate(tensor: &Tensor, patterns: &Vec<&Pattern>, identifier_mapper: &IdentifierMapper) 
            -> Result<(PredictionMatrix, UntouchedDeltaRss, IntersectionsIndices, IntersectionsPercentages), GenericError>{

        let prediction_matrix: HashMap<Dim<IxDynImpl>, f64> = HashMap::new();
        let untouched_rss_s: HashMap<u32, (u32, f64)>= HashMap::new();
        let intersections_indices: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>> = HashMap::new();
        let intersections_percentages: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        let mut overlappings: HashMap<u32, HashSet<u32>> = HashMap::new(); // This is a symmetric relation

        for pattern in patterns {
            let node = identifier_mapper.getRepresentation(&pattern.identifier)?.asDagNode()?;

            for other_pattern in patterns {
                let other_node = identifier_mapper.getRepresentation(&other_pattern.identifier)?.asDagNode()?;
                
                if node.overlappings.contains(&other_node.identifier) || other_node.overlappings.contains(&node.identifier) {
                    overlappings.entry(pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(other_pattern.identifier);

                    overlappings.entry(other_pattern.identifier)
                        .or_insert(HashSet::new())
                        .insert(pattern.identifier);
                }
            }
        }

        let prediction_matrix: Arc<Mutex<HashMap<Dim<IxDynImpl>, f64>>> = Arc::new(Mutex::new(prediction_matrix));
        let untouched_rss_s: Arc<Mutex<HashMap<u32, (u32, f64)>>> = Arc::new(Mutex::new(untouched_rss_s));
        let intersections_indices: Arc<Mutex<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>>> = Arc::new(Mutex::new(intersections_indices));
        let intersections_percentages: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(intersections_percentages));

        patterns.par_iter().try_for_each(|pattern| -> Result<(), GenericError> {

            let mut pattern_intersections: HashMap<u32, Vec<Dim<IxDynImpl>>> = HashMap::new();
            let MAX_PATTERN_INTERSECTIONS = 6;
            let mut pattern_intersections_percentages: HashMap<u32, f64> = HashMap::new();
            let mut all_intersection_indices: HashSet<Dim<IxDynImpl>> = HashSet::new();

            let self_overlappings = overlappings.get(&pattern.identifier);

            for other_pattern in patterns {
                if pattern.identifier == other_pattern.identifier { continue; } // Itself
                
                match self_overlappings {
                    None => continue, // This pattern doesnt overlap any other pattern
                    Some(self_overlappings) => {
                        if !self_overlappings.contains(&other_pattern.identifier) { continue; } // These two do not overlap
                    },
                };

                // Here we know that pattern and other_pattern overlap

                let intersection_indices: Vec<Dim<IxDynImpl>> = pattern.intersection(other_pattern)
                    .into_iter()
                    .map(|index| Dim(index))
                    .collect();

                for index in intersection_indices.iter() {
                    all_intersection_indices.insert(index.clone());
                    prediction_matrix.lock()
                        .as_mut()
                        .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
                        .insert(index.clone(), tensor.density);
                }

                if !intersection_indices.is_empty() { // There are intersections between pattern and other_pattern
                    let intersection_percentage = intersection_indices.len() as f64 / pattern.size as f64;
                    
                    pattern_intersections.insert(other_pattern.identifier, intersection_indices);
                    pattern_intersections_percentages.insert(other_pattern.identifier, intersection_percentage);
                }else{
                    unreachable!("There should be at least one intersection");
                }
            }

            if !pattern_intersections.is_empty(){ // This pattern has intersections with other patterns
                intersections_indices.lock()
                    .as_mut()
                    .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
                    .insert(pattern.identifier, pattern_intersections);
            }

            // if pattern_intersections_percentages.len() > MAX_PATTERN_INTERSECTIONS{
            //     // This truncates pattern_intersections_percentages up to (MAX_PATTERN_INTERSECTIONS - 1) elements
            //     // and inserts the sum of the excess elements on it after
            //     let mut sorted_pattern_intersections_indices: Vec<u32> = pattern_intersections_percentages.keys().cloned().collect();
            //     sorted_pattern_intersections_indices.sort_by(|a, b| { // Decreasing order, based on the intersection percentage
            //         pattern_intersections_percentages.get(b).partial_cmp(&pattern_intersections_percentages.get(a)).unwrap()});

            //     let excess_indices = sorted_pattern_intersections_indices.split_off(MAX_PATTERN_INTERSECTIONS - 1);
            //     let excess_percentages_sum = excess_indices.iter()
            //         .map(|index| pattern_intersections_percentages.get(index).unwrap())
            //         .sum::<f64>();

            //     // Retain in pattern_intersections_percentages only the entries in which the key is in sorted_pattern_intersections_indices
            //     pattern_intersections_percentages.retain(|key, _| sorted_pattern_intersections_indices.contains(key));
            //     pattern_intersections_percentages.insert(0, excess_percentages_sum);
            // }

            let total_intersection_percentage = all_intersection_indices.len() as f64 / pattern.size as f64;
            let untouched_percentage = 1.0 - total_intersection_percentage;
            if untouched_percentage < 0.0 || untouched_percentage > 1.0 {
                unreachable!("Untouched percentage should be between 0 and 1 but it is: {}", untouched_percentage);
            }
            pattern_intersections_percentages.insert(pattern.identifier, untouched_percentage);
            
            intersections_percentages.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
                .insert(pattern.identifier, pattern_intersections_percentages);

            let prediction = &pattern.density;
            let mut untouched_size: u32 = 0;
            let untouched_rss: f64 = pattern.indices_as_dims.clone().into_iter()
                .filter(|index| !all_intersection_indices.contains(index))
                .map(|index| -> Result<f64, GenericError> {
                    let actual_value = tensor.dims_values.get(&index)
                        .ok_or(GenericError::new(&format!("Index {:?} not found", index), file!(), &line!()))?;

                    let prediction_rss = IntersectionMetrics::calculateRss(actual_value, prediction);
                    let lambda_0_rss = IntersectionMetrics::calculateRss(actual_value, &tensor.density);
                    let delta_rss = prediction_rss - lambda_0_rss;

                    untouched_size += 1;
                    Ok(delta_rss)
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();

            untouched_rss_s.lock()
                .as_mut()
                .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
                .insert(pattern.identifier, (untouched_size, untouched_rss));

            return Ok(());
        })?;

        let prediction_matrix = PredictionMatrix::new(prediction_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock prediction matrix", file!(), &line!()))?
            .clone());

        let untouched_rss_s = UntouchedDeltaRss::new(untouched_rss_s.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock untouched rss", file!(), &line!()))?
            .clone());

        let intersections_indices = IntersectionsIndices::new(intersections_indices.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections indices", file!(), &line!()))?
            .clone());

        let pattern_intersections_percentages = IntersectionsPercentages::new(intersections_percentages.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Could not lock intersections percentages", file!(), &line!()))?
            .clone());

        return Ok((prediction_matrix, untouched_rss_s, intersections_indices, pattern_intersections_percentages));
    }
}
### src-tauri/src/model/analysis/metrics/intersection/intersection_metrics.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/mod.rs BEGIN ###
pub mod intersection_metrics;
pub mod intersections_indices;
pub mod prediction_matrix;
pub mod untouched_delta_rss;
pub mod intersections_percentages;
### src-tauri/src/model/analysis/metrics/intersection/mod.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/prediction_matrix.rs BEGIN ###
use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct PredictionMatrix{
    value: HashMap<Dim<IxDynImpl>, f64>,
}

impl Metric<HashMap<Dim<IxDynImpl>, f64>> for PredictionMatrix{
    fn get(&self) -> &HashMap<Dim<IxDynImpl>, f64>{
        return &self.value;
    }
}

impl PredictionMatrix{
    pub fn new(value: HashMap<Dim<IxDynImpl>, f64>) -> PredictionMatrix{
        return PredictionMatrix{
            value: value,
        };
    }

    pub fn insert(&mut self, index: Dim<IxDynImpl>, value: f64){
        self.value.insert(index, value);
    }

    pub fn getValue(&self, value: &Dim<IxDynImpl>) -> Option<&f64>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &Dim<IxDynImpl>) -> Option<&mut f64>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/prediction_matrix.rs END ###

### src-tauri/src/model/analysis/metrics/intersection/untouched_delta_rss.rs BEGIN ###
use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct UntouchedDeltaRss{
    value: HashMap<u32, (u32, f64)>,
}

impl Metric<HashMap<u32, (u32, f64)>> for UntouchedDeltaRss{
    fn get(&self) -> &HashMap<u32, (u32, f64)>{
        return &self.value;
    }
}

impl UntouchedDeltaRss{
    pub fn new(value: HashMap<u32, (u32, f64)>) -> UntouchedDeltaRss{
        return UntouchedDeltaRss{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&(u32, f64)>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut (u32, f64)>{
        return self.value.get_mut(value);
    }
}
### src-tauri/src/model/analysis/metrics/intersection/untouched_delta_rss.rs END ###

### src-tauri/src/model/io/mod.rs BEGIN ###
pub mod translator;
pub mod reader;
pub mod pattern_reader;
pub mod tensor_reader;
### src-tauri/src/model/io/mod.rs END ###

### src-tauri/src/model/io/pattern_reader.rs BEGIN ###
#![allow(non_snake_case)]

use crate::{database::pattern::Pattern, common::generic_error::GenericError};
use super::{translator::Translator, reader::Reader};

pub struct PatternReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
    file_has_densities: bool,
}

impl PatternReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> Result<PatternReader<'a>, GenericError> {
        let file_has_densities = Reader::fileHasDensity(&file_path)?;

        let instance = PatternReader {
            file_path: file_path.clone(),
            translator: translator,
            file_has_densities: file_has_densities,
        };

        return Ok(instance);
    }

    pub fn read<'a>(self) -> Result<Vec<Pattern>, GenericError>{
        let mut patterns: Vec<Pattern> = Vec::new();        
        let lines: Vec<String> = Reader::readRawLines(&self.file_path)?;
        
        for (i, line) in lines.iter().enumerate() {
            let mut density: f64 = 1.0;
            let mut line_dims: Vec<String> = Reader::preProcessLine(&line);

            if self.file_has_densities{
                density = line_dims.pop()
                    .ok_or(GenericError::new("Could not get density", file!(), &line!()))?
                    .parse::<f64>()
                    .map_err(|_| GenericError::new("Could not parse density to f64", file!(), &line!()))?
            }

            patterns.push(Pattern::new(
                i as u32 + 1, 
                self.translator.translateLineDims(&line_dims)?,
                density
            ));
        }

        return Ok(patterns);
    }
}


### src-tauri/src/model/io/pattern_reader.rs END ###

### src-tauri/src/model/io/reader.rs BEGIN ###
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
### src-tauri/src/model/io/reader.rs END ###

### src-tauri/src/model/io/tensor_reader.rs BEGIN ###
#![allow(non_snake_case)]
use std::{num::{ParseFloatError, ParseIntError}, collections::HashSet};

use debug_print::debug_println;
use ndarray::{ArrayD, Array, Dim, IxDynImpl};


use crate::{database::tensor::{Tensor, TensorType}, common::generic_error::GenericError};

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

    fn calculateDimension(&self) -> Result<u32, GenericError>{
        let first_line: String = Reader::readRawFirstLine(&self.file_path)?;
        let first_line: Vec<&str> = first_line
            .split(" ")
            .collect();

        return Ok(first_line.len() as u32 - 1);
    }

    fn getTensorSize(&self) -> Vec<usize>{
        return self.translator.getSize();
    }

    fn createEmptySizedMatrix(&self, size: &Vec<usize>) -> ArrayD<f64>{
        let matrix: ArrayD<f64> = Array::zeros(Dim(size.clone())).into_dyn();
        return matrix;
    }

    fn defineTensorType(&self, lines_dims: &Vec<Vec<String>>) -> Result<TensorType, GenericError> {
        let mut last_values: HashSet<u32> = HashSet::new();
        for line_dims in lines_dims {
            let last_value = line_dims.last()
                .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            
            let float_parse_test: Result<f64, ParseFloatError> = last_value.parse::<f64>(); // Tries to parse to float
            if float_parse_test.is_ok() { // Can be int or float
                
                let int_parse_test: Result<u32, ParseIntError> = last_value.parse::<u32>(); // Tries to parse to int
                if int_parse_test.is_err(){ // Then its number with floating points
                    // 100% of full fuzzy tensors will be identified here, but the others will be identified later (exaustively)
                    return Ok(TensorType::FullFuzzy);
                }
            }

            if float_parse_test.is_err() { // Then its a string (dimension)
                return Ok(TensorType::PartialImplicit); // There can be partial implicits where the last dimension IS NOT a string
            }

            // Here the tensor can be PartialImplicit, PartialExplicit or FullBoolean
            // Last value is for sure an integer

            let last_value = last_value.parse::<u32>()
                .map_err(|_| GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            if last_value != 0 && last_value != 1 {
                // 100% of partial implicits will be identified here, even if they pass the previous test
                return Ok(TensorType::PartialImplicit);
            }

            // Here the tensor can be PartialExplicit or FullBoolean
            // To determine which one it is, we need to iterate through all lines
            last_values.insert(last_value);
        }

        // Here the tensor can be PartialExplicit or FullBoolean
        if last_values.contains(&0) {
            // Then its full boolean
            return Ok(TensorType::FullBoolean);
        }

        return Ok(TensorType::PartialExplicit);
    }

    fn processFile(&self, tensor_size: &Vec<usize>) -> Result<(ArrayD<f64>, TensorType), GenericError>{
        debug_println!("    Processing tensor file ...");
        let lines = Reader::readRawLines(&self.file_path)?;
        let lines_dims: Vec<Vec<String>> = lines.into_iter()
            .map(|line| line.split(" ").map(|i| i.to_owned()).collect())
            .collect();

        // lines_dims[0] = {"a", "d", "g", "density"}

        let mut dims_values_matrix: ArrayD<f64> = self.createEmptySizedMatrix(tensor_size);
        let tensor_type = self.defineTensorType(&lines_dims)?;

        for line_dims in lines_dims{
            let mut line_dims =  line_dims;
            let mut density = 1.0;
            if tensor_type.hasDensity() {
                density = line_dims.pop()
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .parse::<f64>()
                    .map_err(|_| GenericError::new("Error parsing tensor file", file!(), &line!()))?;
            }
            
            let translated_line = self.translator.translateLineDims(&line_dims)?;
            let dims_values: Result<Vec<usize>, _> = translated_line
                .iter()
                .map(|v| v.get(0)
                    .ok_or_else(||
                        GenericError::new("Error parsing tensor file", file!(), &line!())))
                    .map(|res| res.map(|&v| v as usize))
                .collect();
                        
            let index: Dim<IxDynImpl> = Dim(dims_values?);
            let matrix_value = dims_values_matrix.get_mut(index)
                .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;

            *matrix_value = density;
        }
        debug_println!("    Done");
        return Ok((dims_values_matrix, tensor_type));
    }

    fn calculateDensity(&self, dims_values: &ArrayD<f64>, size: &Vec<usize>) -> f64{
        let mut area = 1.0;

        for dim_size in size{
            area *= *dim_size as f64;
        }

        return dims_values.sum() as f64 / area;
    }

    pub fn read(self) -> Result<Tensor, GenericError>{
        let dimension = self.calculateDimension()?;
        let tensor_size = self.getTensorSize();
        let (dims_values, tensor_type) = self.processFile(&tensor_size)?;
        let density = self.calculateDensity(&dims_values, &tensor_size);
        return Ok(
            Tensor::new(&self.file_path, dims_values, &tensor_size, &dimension, &density, tensor_type)
        );
    }
    
}
### src-tauri/src/model/io/tensor_reader.rs END ###

### src-tauri/src/model/io/translator.rs BEGIN ###

#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::common::generic_error::GenericError;

use super::reader::Reader;

#[derive(Default)]
pub struct Translator{
    translator: Vec<HashMap<String, u32>>,
    reversed_translator: Vec<HashMap<u32, String>>,
}

// Translation source path HAS to be a tensor

impl Translator {    

    pub fn new(translation_source_path: &String) -> Result<Translator, GenericError>{
        println!("Creating translator...");
        let translator = Translator::createTranslator(&translation_source_path)?;
        let reversed_translator = Translator::reverseTranslator(&translator);

        return Ok(
            Translator { 
                translator: translator,
                reversed_translator: reversed_translator,
            }
        );
    }
    
    fn createEmptyTranslator(sample_line:&String) -> Vec<HashMap<String, u32>>{
        let mut empty_translator: Vec<HashMap<String, u32>> = Vec::new();
    
        let sample_line: Vec<String> = sample_line.split(" ").map(|i| i.to_owned()).collect();

        let dimensions_nb = sample_line.len() - 1;
        
        for _ in 0..dimensions_nb{
            empty_translator.push(HashMap::new());
        }
    
        return empty_translator;
    }

    fn createTranslator(translation_source_path: &String) -> Result<Vec<HashMap<String, u32>>, GenericError> {
        let lines = Reader::readRawLines(&translation_source_path)?;
        let sample_line = lines.get(0)
            .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;

        let mut translator: Vec<HashMap<String, u32>> = Translator::createEmptyTranslator(sample_line);
        // let file_has_density: bool = AmbientReader::fileHasDensity(&lines);

        for line in lines{
            let dimensions: Vec<String> = line.split(" ")
                .map(|i| i.to_owned())
                .collect();
            
            for (i, dimension) in dimensions.iter().enumerate(){
                if i == dimensions.len() - 1 { // On the last 'dimension' of this line
                    break;
                }
                
                let dim_translator: &mut HashMap<String, u32> = translator.get_mut(i)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?;
    
                let values: Vec<String> = dimension.split(",")
                    .map(|i| i.to_owned())
                    .collect();
    
                // dbg!(&values);
                
                for value in values{
                    let translated_value = dim_translator.get(&value);
    
                    if translated_value.is_none(){ // Key does not exist
                        dim_translator.insert(value, dim_translator.len() as u32); // Starts from 0
                    }
                }
            }
        }
    
        return Ok(translator);
    }

    fn reverseTranslator(translator: &Vec<HashMap<String, u32>>) -> Vec<HashMap<u32, String>> {
        let mut reversed_translator: Vec<HashMap<u32, String>> = Vec::new();
    
        for dim_translator in translator{
            let mut reversed_dim_translator: HashMap<u32, String> = HashMap::new();
    
            for (key, value) in dim_translator{
                reversed_dim_translator.insert(*value, key.to_owned());
            }
            reversed_translator.push(reversed_dim_translator);
        }
    
        return reversed_translator;
    }

    pub fn translateLineDims(&self, line_dims: &Vec<String>) -> Result<Vec<Vec<usize>>, GenericError>{
        let mut translated_lines: Vec<Vec<usize>> = Vec::new();
        // dbg!(&line_dims);
    
        for (i, dim) in line_dims.iter().enumerate(){
            // dbg!(&i);
            
            // dbg!(self.translator.len());
            
            let dim_translator = self.translator.get(i)
                .ok_or(GenericError::new(format!("Could not get translator for dimension index {}", i).as_str(), file!(), &line!()))?;

            let values: Vec<String> = dim.split(",").map(|i| i.to_owned()).collect();
            let mut translated_dim: Vec<usize> = Vec::new();
    
            for value in values{
                let translated_value = dim_translator.get(&value)
                    .ok_or(GenericError::new(format!("Could not translate value: {}", value).as_str(), file!(), &line!()))?;

                let translated_value = usize::try_from(*translated_value)
                    .map_err(|_| GenericError::new(format!("Could not cast {} to usize", translated_value).as_str(), file!(), &line!()))?;

                translated_dim.push(translated_value);
            }
            
            translated_lines.push(translated_dim);
        }
    
        return Ok(translated_lines);
    }
    
    pub fn untranslateLineDims(&self, dims_values: &Vec<Vec<usize>>) -> Result<Vec<String>, GenericError>{
        let mut original_dims: Vec<String> = Vec::new();
        for (i, dim) in dims_values.iter().enumerate(){
            let mut original_dim: Vec<String> = Vec::new();
    
            for value in dim{
                let value = *value as u32;
                let original_value =  self.reversed_translator.get(i)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .get(&value)
                    .ok_or(GenericError::new("Error parsing tensor file", file!(), &line!()))?
                    .to_owned();
    
                original_dim.push(original_value);
            }
    
            original_dims.push(original_dim.join(","));
        }

        return Ok(original_dims);
    }

    pub fn getSize(&self) -> Vec<usize>{
        let mut translator_size: Vec<usize> = Vec::new();

        for dim_translator in self.translator.iter(){
            translator_size.push(dim_translator.len());
        }

        return translator_size;
    }
}
### src-tauri/src/model/io/translator.rs END ###

### src-tauri/src/services/datapoint_service.rs BEGIN ###
use crate::{model::{identifier_mapper::IdentifierMapper, analysis::metrics::{coordinates::Coordinates, metric::Metric}}, database::{datapoint::DataPoint, pattern::Pattern}, common::generic_error::GenericError};

pub struct DataPointService {}

impl DataPointService {
    fn normalizeSize(size: &u32, dimension: &u32) -> f32 {
        // let size_multiplier = 1.0;
        // let normalized_size = size_multiplier * (*size as f32 / *min_size as f32).ln();

        // if normalized_size == 0.0 {
        //     return size_multiplier;
        // }
        // return normalized_size;
        return (*size as f32).powf(1.0 / *dimension as f32);
        
    }

    fn densityToColor(density: &f64) -> (u32, u32, u32, f32) {
        let r = 255 as u32;
        let g = 0;
        let b = 0 as u32;
        // let a = (255 as f64 * density) as u32;
        let a = *density as f32;

        return (r, g, b, a);
    }

    pub fn createDataPoints(identifier_mapper: &IdentifierMapper, coordinates: &Coordinates) -> Result<Vec<DataPoint>, GenericError> {
        let coordinates = coordinates.get();

        let mut pattern_representations: Vec<&Pattern> = Vec::new();
        for (identifier, _) in coordinates{
            let pattern = identifier_mapper.getRepresentation(identifier)?.asPattern()?;
            pattern_representations.push(pattern);
        }

        let mut datapoints: Vec<DataPoint> = Vec::new();
        let dimension = pattern_representations.get(0)
            .ok_or(GenericError::new("Could not get dimension", file!(), &line!()))?
            .dims_values.len() as u32;

        for pattern in pattern_representations {
            let coord = coordinates.get(&pattern.identifier)
                .ok_or(GenericError::new(format!("Could not get coordinate: {}", &pattern.identifier).as_str(), file!(), &line!()))?;
            
            let size = DataPointService::normalizeSize(&pattern.size, &dimension);
            let density = pattern.density as f32;
            // let stroke_width = DataPointService::calculateStrokeWidth(&max_size, &size);
            let stroke_width = 2;
            let color = DataPointService::densityToColor(&pattern.density);
            
            let x = coord.0 as f32;
            // let x = f32::round(100.0 * x) / 100.0;
            
            let y = coord.1 as f32;
            // let y = f32::round(100.0 * y) / 100.0;
            
            let datapoint = DataPoint::new(
                &pattern.identifier,
                &size,
                &pattern.size,
                &density,
                &stroke_width,
                &x,
                &y,
                &color.0,
                &color.1,
                &color.2,
                &color.3
                );

            datapoints.push(datapoint);
        }

        return Ok(datapoints);
    }
}
### src-tauri/src/services/datapoint_service.rs END ###

### src-tauri/src/services/dynamic_paginator_service.rs BEGIN ###
#![allow(non_snake_case)]

use crate::{model::{identifier_mapper::IdentifierMapper, io::translator::{self, Translator}}, database::{pattern::Pattern, raw_pattern::RawPattern}, common::generic_error::GenericError};

pub struct DynamicPaginatorService{
    current_page: u32,
    page_size: u32,
  
    pub first_page: u32,
    pub last_page: u32,
}

impl<'a> Default for DynamicPaginatorService{
    fn default() -> Self { 
        return DynamicPaginatorService {
            current_page: 0,
            page_size: 1, 
            first_page: 0, 
            last_page: 0 };
    }
}

impl DynamicPaginatorService{
    pub fn getSoundingPattern(&self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<RawPattern, GenericError>{
        return Ok(identifier_mapper.getRepresentation(&1)?.asRawPattern(translator)?); // ID's start at 1
    }

    pub fn refreshPageSize(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator, page_size: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        self.page_size = page_size;
        self.refreshLastPage(identifier_mapper);

        let first_page = self.first_page.clone();
        return self.goToPage(identifier_mapper, translator, &first_page);
    }

    fn refreshLastPage(&mut self, identifier_mapper: &IdentifierMapper){
        self.last_page = (identifier_mapper.length() as f64 / self.page_size as f64).ceil() as u32 - 1;
    }

    pub fn goToPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator, page_index: &u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        if *page_index > self.last_page {
            return self.goToPage(identifier_mapper, translator, &self.last_page.clone());
        }

        if *page_index < self.first_page {
            return self.goToPage(identifier_mapper, translator, &self.first_page.clone());
        }

        let mut page_patterns: Vec<RawPattern> = Vec::new();
        self.current_page = *page_index;

        let first_index = self.current_page * self.page_size;
        let last_index = first_index + self.page_size - 1;
        let last_pattern_index = identifier_mapper.length() - 1;

        for i in first_index..last_index + 1{
            if i > last_pattern_index {
                break;
            }
            page_patterns.push(identifier_mapper.getRepresentation(&(i + 1))?.asRawPattern(translator)?);
        }

        return Ok((page_patterns, self.current_page.clone(), self.last_page.clone()));
    }
    
    pub fn nextPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        return self.goToPage(identifier_mapper, translator, &(self.current_page + 1));
    }

    pub fn previousPage(&mut self, identifier_mapper: &IdentifierMapper, translator: &Translator) -> Result<(Vec<RawPattern>, u32, u32), GenericError>{
        if self.current_page == self.first_page { // Prevents u32 overflow when trying to go to page -1
            return self.goToPage(identifier_mapper, translator, &(self.first_page).clone());
        }
        return self.goToPage(identifier_mapper, translator, &(self.current_page - 1).clone());
    }
}
### src-tauri/src/services/dynamic_paginator_service.rs END ###

### src-tauri/src/services/io_service.rs BEGIN ###
use crate::{model::io::{translator::Translator, tensor_reader::TensorReader, pattern_reader::PatternReader}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct IoService {
    tensor_path: String,
    patterns_path: String,
    translator: Translator,
}

impl Default for IoService{
    fn default() -> Self {
        return IoService{
            tensor_path: String::from(""),
            patterns_path: String::from(""),
            translator: Translator::default(),
        };
    }
}

impl IoService {
    pub fn new(tensor_path: &String, patterns_path: &String) -> Result<IoService, GenericError> {
        let translator = Translator::new(&tensor_path)?;

        return Ok(
            IoService {
                tensor_path: tensor_path.to_owned(),
                patterns_path: patterns_path.to_owned(),
                translator: translator,
            }
        );
    }

    pub fn setPatternsPath(&mut self, patterns_path: &String) {
        self.patterns_path = patterns_path.to_owned();
    }

    pub fn readTensor(&self) -> Result<Tensor, GenericError> {
        println!("Reading tensor ...");
        let tensor_reader = TensorReader::new(
            &self.tensor_path,
            &self.translator);
        return tensor_reader.read();
    }

    pub fn readPatterns(&self) -> Result<Vec<Pattern>, GenericError> {
        println!("Reading patterns ...");
        let pattern_reader = PatternReader::new(
                &self.patterns_path,
                &self.translator)?;

        return pattern_reader.read();
    }

    pub fn getTranslator(&self) -> &Translator {
        return &self.translator;
    }
}
### src-tauri/src/services/io_service.rs END ###

### src-tauri/src/services/metrics_service.rs BEGIN ###
#![allow(non_snake_case)]

use crate::model::analysis::metrics::intersection::intersection_metrics::IntersectionMetrics;
use crate::model::analysis::metrics::intersection::intersections_percentages::IntersectionsPercentages;
use crate::model::analysis::metrics::intersections_predictions::IntersectionsPredictions;
use crate::{model::{analysis::metrics::{empty_model_rss::EmptyModelRss, distances::Distances, coordinates::Coordinates, rss_evolution::RssEvolution}, identifier_mapper::IdentifierMapper}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct MetricsService{
    pub empty_model_rss: EmptyModelRss,
    pub rss_evolution: RssEvolution,
    pub all_initial_visible_distances: Distances,
    pub coordinates: Coordinates,
    pub intersections_percentages: IntersectionsPercentages,
}

impl MetricsService{
    pub fn new(identifier_mapper: &IdentifierMapper, tensor: &Tensor, visible_identifiers: &Vec<u32>) -> Result<MetricsService, GenericError>{
        println!("Calculating metrics...");

        let intersections_predictions = IntersectionsPredictions::new(identifier_mapper)?;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &identifier_mapper.getOrderedPatterns(),
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;

        let empty_model_rss = EmptyModelRss::new(tensor);
        let patterns: Vec<&Pattern> = identifier_mapper.getOrderedPatterns();

        let rss_evolution = RssEvolution::new(
            identifier_mapper,
            tensor,
            &empty_model_rss,
            &patterns,
            &mut prediction_matrix,
            &untouched_delta_rss,
            &intersections_indices
        )?;

        let distances = Distances::new(
            identifier_mapper,
            tensor,
            &intersections_predictions,
            &visible_identifiers,
        )?;

        let coordinates = Coordinates::new(
            &distances,
        )?;

        println!("All metrics done!");
        return Ok(
            MetricsService {
                empty_model_rss: empty_model_rss,
                rss_evolution: rss_evolution,
                all_initial_visible_distances: distances,
                coordinates: coordinates,
                intersections_percentages: intersections_percentages,
             }
        );
    }

    pub fn update(&mut self, tensor: &Tensor, identifier_mapper: &IdentifierMapper, visible_identifiers: &Vec<u32>, lazy: &bool)
            -> Result<(), GenericError>{

        let visible_patterns = identifier_mapper.getOrderedPatternsFrom(visible_identifiers);
        
        let coordinates = Coordinates::new(
            &self.all_initial_visible_distances.getView(identifier_mapper, visible_identifiers)?,
        )?;
        self.coordinates = coordinates;

        let (prediction_matrix, 
            untouched_delta_rss, 
            intersections_indices,
            intersections_percentages) = IntersectionMetrics::calculate(
                tensor,
                &visible_patterns,
                identifier_mapper)?;
        let mut prediction_matrix = prediction_matrix;
        self.intersections_percentages = intersections_percentages;

        if !lazy{ // Re-calculate rss_evolution
            let rss_evolution = RssEvolution::new(
                identifier_mapper,
                tensor,
                &self.empty_model_rss,
                &visible_patterns,
                &mut prediction_matrix,
                &untouched_delta_rss,
                &intersections_indices
            )?;

            self.rss_evolution = rss_evolution;
        
        }else if *lazy{ // Just truncate
            let new_size = visible_identifiers.len() as u32;
            self.rss_evolution.truncate(&new_size);
        }

        return Ok(());
    }
}
### src-tauri/src/services/metrics_service.rs END ###

### src-tauri/src/services/mod.rs BEGIN ###
pub mod application;
pub mod dynamic_paginator_service;
pub mod dag;
pub mod io_service;
pub mod datapoint_service;
pub mod plot_service;
pub mod metrics_service;
### src-tauri/src/services/mod.rs END ###

### src-tauri/src/services/plot_service.rs BEGIN ###
#![allow(non_snake_case)]
use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, Circle}, style::{WHITE, Color, IntoFont, RGBAColor, RGBColor, TextStyle}};

use crate::{model::{identifier_representation::IdentifierRepresentation}};

use super::application::application_state_service::ApplicationStateService;

pub struct PlotService{}

impl PlotService{
    pub fn plot(application_state: &ApplicationStateService){
        // let root = BitMapBackend::new("scatter.png", (1600, 900)).into_drawing_area();
        // root.fill(&WHITE).unwrap();

        // let identifier_mapper = application_state.identifierMapper().unwrap();

        // let visible_identifiers = application_state.getVisibleIdentifiers();
        // let visible_representations: Vec<&IdentifierRepresentation> = visible_identifiers.iter()
        //     .map(|identifier| identifier_mapper.getRepresentation(identifier).unwrap())
        //     .collect();
    
        // let mut x_range = 0.0;
        // let mut y_range = 0.0;
        // for identifier_representation in visible_representations.iter(){
        //     let datapoint = identifier_representation.asDataPoint();
            
        //     let positive_x_range = x_range.clone();
        //     let negative_x_range = x_range.clone() * -1.0;
    
        //     let positive_y_range = y_range.clone();
        //     let negative_y_range = y_range.clone() * -1.0;
    
        //     if datapoint.x > positive_x_range{ x_range = datapoint.x.clone().abs(); }
        //     else if datapoint.x < negative_x_range{ x_range = datapoint.x.clone().abs(); }
    
        //     if datapoint.y > positive_y_range{ y_range = datapoint.y.clone().abs(); }
        //     else if datapoint.y < negative_y_range{ y_range = datapoint.y.clone().abs(); }
        // }
    
        // x_range *= 1.1;
        // y_range *= 1.1;
    
        // let mut chart = ChartBuilder::on(&root)
        //     .caption("Scatter Plot", ("sans-serif", 50).into_font())
        //     .margin(5)
        //     .x_label_area_size(30)
        //     .y_label_area_size(30)
        //     .build_cartesian_2d(-1.0 * x_range..x_range, -1.0 * y_range..y_range).unwrap();
    
        // chart.configure_mesh().draw().unwrap();
    
        // // Enforcing that overlapping points are drawn in the correct order
        // let mut representations = visible_representations;
        // representations.sort_by(|a, b| 
        //     b.asDataPoint().size.partial_cmp(&a.asDataPoint().size).unwrap()); 
            
        // for identifier_representation in representations{
        //     let datapoint = identifier_representation.asDataPoint();
        //     let mut color = RGBColor(datapoint.r as u8, datapoint.g as u8, datapoint.b as u8).filled();
            

        //     // 2 e 7
        //     // if datapoint.identifier == 2 {color = RGBColor(0, 255, 0).filled();}
        //     // if datapoint.identifier == 7 {color = RGBColor(0, 0, 255).filled();}

        //     // let pattern = identifier_representation.asPattern();
        //     // dbg!(datapoint.color);
        //     // dbg!(pattern.identifier);
            
        //     chart.draw_series(
        //         std::iter::once(Circle::new((
        //             datapoint.x, datapoint.y), 
        //             2 * datapoint.size as i32,
        //             color.filled()
        //             // ShapeStyle {
        //             //     color: datapoint.color.to_rgba(),
        //             //     filled: false,
        //             //     stroke_width: datapoint.stroke_width,
        //             // }
        //         ))
        //     ).unwrap();
        // }
    
        // println!("PLOTTED TEST GRAPH");
    }
}
### src-tauri/src/services/plot_service.rs END ###

### src-tauri/src/services/application/application_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::{collections::HashMap, time::Instant};
use itertools::Itertools;
use plotters::data;

use crate::{common::generic_error::GenericError, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, pattern::Pattern, raw_pattern::RawPattern}, model::{analysis::metrics::metric::Metric, identifier_mapper::IdentifierMapper, io::translator::Translator}, services::{io_service::IoService, plot_service::PlotService}};
use super::application_state_service::ApplicationStateService;

pub struct ApplicationService{
    io_service: IoService,
    application_state_service: ApplicationStateService,
}

impl Default for ApplicationService{
    fn default() -> Self {
        return ApplicationService{
            io_service: IoService::default(),
            application_state_service: ApplicationStateService::default(),
        };
    }
}

impl ApplicationService{
    pub fn init(&mut self, tensor_path: &String, patterns_path: &String) -> Result<(), GenericError>{
        let start_time = Instant::now();
        println!("Initializing model...");

        self.io_service = IoService::new(tensor_path, patterns_path)?;
        let tensor = self.io_service.readTensor()?;
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service = ApplicationStateService::default();
        self.application_state_service.init(tensor, patterns)?;

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("Total time taken: {:?}", duration);

        PlotService::plot(&self.application_state_service);
        return Ok(());
    }

    pub fn getFlattenedSupers(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSupers(identifier_mapper)?
        );
    }

    pub fn getFlattenedSubs(&self) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let identifier_mapper = self.application_state_service.identifierMapper()?;
        return Ok(
            self.application_state_service.getDagService()?.getFlattenedSubs(identifier_mapper)?
        );
    }

    pub fn getDistances(&self) -> Result<&HashMap<u32, HashMap<u32, f64>>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.all_initial_visible_distances.get()
        );
    }

    pub fn getIdentifierMapper(&self) -> Result<&IdentifierMapper, GenericError> {
        return self.application_state_service.identifierMapper();
    }

    pub fn getTranslator(&self) -> &Translator {
        return self.io_service.getTranslator();
    }

    // ================ External API ================

    pub fn changePatterns(&mut self, patterns_path: &String) -> Result<(), GenericError>{
        println!("\nChanging patterns to: {}", patterns_path);
        self.io_service.setPatternsPath(patterns_path);
        let patterns = self.io_service.readPatterns()?;

        self.application_state_service.changePatterns(patterns)?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<(), GenericError>{
        println!("\nAscending dag");
        self.application_state_service.ascendDag()?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<(), GenericError> {
        println!("\nDescending dag to: {}", next_identifier);
        self.application_state_service.descendDag(next_identifier)?;
        PlotService::plot(&self.application_state_service);

        return Ok(());
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<Vec<(f32, f32)>, GenericError> {
        println!("\nTruncating model to {} patterns", new_size);
        self.application_state_service.truncateModel(&new_size)?;
        PlotService::plot(&self.application_state_service);

        let mut datapoints = self.getDataPoints()?;
        datapoints.truncate(*new_size as usize);

        let datapoints_changes: Vec<(f32, f32)> = datapoints.into_iter()
            .map(|datapoint| (datapoint.x, datapoint.y))
            .collect();

        return Ok(datapoints_changes);
    }

    pub fn getDataPoints(&self) -> Result<Vec<DataPoint>, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();
        // let identifiers = self.application_state_service.getAllIdentifiers()?;
        let datapoints: Vec<DataPoint> = self.application_state_service.identifierMapper()?
            .getOrderedDataPointsFrom(visible_identifiers).into_iter()
            .map(|datapoint| datapoint.clone())
            .collect();

        return Ok(datapoints);
    }

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return self.application_state_service.getAllSubPatternsIdentifiers();
    }

    pub fn getRawPattern(&self, identifier: &u32) -> Result<RawPattern, GenericError>{
        let visible_identifiers = self.application_state_service.getVisibleIdentifiers();

        if !visible_identifiers.contains(identifier){
            return Err(GenericError::new("Identifier not visible", file!(), &line!()));
        }

        return self.getIdentifierMapper()?.getIdentifier(identifier)?
            .asRawPattern(self.io_service.getTranslator());
    }

    pub fn getFullRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.get().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getTruncatedRssEvolution(&self) -> Result<Vec<f64>, GenericError>{
        return Ok(
            self.application_state_service.getMetricsService()?.rss_evolution.getTruncated().clone()
            .into_iter()
            .map(|size_rss| size_rss.1)
            .collect()
        );
    }

    pub fn getIntersectionsPercentages(&self, identifier: &u32) -> Result<HashMap<u32, f64>, GenericError> {
        let intesections_percentages = self.application_state_service.getMetricsService()?.intersections_percentages.get();
        return Ok(
            intesections_percentages.get(identifier)
                .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?
                .clone()
        );
    }

    pub fn getIntersectionDetails(&self, identifier: &u32) -> Result<IntersectionsDetails, GenericError>{
        let intersection_percentages: HashMap<u32, f64> = match self.application_state_service.getMetricsService()?
            .intersections_percentages.get().get(identifier){

            Some(intersection_percentages) => intersection_percentages.clone(),
            None => HashMap::new(),
        };

        let total_untouched_percentage = intersection_percentages.get(identifier)
            .expect("Should have a total untouched percentage, even if its 0").clone();
        let total_intersection_percentage = 1.0 - total_untouched_percentage;
        
        let current_pattern = self.getIdentifierMapper()?.getIdentifier(identifier)?.asPattern()?;
        let all_dims_intersections: Result<HashMap<u32, (f64, Vec<Vec<String>>)>, GenericError> = intersection_percentages.into_iter()
            .filter(|(other_identifier, _)| *other_identifier != *identifier)
            .map(|(other_identifier, percentage)| {

                let other_pattern = self.getIdentifierMapper()?.getIdentifier(&other_identifier)?.asPattern()?;
                
                let dims_intersections = current_pattern.dimIntersection(&other_pattern)?;
                let dims_intersections = self.getTranslator()
                    .untranslateLineDims(&dims_intersections)?.iter()
                    .map(|line| {
                        let values: Vec<String> = line.split(",").map(|dim| dim.to_string()).collect_vec();
                        return values;
                    })
                    .collect();

                return Ok((other_identifier, (percentage, dims_intersections)));
            })
            .collect();
        let all_dims_intersections = all_dims_intersections?;
        
        let intersections_details = IntersectionsDetails::new(*identifier, 
            total_untouched_percentage, total_intersection_percentage, all_dims_intersections);
        
        return Ok(intersections_details);
    }

    

}
### src-tauri/src/services/application/application_service.rs END ###

### src-tauri/src/services/application/application_state_service.rs BEGIN ###
#![allow(non_snake_case)]
use crate::common::generic_error::GenericError;
use crate::database::pattern::Pattern;
use crate::database::tensor::Tensor;

use crate::model::analysis::metrics::metric::Metric;
use crate::model::identifier_mapper::{IdentifierMapper, self};
use crate::services::dag::dag_service::DagService;
use crate::services::datapoint_service::DataPointService;
use crate::services::metrics_service::{MetricsService, self};

#[derive(Default)]
pub struct ApplicationStateService{
    tensor: Option<Tensor>,
    identifier_mapper: Option<IdentifierMapper>,

    metrics_service: Option<MetricsService>,
    dag_service: Option<DagService>,

    current_identifier: u32,
    current_level_identifiers: Vec<u32>,
    visible_identifiers: Vec<u32>,
}

impl ApplicationStateService{
    pub fn init(&mut self, tensor: Tensor, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        self.tensor = Some(tensor);
        self.changePatterns(patterns)?;

        return Ok(());
    }

    pub fn changePatterns(&mut self, patterns: Vec<Pattern>) -> Result<(), GenericError>{
        // Inserts the pattern representations
        let mut identifier_mapper = IdentifierMapper::new(patterns);

        // Inserts the dag node representations
        identifier_mapper.insertDagNodeRepresentations(
            DagService::createAndArrange(&identifier_mapper)?,
        )?;
        
        let dag_service = DagService::new(&identifier_mapper)?;
        self.dag_service = Some(dag_service);

        self.current_level_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getFontNodes();

        self.visible_identifiers = self.current_level_identifiers.clone();

        // Inserts the data point representations
        let metrics_service = MetricsService::new(
            &identifier_mapper,
            self.tensor.as_ref()
                .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?,
            &self.visible_identifiers
        )?;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, &metrics_service.coordinates)?
        )?;

        self.metrics_service = Some(metrics_service);
        self.identifier_mapper = Some(identifier_mapper);

        return Ok(());
    }

    fn update(&mut self, new_current_level_identifiers: &Option<Vec<u32>>) -> Result<(), GenericError>{
        let tensor = self.tensor.as_ref()
            .ok_or(GenericError::new("Tensor not initialized", file!(), &line!()))?;

        let identifier_mapper = self.identifier_mapper.as_mut()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()))?;

        let lazy = match new_current_level_identifiers {
            Some(_) => false, // Changing the current level identifiers has to be done eagerly
            None => true, // Here we do not need to re-calculate rss_evolution
        };

        let identifiers_used_to_update = match new_current_level_identifiers {
            Some(new_current_level_identifiers) => new_current_level_identifiers, // Updates all identifiers and reset visible identifiers (dag movement)
            None => &self.visible_identifiers, // Updates only the visible identifiers, (truncation)
        };

        self.metrics_service.as_mut().ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .update(tensor, identifier_mapper, identifiers_used_to_update, &lazy)?;

        let coordinates = &self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
            .coordinates;

        identifier_mapper.insertDataPointRepresentations(
            DataPointService::createDataPoints(&identifier_mapper, coordinates)?
        )?;

        // Should also insert dagNode representations
        
        if !lazy{ // Reset everything because current_level_identifiers is gonna be changed
            self.current_level_identifiers = identifiers_used_to_update.clone();
            self.visible_identifiers = identifiers_used_to_update.clone();
        }

        return Ok(());
    }

    pub fn ascendDag(&mut self) -> Result<(), GenericError>{
        if self.current_identifier == 0{ return Ok(()); }

        let previous_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .ascendDag(self.identifierMapper()?, &self.current_identifier)?;

        self.update(&Some(previous_identifiers))?;

        return Ok(());
    }

    pub fn descendDag(&mut self, next_identifier: &u32) -> Result<(), GenericError>{
        let next_identifiers = self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .descendDag(self.identifierMapper()?, next_identifier)?;

        if next_identifiers.len() == 0{ return Ok(()); }

        self.update(&Some(next_identifiers))?;
        return Ok(());
    }

    pub fn truncateModel(&mut self, new_size: &u32) -> Result<(), GenericError>{
        let mut all_identifiers = self.identifierMapper()?
            .getIdentifiers().clone();
            
        all_identifiers.sort();
        all_identifiers.truncate(*new_size as usize);

        let mut current_level_identifiers = self.current_level_identifiers.clone();
        current_level_identifiers.sort();

        self.visible_identifiers = current_level_identifiers.iter()
            .filter(|identifier| all_identifiers.contains(&identifier))
            .map(|identifier| identifier.clone())
            .collect();

        self.update(&None)?;

        return Ok(());
    }

    pub fn identifierMapper(&self) -> Result<&IdentifierMapper, GenericError>{
        return self.identifier_mapper.as_ref()
            .ok_or(GenericError::new("Identifier mapper not initialized", file!(), &line!()));
    }

    pub fn getAllIdentifiers(&self) -> Result<&Vec<u32>, GenericError>{
        return Ok(
            &self.current_level_identifiers
        );
    }

    pub fn getVisibleIdentifiers(&self) -> &Vec<u32>{
        return &self.visible_identifiers;
    }

    pub fn getAllSubPatternsIdentifiers(&self) -> Result<Vec<u32>, GenericError>{
        return Ok(
            self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()))?
            .getSubNodes()
        );
    }

    pub fn getMetricsService(&self) -> Result<&MetricsService, GenericError>{
        return Ok(
            self.metrics_service.as_ref()
            .ok_or(GenericError::new("Metrics service not initialized", file!(), &line!()))?
        );
    }

    pub fn getDagService(&self) -> Result<&DagService, GenericError>{
        return self.dag_service.as_ref()
            .ok_or(GenericError::new("Dag service not initialized", file!(), &line!()));
    }
}
### src-tauri/src/services/application/application_state_service.rs END ###

### src-tauri/src/services/application/mod.rs BEGIN ###
pub mod application_service;
pub mod application_state_service;
### src-tauri/src/services/application/mod.rs END ###

### src-tauri/src/services/dag/dag_arranger_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::{database::dag_node::DagNode, common::generic_error::GenericError};
use debug_print::debug_println;

pub (in crate::services::dag) struct DagArrangerService{
    fonts: Vec<u32>,
    nodes: HashMap<u32, DagNode>,
}

impl DagArrangerService{
    pub fn new() -> DagArrangerService{
        return DagArrangerService {
            fonts: Vec::new(),
            nodes: HashMap::new(),
        };
    }

    pub fn init(&mut self, nodes: Vec<DagNode>){
        let nodes: HashMap<u32, DagNode> = nodes.into_iter()
            .map(|node| (node.identifier, node))
            .collect();

        self.nodes = nodes;
    }

    pub fn addFont(&mut self, new_font: &u32){
        debug_println!("    {} is now a font", new_font);
        if !self.fonts.contains(new_font){
            self.fonts.push(*new_font);
        }
    }
    
    pub fn removeFont(&mut self, old_font: &u32){
        debug_println!("    {} is not a font anymore", old_font);
        self.fonts.retain(|f| f != old_font);
    }

    pub fn addOverlappingNode(&mut self, overlapped_node: &u32, overlapping_node: &u32) -> Result<(), GenericError>{
        let overlapped_node = self.nodes.get_mut(overlapped_node)
            .ok_or(GenericError::new("Error adding overlapping node", file!(), &line!()))?;

        overlapped_node.overlappings.insert(*overlapping_node);

        return Ok(());
    }

    pub fn addBellow(&mut self, adding_node: &u32, parent: &u32) -> Result<(), GenericError>{
        let adding_node_p = self.nodes.get_mut(&adding_node)
            .ok_or(GenericError::new("Error adding node", file!(), &line!()))?;

        adding_node_p.supers.push(*parent);

        let parent_p = self.nodes.get_mut(&parent)
            .ok_or(GenericError::new("Error adding node", file!(), &line!()))?;
        
        parent_p.subs.push(*adding_node);

        return Ok(());
    }

    pub fn moveSubtreeBellow(&mut self, moving_node: &u32, new_parent: &u32) -> Result<(), GenericError>{
        let mut moving_node_p = self.nodes.get_mut(&moving_node)
            .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

        let old_parents: Vec<u32> = moving_node_p.supers.clone();
        moving_node_p.supers = vec![*new_parent]; // Removes old parents and adds new super of moving node

        for old_parent in old_parents{ // Deletes moving node from its old parents
            let old_parent_p = self.nodes.get_mut(&old_parent)
                .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

            old_parent_p.subs.retain(|p| p != moving_node);
        }

        let new_parent = self.nodes.get_mut(&new_parent)
            .ok_or(GenericError::new("Error moving node", file!(), &line!()))?;

        new_parent.subs.push(*moving_node); // Adds moving node to its new super

        return Ok(());
    }

    pub fn traverse(&self, to_node: &u32) -> Result<&Vec<u32>, GenericError>{
        return Ok(
            &self.nodes.get(to_node)
                .ok_or(GenericError::new("Error while traversing dag", file!(), &line!()))?
                .subs
        );
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_subs.insert(*id, node.subs.clone());
        }        

        return flattened_subs;
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, node) in self.nodes.iter(){
            flattened_supers.insert(*id, node.supers.clone());
        }        

        return flattened_supers;
    }

    pub fn getNode(&self, identifier: &u32) -> Result<&DagNode, GenericError>{
        return Ok(
            self.nodes.get(identifier)
                .ok_or(GenericError::new("Error getting node", file!(), &line!()))?
        );
    }

    pub fn getNodesIdentifiers(&self) -> Vec<u32>{
        let mut nodes: Vec<u32> = Vec::new();
        for node in self.nodes.values(){
            nodes.push(node.identifier);
        }
        return nodes;
    }

    pub fn getFontNodes(&self) -> Vec<&DagNode> {
        let mut font_nodes: Vec<&DagNode> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(self.getNode(font).expect("Should have found font node"));
        }   

        return font_nodes;
    }

    pub fn getFontNodesIdentifiers(&self) -> Vec<u32>{
        let mut font_nodes: Vec<u32> = Vec::new();

        for font in self.fonts.iter(){
            font_nodes.push(*font);
        }   

        return font_nodes;
    }

    pub fn finish(self) -> Vec<DagNode> {
        return self.nodes.into_iter()
            .map(|(_, node)| node)
            .collect();
    }
}
### src-tauri/src/services/dag/dag_arranger_service.rs END ###

### src-tauri/src/services/dag/dag_creator_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;
use crate::common::generic_error::GenericError;
use crate::database::dag_node::DagNode;
use crate::database::pattern::{Pattern, Relation};
use crate::model::identifier_mapper::IdentifierMapper;
use crate::common::progress_bar;
use colored::Colorize;
use debug_print::{debug_println, debug_print};

use super::dag_arranger_service::DagArrangerService;


#[derive(PartialEq, Debug, Clone, Copy)]
enum InsertionPlace{
    Bellow,
    Above,
}

pub (in crate::services::dag) struct DagCreatorService<'a>{
    dag_arranger_service: DagArrangerService,
    identifier_mapper: &'a IdentifierMapper,

    actual_node: u32,
    insertion_points: HashMap<u32, InsertionPlace>,

    assigned_belonging_level: bool,
    belonging_level: u32,
    belonging_branch: u32,
}

impl DagCreatorService<'_>{
    pub fn new<'a>(identifier_mapper: &'a IdentifierMapper) -> DagCreatorService<'a>{
        return DagCreatorService {
            dag_arranger_service: DagArrangerService::new(),
            identifier_mapper: identifier_mapper,
            actual_node: 0,
            insertion_points: HashMap::new(),
            assigned_belonging_level: false,
            belonging_level:0, 
            belonging_branch:0, 
        };
    }

    fn changePosition(&mut self, new_position: u32) -> Result<&Vec<u32>, GenericError> {
        self.actual_node = new_position;
        return Ok(
            self.dag_arranger_service.traverse(&self.actual_node)?
        );
    }

    fn firstRelationToSecond(&self, first_node: &u32, second_node: &u32) -> Result<Relation, GenericError> {
        let first_patern: &Pattern = self.identifier_mapper.getRepresentation(first_node)?.asPattern()?;
        let second_patern: &Pattern = self.identifier_mapper.getRepresentation(second_node)?.asPattern()?;
        return first_patern.selfRelationTo(second_patern);
    }

    fn refreshOverlappingRelations(&mut self, first_node: &u32, second_node: &u32, relation: Relation) -> Result<(), GenericError>{
        if relation == Relation::NotRelatable{ return Ok(()); }

        let first_pattern_density = self.identifier_mapper.getRepresentation(first_node)?.asPattern()?.density;
        let second_pattern_density = self.identifier_mapper.getRepresentation(second_node)?.asPattern()?.density;

        if first_pattern_density >= second_pattern_density{
            self.dag_arranger_service.addOverlappingNode(second_node, first_node)?;
        }

        if first_pattern_density <= second_pattern_density {
            self.dag_arranger_service.addOverlappingNode(first_node, second_node)?;
        } 

        return Ok(());
    }

    fn traverseTree(&mut self, subtree_font: &u32, node_to_compare: &u32, current_branch: u32, current_level: u32) -> Result<(), GenericError>{
        debug_print!("\n    => Traversing subtree of {}, ", subtree_font);
        let current_level_nodes: Vec<u32> = self.changePosition(*subtree_font)?.clone();
        let mut next_level_fonts: Vec<u32> = Vec::new();
        debug_println!("level: {}, level size: {}, branch: {}, belonging_branch: {}, belonging_level: {}", &current_level, current_level_nodes.len(), &current_branch, &self.belonging_branch, &self.belonging_level);

        let mut belongs_to_this_level: bool = false;
        let relation = self.firstRelationToSecond(node_to_compare, &subtree_font)?;
        if relation == Relation::SubPattern{ belongs_to_this_level = true; }
        self.refreshOverlappingRelations(node_to_compare, subtree_font, relation)?;
        
        for current_level_node in current_level_nodes.iter(){
            if relation == Relation::SuperPattern{ // Node to compare is super of subtree_font, does not need to traverse this branch
                continue;
            }

            if relation == Relation::NotRelatable{ // Node to compare does not have 'physical' contact with subtree_font, does not need to traverse this branch
                continue;
            }
            next_level_fonts.push(*current_level_node);
        }

        for (i, next_level_font) in next_level_fonts.iter().enumerate(){ // RECURSIVE
            let next_branch = current_branch + i as u32;
            self.traverseTree(&next_level_font, node_to_compare, next_branch, current_level + 1)?;
        }
        // Recursion returnal bellow

        // Insertion operation
        if belongs_to_this_level && !self.assigned_belonging_level{ // Makes sure to insert on the deepest possible
            debug_println!("    Setting insertion point to bellow {}", subtree_font);
            
            self.insertion_points.insert(*subtree_font, InsertionPlace::Bellow);
            self.assigned_belonging_level = true; // Previous levels cannot change insertion point now
            self.belonging_level = current_level;
            self.belonging_branch = current_branch;
            debug_println!("    Belonging branch is now {}", &self.belonging_branch);
        }

        // Connects node_to_compare as super of this subtree font
        if relation == Relation::SuperPattern{
            // A pattern (node_to_compare) from an upper branch is super of the font of this branch
            // Sets the super relation on the recursion returnal
            debug_println!("    {} {} {}{}", format!("{}", &node_to_compare).yellow(), "located in a different upper branch is super of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag_arranger_service.addBellow(subtree_font, node_to_compare)?;
        }
    
        // Connects node_to_compare as sub of different branches
        if relation == Relation::SubPattern && current_branch != self.belonging_branch{ // Makes sure to connect ONLY to different branches
            // A pattern (node_to_compare) from a DIFFERENT branch is sub of the font of this branch
            // Sets the sub relation on the recursion returnal

            if current_level < self.belonging_level{ // Avoids the connection of patterns that are above the insertion point
                return Ok(());
            }

            debug_println!("    {} {} {}{}", format!("{}", node_to_compare).yellow(), "located in a different below branch is sub of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag_arranger_service.addBellow(node_to_compare, subtree_font)?;
        }

        return Ok(());

    }

    fn getRelationedFonts(&mut self,node: &u32) -> Result<HashMap<u32, Relation>, GenericError> {
        let fonts: Vec<u32> = self.dag_arranger_service.getFontNodesIdentifiers();
        debug_println!("    Current fonts {:?}", fonts);
        let mut relationed_fonts: HashMap<u32, Relation> = HashMap::new();

        for font in fonts{
            let relation = self.firstRelationToSecond(node, &font)?;
            if relation == Relation::NotRelatable{ continue; }
            self.refreshOverlappingRelations(node, &font, relation)?;

            relationed_fonts.insert(font, relation);
        }
        return Ok(relationed_fonts);
    }

    fn setInsertionPoints(&mut self, node: &u32) -> Result<(), GenericError>{
        debug_println!("{} {}", "\n=> SETTING insertion points for node".green(), format!("{}", node).green());
        self.insertion_points.clear();
        let relationed_fonts: HashMap<u32, Relation> = self.getRelationedFonts(node)?;

        if relationed_fonts.len() == 0{
            // This node is a new font
            debug_println!("    Node does not have any relationed font, setting it to be a new font");
            return Ok(());
        }

        debug_println!("    Found relationed fonts: {:?}", &relationed_fonts);

        for relationed_font in relationed_fonts {
            // Finds the insertion points on each font subtree
            if relationed_font.1 == Relation::SuperPattern{
                // Node is super of relationed_font, consequently node is the new font
                debug_println!("    {} is super of {}, setting insertion point to be above {}", node, &relationed_font.0, &relationed_font.0);
                self.insertion_points.insert(relationed_font.0, InsertionPlace::Above);
                continue;
            }

            self.assigned_belonging_level = false;
            self.belonging_branch = 0;
            self.belonging_level = 0;
            self.traverseTree(&relationed_font.0, node, 1, 2)?;
        }

        return Ok(());
    }

    fn insertNodeAbove(&mut self, node: &u32, insertion_point: &u32) -> Result<(), GenericError>{
        debug_println!("    Inserting {} above {}", node, insertion_point);
        self.dag_arranger_service.moveSubtreeBellow(insertion_point, node)?;

        return Ok(());
    }

    fn insertNodeBellow(&mut self, node: &u32, insertion_point: &u32) -> Result<(), GenericError>{
        let subs = self.dag_arranger_service.traverse(insertion_point)?.clone();

        debug_println!("    Inserting {} bellow {}", node, insertion_point);
        self.dag_arranger_service.addBellow(node, insertion_point)?;

        if subs.is_empty(){
            return Ok(());
        }

        debug_println!("    Insertion point has subs {:?}", &subs);
        for sub in subs{
            let relation = self.firstRelationToSecond(node, &sub)?;
            self.refreshOverlappingRelations(node, &sub, relation)?;

            if relation == Relation::SuperPattern{
                // If the node is super of someone rearrange dag
                debug_println!("    {} is super of {}, putting {} subtree bellow {}", node, &sub, &sub, node);
                self.dag_arranger_service.moveSubtreeBellow(&sub, node)?;
            }
        }

        return Ok(());
    }

    fn insertNodeOnDag(&mut self, node: &u32) -> Result<(), GenericError>{
        debug_println!("{} {} {}", "\n==> INSERTING node".yellow(), format!("{}", node).yellow(), "on DAG".yellow());
        debug_println!("    Insertion points: {:?} (empty if is new font)", &self.insertion_points);

        if self.insertion_points.is_empty(){
            self.dag_arranger_service.addFont(node);
        }

        for insertion_point in self.insertion_points.clone().iter(){
            debug_println!();
            let insertion_place = insertion_point.1;
            let insertion_point = insertion_point.0;

            if *insertion_place == InsertionPlace::Above{
                // This should only trigger if the dag has draw a pseudo-font
                self.dag_arranger_service.removeFont(insertion_point);
                self.dag_arranger_service.addFont(node);
                
                self.insertNodeAbove(node, insertion_point)?;
                continue;
            }

            if *insertion_place == InsertionPlace::Bellow{
                self.insertNodeBellow(node, insertion_point)?;
                continue;
            }
        }

        return Ok(());
    }

    pub fn create(mut self, flat_dag_nodes: Vec<DagNode>) -> Result<Vec<DagNode>, GenericError>{
        debug_println!("Nodes: {:?}", &flat_dag_nodes.iter().map(|node| node.identifier).collect::<Vec<u32>>());
        let bar = progress_bar::new(flat_dag_nodes.len() as u64,"Patterns inserted on DAG");

        self.dag_arranger_service.init(flat_dag_nodes);
        
        for id in self.dag_arranger_service.getNodesIdentifiers(){
            self.setInsertionPoints(&id)?;
            self.insertNodeOnDag(&id)?;
            bar.inc(1);
        }
        bar.finish();

        debug_println!("Subs: {:?}", self.dag_arranger_service.getFlattenedSubs());
        debug_println!("Supers: {:?}", self.dag_arranger_service.getFlattenedSupers());

        println!("\n  Nb of fonts found: {}", self.dag_arranger_service.getFontNodes().len());

        return Ok(self.dag_arranger_service.finish());
    }
}
### src-tauri/src/services/dag/dag_creator_service.rs END ###

### src-tauri/src/services/dag/dag_service.rs BEGIN ###
#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{database::dag_node::DagNode, common::generic_error::GenericError};
use crate::model::identifier_mapper::IdentifierMapper;
use super::dag_creator_service::DagCreatorService;

pub struct DagService{
    font_nodes: Vec<u32>,
    sub_nodes: Vec<u32>,
}

impl DagService{
    fn createFlatDagNodes(identifier_mapper: &IdentifierMapper) -> Vec<DagNode> {
        let mut nodes: Vec<DagNode> = Vec::new();
        for id in identifier_mapper.getIdentifiers(){
            nodes.push(DagNode::new(&id));
        }
        return nodes;
    }

    pub fn createAndArrange(identifier_mapper: &IdentifierMapper) -> Result<Vec<DagNode>, GenericError> {
        let flat_dag_nodes = DagService::createFlatDagNodes(identifier_mapper);
        let dag_creator_service = DagCreatorService::new(identifier_mapper);
        return dag_creator_service.create(flat_dag_nodes);
    }

    fn identifyNodes(identifier_mapper: &IdentifierMapper) -> Result<(Vec<u32>, Vec<u32>), GenericError>{
        let mut font_nodes: Vec<u32> = Vec::new();
        let mut sub_nodes: Vec<u32> = Vec::new();

        for representation in identifier_mapper.getRepresentations(){
            let dag_node = representation.asDagNode()?;

            if dag_node.supers.len() == 0{
                font_nodes.push(dag_node.identifier);
            }else{
                sub_nodes.push(dag_node.identifier);
            }
        
        }

        return Ok((font_nodes, sub_nodes));
    }

    pub fn new(identifier_mapper: &IdentifierMapper) -> Result<DagService, GenericError>{
        let (font_nodes, sub_nodes) = DagService::identifyNodes(identifier_mapper)?;
        return Ok(
            DagService{
                font_nodes: font_nodes,
                sub_nodes: sub_nodes,
            }
        );
    }

    pub fn getFontNodes(&self) -> Vec<u32> {
        return self.font_nodes.clone();
    }

    pub fn getSubNodes(&self) -> Vec<u32> {
        return self.sub_nodes.clone();
    }

    pub fn ascendDag(&self, identifier_mapper: &IdentifierMapper, current_identifier: &u32) -> Result<Vec<u32>, GenericError> {
        let supers = &identifier_mapper.getRepresentation(current_identifier)?.asDagNode()?.supers;
        if supers.len() == 0{
            return Ok(self.getFontNodes());
        }

        return Ok(supers.clone());
    }

    pub fn descendDag(&self, identifier_mapper: &IdentifierMapper, next_identifier: &u32) -> Result<Vec<u32> , GenericError> {
        let dag_node = identifier_mapper.getRepresentation(next_identifier)?.asDagNode()?;
        return Ok(dag_node.subs.clone());
    }

    pub fn getFlattenedSubs(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_subs.insert(dag_node.identifier, dag_node.subs.clone());
        }

        return Ok(flattened_subs);
    }

    pub fn getFlattenedSupers(&self, identifier_mapper: &IdentifierMapper) -> Result<HashMap<u32, Vec<u32>>, GenericError>{
        let dag_nodes: Result<Vec<&DagNode>, GenericError> = identifier_mapper.getRepresentations().iter()
            .map(|representation| representation.asDagNode())
            .collect();

        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for dag_node in dag_nodes?{
            flattened_supers.insert(dag_node.identifier, dag_node.supers.clone());
        }

        return Ok(flattened_supers);
    }
    
}
### src-tauri/src/services/dag/dag_service.rs END ###

### src-tauri/src/services/dag/mod.rs BEGIN ###
pub mod dag_service;
mod dag_creator_service;
mod dag_arranger_service;
### src-tauri/src/services/dag/mod.rs END ###

### DIRECTORY src-tauri/src/ FLATTENED CONTENT ###
