#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Mutex, Arc}};
use ndarray::{IxDynImpl, Dim, ArrayD, Array, Array2};
use numpy::{PyArray2, IntoPyArray};
use pyo3::{Python, types::PyDict};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{model::identifier_mapper::IdentifierMapper};

use super::{metric::Metric, distances::Distances};

pub struct Coordinates {
    value: HashMap<u32, (f32, f32)>,
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, (f32, f32)>> for Coordinates{
    fn get(&self) -> &HashMap<u32, (f32, f32)> {
        return &self.value;
    }
}

impl Coordinates {
    pub fn new(identifier_mapper: &IdentifierMapper, distances: &Distances) -> Coordinates{
        println!("  Coordinates...");
        return Coordinates { 
            value: Coordinates::calculate(identifier_mapper, distances),
        };
    }

    fn buildDissimilarityMatrix(distances: &Distances, n: usize) -> Array2<f64> {
        let size: Vec<usize> = vec![n, n];
        let distance_matrix: Arc<Mutex<ArrayD<f64>>> = Arc::new(Mutex::new(Array::zeros(Dim(size.clone())).into_dyn()));

        distances.get().par_iter().for_each(|(pattern_1, columns)| {
            let pattern_1 = (pattern_1 - 1) as usize;

            for (pattern_2, distance) in columns{
                let pattern_2 = (pattern_2 - 1) as usize;

                let index: Dim<IxDynImpl> = Dim(vec![pattern_1, pattern_2]);
                let mut distance_matrix_lock = distance_matrix.lock().unwrap();
                let matrix_value = distance_matrix_lock.get_mut(index).unwrap();
                *matrix_value = *distance;
            }
        });

        let squared_proximity_matrix: Array2<f64> = distance_matrix.lock().unwrap().clone().into_shape((n, n)).unwrap();
        return squared_proximity_matrix;
    }

    fn SklearnMDS(dissimilarity_matrix: Array2<f64>) -> HashMap<u32, (f32, f32)>{
        pyo3::prepare_freethreaded_python();
        let xy_matrix: Array2<f64> = Python::with_gil(|py| {
            let sklearn = py.import("sklearn.manifold").unwrap();

            let kwargs = PyDict::new(py);
            kwargs.set_item("dissimilarity", "precomputed").unwrap();
            kwargs.set_item("normalized_stress", false).unwrap();
            kwargs.set_item("random_state", 1).unwrap();
            kwargs.set_item("n_components", 2).unwrap();
            // kwargs.set_item("n_jobs", -1).unwrap();
            let mds = sklearn.getattr("MDS").unwrap()
                .call((), Some(kwargs))
                .unwrap();

            let dissimilarities_py: &PyArray2<f64> = dissimilarity_matrix.into_pyarray(py);
            let embedding: &PyArray2<f64> = mds.call_method1("fit_transform", (dissimilarities_py,)).unwrap().extract().unwrap();
            embedding.readonly().as_array().to_owned()
        });

        let n_rows = xy_matrix.shape()[0];
        let mut xys: HashMap<u32, (f32, f32)> = HashMap::new();
        for i in 0..n_rows {
            let xy_row = xy_matrix.row(i);
            let identifier = (i + 1) as u32;
            let x = *xy_row.get(0).unwrap() ;
            let y = *xy_row.get(1).unwrap();
            xys.insert(identifier, (x as f32, y as f32));
        }
        return xys;
    }

    pub fn fitTransformMDS(distances: &Distances, identifier_mapper: &IdentifierMapper) -> HashMap<u32, (f32, f32)>{
        println!("Applying Multi Dimensional Scaling...");
        let n: usize = identifier_mapper.length() as usize;
        let dissimilarity_matrix = Coordinates::buildDissimilarityMatrix(distances, n);
        let xys = Coordinates::SklearnMDS(dissimilarity_matrix);
        return xys;
    }

    fn calculate(identifier_mapper: &IdentifierMapper, distances: &Distances) -> HashMap<u32, (f32, f32)> {
        return Coordinates::fitTransformMDS(&distances, &identifier_mapper);
    }
}

