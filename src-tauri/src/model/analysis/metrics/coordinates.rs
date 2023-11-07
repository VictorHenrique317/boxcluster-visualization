#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::{DMatrix, DVector, SVD};
use ndarray::{IxDynImpl, Dim, ArrayD, Array, Array2};
use numpy::{PyArray2, IntoPyArray};
use pyo3::{Python, types::PyDict};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::model::identifier_mapper::IdentifierMapper;
use super::{metric::Metric, distances::DistancesTrait};

pub struct Coordinates {
    value: HashMap<u32, (f64, f64)>,
}

#[allow(non_camel_case_types)]
impl Metric<HashMap<u32, (f64, f64)>> for Coordinates{
    fn get(&self) -> &HashMap<u32, (f64, f64)> {
        return &self.value;
    }
}

impl Coordinates {
    pub fn new<T: DistancesTrait>(identifier_mapper: &IdentifierMapper, distances: &T) -> Coordinates{
        println!("  Coordinates...");
        return Coordinates { 
            value: Coordinates::calculate(identifier_mapper, distances),
        };
    }

    fn buildDissimilarityMatrix<T: DistancesTrait>(distances: &T, n: usize) -> DMatrix<f64> {
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

        let dissimilarity_matrix: DMatrix<f64> = DMatrix::from_fn(n, n, |i, j| {
            let index: Dim<IxDynImpl> = Dim(vec![i, j]);
            let distance_matrix_lock = distance_matrix.lock().unwrap();
            let matrix_value = distance_matrix_lock.get(index).unwrap();
            return *matrix_value;
        });

        return dissimilarity_matrix;
    }

    // fn SklearnMDS(dissimilarity_matrix: Array2<f64>) -> HashMap<u32, (f64, f64)>{
    //     pyo3::prepare_freethreaded_python();
    //     let xy_matrix: Array2<f64> = Python::with_gil(|py| {
    //         let sklearn = py.import("sklearn.manifold").unwrap();

    //         let kwargs = PyDict::new(py);
    //         kwargs.set_item("dissimilarity", "precomputed").unwrap();
    //         kwargs.set_item("normalized_stress", false).unwrap();
    //         kwargs.set_item("random_state", 1).unwrap();
    //         kwargs.set_item("n_components", 2).unwrap();
    //         // kwargs.set_item("n_jobs", -1).unwrap();
    //         let mds = sklearn.getattr("MDS").unwrap()
    //             .call((), Some(kwargs))
    //             .unwrap();

    //         let dissimilarities_py: &PyArray2<f64> = dissimilarity_matrix.into_pyarray(py);
    //         let embedding: &PyArray2<f64> = mds.call_method1("fit_transform", (dissimilarities_py,)).unwrap().extract().unwrap();
    //         embedding.readonly().as_array().to_owned()
    //     });

    //     let n_rows = xy_matrix.shape()[0];
    //     let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
    //     for i in 0..n_rows {
    //         let xy_row = xy_matrix.row(i);
    //         let identifier = (i + 1) as u32;
    //         let x = *xy_row.get(0).unwrap() ;
    //         let y = *xy_row.get(1).unwrap();
    //         xys.insert(identifier, (x, y));
    //     }
    //     return xys;
    // }

    fn mds(distances: DMatrix<f64>, dimensions: usize) -> HashMap<u32, (f64, f64)> {
        // square distances
        let mut m = distances.map(|x| -0.5 * x.powi(2));

        // double centre the rows/columns
        let row_means = m.row_mean();
        let col_means = m.column_mean();
        let total_mean = row_means.mean();

        for i in 0..m.nrows() {
            for j in 0..m.ncols() {
                m[(i, j)] += total_mean - row_means[i] - col_means[j];
            }
        }

        // take the SVD of the double centred matrix, and return the
        // points from it
        let svd = SVD::new(m, true, true);
        let eigen_values = svd.singular_values.map(|x| x.sqrt());

        let u = svd.u.unwrap();
        let mut result = DMatrix::zeros(u.nrows(), dimensions);

        for i in 0..u.nrows() {
            for j in 0..dimensions {
                result[(i, j)] = u[(i, j)] * eigen_values[j];
            }
        }

        // Convert result to hashmap
        let n_rows = result.nrows();
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n_rows {
            let identifier = (i + 1) as u32;
            let x = result[(i, 0)];
            let y = result[(i, 1)];
            xys.insert(identifier, (x, y));
        }

        return xys;
    }

    fn calculate<T: DistancesTrait>(identifier_mapper: &IdentifierMapper, distances: &T) -> HashMap<u32, (f64, f64)> {
        println!("  Applying Multi Dimensional Scaling...");
        let n: usize = identifier_mapper.length() as usize;
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n);
        let xys = Coordinates::mds(dissimilarity_matrix, 2);
        return xys;
    }
}

