// #![allow(non_snake_case)]
// use crate::dag::dag::Dag;
// use crate::dag::dag_creator::DagCreator;
// use crate::pattern::pattern::Pattern;
// use crate::states::states::DagState;
// use tauri::State;

// #[tauri::command]
// pub fn getCurrentDag(dag_state:State<DagState>) -> Dag{
//     let mut current_dag = dag_state.0.lock().unwrap(); 
//     return current_dag.clone();
// }