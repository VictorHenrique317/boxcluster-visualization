#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::{DMatrix, SVD};
use ndarray::{IxDynImpl, Dim, ArrayD, Array};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use crate::{model::identifier_mapper::IdentifierMapper, common::generic_error::GenericError};
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
    pub fn new<T: DistancesTrait>(identifier_mapper: &IdentifierMapper, distances: &T) -> Result<Coordinates, GenericError>{
        println!("  Coordinates...");
        return Ok(
            Coordinates { 
                value: Coordinates::calculate(identifier_mapper, distances)?,
            }
        );
    }

    fn buildDissimilarityMatrix<T: DistancesTrait>(distances: &T, n: usize) -> Result<DMatrix<f64>, GenericError> {
        let size: Vec<usize> = vec![n, n];
        let distance_matrix: Arc<Mutex<ArrayD<f64>>> = Arc::new(Mutex::new(Array::zeros(Dim(size.clone())).into_dyn()));
        
        let i: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
        distances.get().par_iter().try_for_each(|(_, columns)| 
                -> Result<(), GenericError> {

            let mut i_lock = i.lock().map_err(|_| GenericError::new("Error while getting i thread lock", file!(), &line!()))?;
            let i_value = i_lock.clone();
            *i_lock += 1;
            drop(i_lock);
            
            let mut j: usize = 0;
            for (_, distance) in columns{
                let index: Dim<IxDynImpl> = Dim(vec![i_value, j]);
                
                let mut distance_matrix_lock = distance_matrix.lock()
                    .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                let matrix_value = distance_matrix_lock.get_mut(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;

                *matrix_value = *distance;
                j += 1;
            }

            return Ok(());
        })?;

        let distance_matrix = distance_matrix.lock()
            .as_mut()
            .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?
            .clone();
        
        let mut dissimilarity_matrix = DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                let matrix_value = distance_matrix.get(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;

                dissimilarity_matrix[(i, j)] = *matrix_value;
            }
        }

        return Ok(dissimilarity_matrix);
    }

    fn mds<T: DistancesTrait>(dissimilarity_matrix: DMatrix<f64>, dimensions: usize, original_distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        // square distances
        let mut m = dissimilarity_matrix.map(|x| -0.5 * x.powi(2));

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

        let u = svd.u
            .ok_or(GenericError::new("Error getting U matrix from SVD", file!(), &line!()))?;

        let mut result = DMatrix::zeros(u.nrows(), dimensions);

        for i in 0..u.nrows() {
            for j in 0..dimensions {
                result[(i, j)] = u[(i, j)] * eigen_values[j];
            }
        }

        // Convert result to hashmap
        let n_rows = result.nrows();
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        let mut visible_identifiers: Vec<u32> = original_distances.get().keys().map(|x| x.clone()).collect();
        visible_identifiers.sort();
        for i in 0..n_rows {
            let identifier = visible_identifiers[i];
            let x = result[(i, 0)];
            let y = result[(i, 1)];
            xys.insert(identifier, (x, y));
        }

        return Ok(xys);
    }

    fn calculate<T: DistancesTrait>(identifier_mapper: &IdentifierMapper, distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if distances.get().len() == 0{ // Only one datapoint, no need to calculate MDS
            let mut xys = HashMap::new();
            xys.insert(1, (0.0, 0.0));
            return Ok(xys);
        }

        println!("  Applying Multi Dimensional Scaling...");
        let n: usize = distances.get().len();
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n)?;
        let xys: Result<HashMap<u32, (f64, f64)>, GenericError> = Coordinates::mds(dissimilarity_matrix, 2, distances);
        return xys;
    }
}

