// #![allow(non_snake_case)]
// use crate::dag::dag::Dag;
// use crate::dag::dag_creator::DagCreator;
// use crate::io::pattern_reader::PatternReader;
// use crate::pattern::pattern::Pattern;
// use crate::states::states::DagState;
// use tauri::State;

// #[tauri::command]
// pub fn readPatternFile(path: String, dag_state:State<DagState>){
//     let pattern_reader = PatternReader::new(&path);
//     let patterns = pattern_reader.getPatternsCopy();

//     let dag_creator = DagCreator::new(patterns.clone());
//     let new_dag = dag_creator.create();

//     let mut old_dag = dag_state.0.lock().unwrap(); 
//     *old_dag = new_dag.clone();
// }