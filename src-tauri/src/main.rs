#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::{self, controller::states::states::*, database::pattern::Pattern};
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//////////////////////////// Paginator ////////////////////////////

#[tauri::command]
fn getSoundingPattern(paginator_service: State<PaginatorServiceState>) -> Pattern {
    let paginator_service = paginator_service.0.lock().unwrap();
    return paginator_service.getSoundingPattern();
}

#[tauri::command]
fn refreshPageSize(paginator_service: State<PaginatorServiceState>, page_size: u32) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    return paginator_service.refreshPageSize(page_size);
}

#[tauri::command]
fn goToPage(paginator_service: State<PaginatorServiceState>, page_index: u32) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    return paginator_service.goToPage(&page_index);
}

#[tauri::command]
fn goToFirstPage(paginator_service: State<PaginatorServiceState>) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    let first_page = paginator_service.first_page.clone();
    return paginator_service.goToPage(&first_page);
}

#[tauri::command]
fn goToLastPage(paginator_service: State<PaginatorServiceState>) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    let last_page = paginator_service.last_page.clone();
    return paginator_service.goToPage(&last_page);
}

#[tauri::command]
fn nextPage(paginator_service: State<PaginatorServiceState>) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    return paginator_service.nextPage();
}

#[tauri::command]
fn previousPage(paginator_service: State<PaginatorServiceState>) -> (Vec<Pattern>, u32, u32) {
    let mut paginator_service = paginator_service.0.lock().unwrap();
    return paginator_service.previousPage();
}

//////////////////////////// Application ////////////////////////////
#[tauri::command]
fn changeTensor(application_service: State<ApplicationServiceState>, tensor_path: String, patterns_path: String) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.changeTensor(&tensor_path, &patterns_path);
}

#[tauri::command]
fn changePatterns(application_service: State<ApplicationServiceState>, patterns_path: String) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.changePatterns(&patterns_path);
}

#[tauri::command]
fn truncateModel(application_service: State<ApplicationServiceState>, new_size: u32) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.truncateModel(&new_size);
}

#[tauri::command]
fn getRssEvolution(application_service: State<ApplicationServiceState>) -> Vec<f64> {
    let application_service = application_service.0.lock().unwrap();
    return application_service.getRssEvolution();
}

#[tauri::command]
fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.descendDag(&next_identifier);
}

#[tauri::command]
fn ascendDag(application_service: State<ApplicationServiceState>) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.ascendDag();
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

            changeTensor,
            changePatterns,
            ascendDag,
            descendDag,
            truncateModel,
            getRssEvolution
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}
