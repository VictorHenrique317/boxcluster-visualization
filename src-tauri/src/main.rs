#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::{self, controller::states::states::*};
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn ascendDag(application_service: State<ApplicationServiceState>) {
    let mut application_service = application_service.0.lock().unwrap();
    application_service.ascendDag();
}

fn main() {
    // tauri::Builder::default()
    //     .manage(ApplicationServiceState(Default::default()))
    //     .manage(PaginatorServiceState(Default::default()))
    //     .invoke_handler(tauri::generate_handler![ 
    //         ascendDag,
    //         // getSoundingPattern,
    //         // refreshPageSize,
    //         // goToPage,
    //         // goToFirstPage,
    //         // goToLastPage,
    //         // nextPage,
    //         // previousPage
    //         ])
    //     .run(tauri::generate_context!())
    //     .expect("Error while running tauri application");

    boxcluster_visualization::main()
}
