#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    // tauri::Builder::default()
    //     .manage(DagState(Default::default()))
    //     .manage(PaginatorState(Default::default()))
    //     .invoke_handler(tauri::generate_handler![greet, 
    //         readPatternFile,
    //         getSoundingPattern,
    //         refreshPageSize,
    //         goToPage,
    //         goToFirstPage,
    //         goToLastPage,
    //         nextPage,
    //         previousPage
    //         ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
    boxcluster_visualization::main()
}
