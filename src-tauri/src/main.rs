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
            getDatapointsWithSubPatterns,
            getPattern,
            getIntersectionsPercentages,
            getIntersectionDetails,
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}
