#![allow(non_snake_case)]
#[allow(non_camel_case_types)]
use ndarray::{Array2};
use pyo3::types::{PyDict};
use numpy::{PyArray2};
use pyo3::{Python};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{sync::{Arc, Mutex}, collections::HashMap};
use ndarray::{Array, Dim, ArrayD, IxDynImpl};
use numpy::IntoPyArray;

use crate::model::common::identifier_mapper::IdentifierMapper;

use super::metrics::distances::Distances;

#[derive(Default)]
pub struct MultiDimScaling{
}

impl MultiDimScaling{
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

    fn SklearnMDS(dissimilarity_matrix: Array2<f64>) -> HashMap<u32, (f64, f64)>{
        pyo3::prepare_freethreaded_python();
        let xy_matrix: Array2<f64> = Python::with_gil(|py| {
            let sklearn = py.import("sklearn.manifold").unwrap();

            let kwargs = PyDict::new(py);
            kwargs.set_item("dissimilarity", "precomputed").unwrap();
            kwargs.set_item("normalized_stress", false).unwrap();
            kwargs.set_item("random_state", 1).unwrap();
            kwargs.set_item("n_components", 2).unwrap();
            let mds = sklearn.getattr("MDS").unwrap()
                .call((), Some(kwargs))
                .unwrap();

            let dissimilarities_py: &PyArray2<f64> = dissimilarity_matrix.into_pyarray(py);
            let embedding: &PyArray2<f64> = mds.call_method1("fit_transform", (dissimilarities_py,)).unwrap().extract().unwrap();
            embedding.readonly().as_array().to_owned()
        });

        let n_rows = xy_matrix.shape()[0];
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n_rows {
            let xy_row = xy_matrix.row(i);
            let identifier = (i + 1) as u32;
            let x = *xy_row.get(0).unwrap() ;
            let y = *xy_row.get(1).unwrap();
            xys.insert(identifier, (x as f64, y as f64));
        }
        return xys;
    }

    pub fn fitTransform(distances: &Distances, identifier_mapper: &IdentifierMapper) -> HashMap<u32, (f64, f64)>{
        println!("Applying Multi Dimensional Scaling...");
        let n: usize = identifier_mapper.length() as usize;
        let dissimilarity_matrix = MultiDimScaling::buildDissimilarityMatrix(distances, n);
        let xys = MultiDimScaling::SklearnMDS(dissimilarity_matrix);
        return xys;
    }
}