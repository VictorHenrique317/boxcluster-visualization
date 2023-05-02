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