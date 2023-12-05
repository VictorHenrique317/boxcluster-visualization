#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::{self, controller::states::states::*, database::{pattern::Pattern, datapoint::DataPoint}};
use tauri::State;
use boxcluster_visualization::common::generic_error::GenericError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//////////////////////////// Paginator ////////////////////////////

#[tauri::command]
fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<Pattern, GenericError> {

    println!("Calling getSoundingPattern...");
    let paginator_service = paginator_service.0.lock()?;
    let application_service = application_service.0.lock()?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.getSoundingPattern(identifier_mapper);
}

#[tauri::command]
fn refreshPageSize(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_size: u32) -> Result<(Vec<Pattern>, u32, u32), GenericError> {
    
    println!("Calling refreshPageSize...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    return paginator_service.refreshPageSize(application_service.getIdentifierMapper()?, page_size);
}

#[tauri::command]
fn goToPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_index: u32) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToPage...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    return paginator_service.goToPage(application_service.getIdentifierMapper()?, &page_index);
}

#[tauri::command]
fn goToFirstPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToFirstPage...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    let first_page = paginator_service.first_page.clone();
    return paginator_service.goToPage(application_service.getIdentifierMapper()?, &first_page);
}

#[tauri::command]
fn goToLastPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToLastPage...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    let last_page = paginator_service.last_page.clone();
    return paginator_service.goToPage(application_service.getIdentifierMapper()?, &last_page);
}

#[tauri::command]
fn nextPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling nextPage...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    return paginator_service.nextPage(application_service.getIdentifierMapper()?);
}

#[tauri::command]
fn previousPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling previousPage...");

    let mut paginator_service = paginator_service.0.lock().unwrap();
    let application_service = application_service.0.lock().unwrap();
    return paginator_service.previousPage(application_service.getIdentifierMapper()?);
}

//////////////////////////// Application ////////////////////////////
#[tauri::command]
fn initApplication(application_service: State<ApplicationServiceState>, tensor_path: String, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changeTensor...");

    let mut application_service = application_service.0.lock().unwrap();
    application_service.init(&tensor_path, &patterns_path)?;
    
    return Ok(());
}

#[tauri::command]
fn changePatterns(application_service: State<ApplicationServiceState>, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changePatterns...");

    let mut application_service = application_service.0.lock().unwrap();
    application_service.changePatterns(&patterns_path)?;

    return Ok(());
}

#[tauri::command]
fn truncateModel(application_service: State<ApplicationServiceState>, new_size: u32) 
        -> Result<(), GenericError>{
    println!("Calling truncateModel...");

    let mut application_service = application_service.0.lock().unwrap();
    application_service.truncateModel(&new_size)?;

    return Ok(());
}

#[tauri::command]
fn getFullRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getFullRssEvolution...");

    let application_service = application_service.0.lock().unwrap();
    return application_service.getFullRssEvolution();
}

#[tauri::command]
fn getTruncatedRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getTruncatedRssEvolution...");

    let application_service = application_service.0.lock().unwrap();
    return application_service.getTruncatedRssEvolution();
}

#[tauri::command]
fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) -> Result<(), GenericError>{
    println!("Calling descendDag...");

    let mut application_service = application_service.0.lock().unwrap();
    application_service.descendDag(&next_identifier)?;

    return Ok(());
}

#[tauri::command]
fn ascendDag(application_service: State<ApplicationServiceState>) -> Result<(), GenericError>{
    println!("Calling ascendDag...");

    let mut application_service = application_service.0.lock().unwrap();
    application_service.ascendDag()?;

    return Ok(());
}

#[tauri::command]
fn getDataPoints(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDataPoints...");

    let application_service = application_service.0.lock().unwrap();
    return application_service.getDataPoints();
}

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
            getDataPoints
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}
