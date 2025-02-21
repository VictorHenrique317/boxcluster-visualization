use std::collections::HashMap;

use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::ApplicationServiceState, database::{datapoint::DataPoint, intersections_details::IntersectionsDetails, raw_pattern::RawPattern}};

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
pub fn getAllSubPatternsIdentifiers(application_service: State<ApplicationServiceState>) -> Result<Vec<u32>, GenericError> {
    println!("Calling getAllSubPatternsIdentifiers...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getAllSubPatternsIdentifiers();
}

#[tauri::command]
pub fn getDatapointsWithSubPatterns(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDatapointsWithSubPatterns...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getDatapointsWithSubPatterns();
}

#[tauri::command]
pub fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) -> Result<Vec<DataPoint>, GenericError>{
    println!("Calling descendDag...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.descendDag(&next_identifier);
}

#[tauri::command]
pub fn ascendDag(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError>{
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

#[tauri::command]
pub fn getCurrentLevelBackgroundDensity(application_service: State<ApplicationServiceState>) -> Result<f64, GenericError> {
    println!("Calling getCurrentLevelBackgroundDensity...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getCurrentLevelBackgroundDensity();
}

#[tauri::command]
pub fn getAllDimsValues(application_service: State<ApplicationServiceState>) -> Result<Vec<Vec<String>>, GenericError> {
    println!("Calling getAllDimsValues...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getAllDimsValues();
}

#[tauri::command]
pub fn filterDatapoints(application_service: State<ApplicationServiceState>, identifiers: Vec<u32>,filters: Vec<Vec<String>>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling filterDatapoints...");

    let mut application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.filterDatapoints(&identifiers, &filters);
}

#[tauri::command]
pub fn getSubpatterns(application_service: State<ApplicationServiceState>, identifier: u32, filters: Vec<Vec<String>>) -> Result<Vec<u32>, GenericError> {
    println!("Calling getNbOfSubpatterns...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getSubpatterns(&identifier, &filters);
}