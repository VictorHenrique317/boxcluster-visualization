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
pub fn getPattern(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<RawPattern, GenericError> {
    println!("Calling getPattern...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getRawPattern(&identifier);
}

#[tauri::command]
pub fn getIntersectionDetails(application_service: State<ApplicationServiceState>, identifier: u32) -> Result<IntersectionsDetails, GenericError> {
    println!("Calling getIntersectionDetails...");

    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;
    return application_service.getIntersectionsDetails(&identifier);
}