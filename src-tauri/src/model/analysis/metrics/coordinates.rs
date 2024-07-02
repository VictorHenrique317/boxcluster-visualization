#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::{DMatrix, SVD};
use ndarray::{IxDynImpl, Dim, ArrayD, Array};
use rayon::{iter::IndexedParallelIterator, prelude::{IntoParallelRefIterator, ParallelIterator}};
use crate::{common::generic_error::GenericError, model::identifier_mapper::IdentifierMapper};
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
    pub fn new<T: DistancesTrait>(distances: &T) -> Result<Coordinates, GenericError>{
        println!("  Coordinates...");
        return Ok(
            Coordinates { 
                value: Coordinates::calculate(distances)?,
            }
        );
    }

    fn printMatrix(matrix: &DMatrix<f64>){
        println!("Printing matrix:");
        for i in 0..matrix.nrows(){
            for j in 0..matrix.ncols(){
                print!("{:.2} ", matrix[(i, j)]);
            }
            println!("");
        }
        println!("");
    }

    // fn printHashMapAsMatrix(map: &HashMap<u32, HashMap<u32, f64>>){
    //     let mut lines = map.keys().cloned().collect::<Vec<u32>>();
    //     lines.sort();

    //     for i in lines.iter(){
    //         let mut columns = map.get(i).expect("Key not found").keys().cloned().collect::<Vec<u32>>();
    //         columns.sort();

    //         for j in columns.iter(){
    //             print!("{:.2} ", map.get(i).expect("Key not found").get(j).expect("Key not found"));
    //         }

    //         println!("");
    //     }
    // }

    fn buildDissimilarityMatrix<T: DistancesTrait>(distances: &T, n: usize) -> Result<DMatrix<f64>, GenericError> {
        let size: Vec<usize> = vec![n, n];
        let distance_matrix: Arc<Mutex<ArrayD<f64>>> = Arc::new(Mutex::new(Array::zeros(Dim(size.clone())).into_dyn()));

        let mut visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        visible_identifiers.sort();
        let visible_identifiers2: Vec<u32> = visible_identifiers.clone();
        
        let distances: Arc<Mutex<HashMap<u32, HashMap<u32, f64>>>> = Arc::new(Mutex::new(distances.get().clone()));
        visible_identifiers.par_iter().enumerate().try_for_each(|(i, &identifier_1)| -> Result<(), GenericError> {
            
            for (j, identifier_2) in visible_identifiers2.iter().enumerate(){
                let distances_lock = distances.lock()
                    .map_err(|_| GenericError::new("Error while getting distances thread lock", file!(), &line!()))?;
                
                let mut distance: f64 = 0.0;
                if identifier_1 != *identifier_2 {
                    distance = distances_lock.get(&identifier_1)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_1), file!(), &line!()))?
                    .get(identifier_2)
                    .ok_or(GenericError::new(&format!("Identifier {} not found", identifier_2), file!(), &line!()))?.clone();
                }
                
                let index: Dim<IxDynImpl> = Dim(vec![i, j]);
                
                let mut distance_matrix_lock = distance_matrix.lock()
                    .map_err(|_| GenericError::new("Error while getting distance matrix thread lock", file!(), &line!()))?;

                let matrix_value = distance_matrix_lock.get_mut(&index)
                    .ok_or(GenericError::new(&format!("Index {:?} does not exist on distance matrix", &index), file!(), &line!()))?;
                
                *matrix_value = distance;
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

    fn mds(dissimilarity_matrix: DMatrix<f64>, dimensions: usize) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        // Returns a hashmap of the points in the new space, the indices DO NOT represent the identifiers
        // dbg!(&dissimilarity_matrix);
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

        // dbg!(&m);

        // take the SVD of the double centred matrix, and return the
        // points from it
        let svd = SVD::new(m, true, true);
        let eigen_values = svd.singular_values.map(|x| x.sqrt());

        let u = svd.u
            .ok_or(GenericError::new("Error getting U matrix from SVD", file!(), &line!()))?;

        let mut result = DMatrix::zeros(u.nrows(), dimensions);
        // dbg!(&eigen_values);
        // dbg!(&result);

        for i in 0..u.nrows() {
            for j in 0..dimensions {
                result[(i, j)] = u[(i, j)] * eigen_values[j];
            }
        }

        // Convert result to hashmap
        let n_rows = result.nrows();
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n_rows {
            let x = result[(i, 0)];
            let y = result[(i, 1)];
            xys.insert(i as u32, (x, y));
        }

        return Ok(xys);
    }

    fn calculate<T: DistancesTrait>(distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if distances.get().len() == 0{ // Only one datapoint, no need to calculate MDS
            let mut xys = HashMap::new();
            xys.insert(1, (0.0, 0.0));
            return Ok(xys);
        }

        println!("  Applying Multi Dimensional Scaling...");
        dbg!(distances.get());
        let n: usize = distances.get().len();
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n)?;
        let xys: HashMap<u32, (f64, f64)> = Coordinates::mds(dissimilarity_matrix, 2)?;

        let mut visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        visible_identifiers.sort();

        let mut result: HashMap<u32, (f64, f64)> = HashMap::new();
        for entry in xys.iter(){
            let i = entry.0;
            let identifier = visible_identifiers.get(*i as usize)
                .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?;

            result.insert(*identifier, *entry.1);
        }

        return Ok(result);
    }
}

