#![allow(non_snake_case)]

use core::panic;
use std::{collections::HashMap, sync::{Mutex, Arc}};
use nalgebra::{DMatrix, SVD};
use ndarray::{Array, Array1, ArrayD, Dim, IxDynImpl};
use rayon::{iter::IndexedParallelIterator, prelude::{IntoParallelRefIterator, ParallelIterator}};
use crate::{common::{generic_error::GenericError, progress_bar}, model::identifier_mapper::{self, IdentifierMapper}};
use super::{metric::Metric, distances::DistancesTrait};
use ndarray::{Array2, Axis};
use rand::prelude::*;
use rand::SeedableRng;
use ndarray_rand::rand_distr::Uniform;

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

    // fn mds(dissimilarity_matrix: DMatrix<f64>, dimensions: usize) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
    //     // Returns a hashmap of the points in the new space, the indices DO NOT represent the identifiers
    //     // dbg!(&dissimilarity_matrix);
    //     let mut m = dissimilarity_matrix.map(|x| -0.5 * x.powi(2));

    //     // double centre the rows/columns
    //     let row_means = m.row_mean();
    //     let col_means = m.column_mean();
    //     let total_mean = row_means.mean();

    //     for i in 0..m.nrows() {
    //         for j in 0..m.ncols() {
    //             m[(i, j)] += total_mean - row_means[i] - col_means[j];
    //         }
    //     }

    //     // dbg!(&m);

    //     // take the SVD of the double centred matrix, and return the
    //     // points from it
    //     let svd = SVD::new(m, true, true);
    //     let eigen_values = svd.singular_values.map(|x| x.sqrt());

    //     let u = svd.u
    //         .ok_or(GenericError::new("Error getting U matrix from SVD", file!(), &line!()))?;

    //     let mut result = DMatrix::zeros(u.nrows(), dimensions);
    //     // dbg!(&eigen_values);
    //     // dbg!(&result);

    //     for i in 0..u.nrows() {
    //         for j in 0..dimensions {
    //             result[(i, j)] = u[(i, j)] * eigen_values[j];
    //         }
    //     }

    //     // Convert result to hashmap
    //     let n_rows = result.nrows();
    //     let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
    //     for i in 0..n_rows {
    //         let x = result[(i, 0)];
    //         let y = result[(i, 1)];
    //         xys.insert(i as u32, (x, y));
    //     }

    //     return Ok(xys);
    // }

    fn euclideanNorm(v: &Array1<f64>) -> f64 {
        let mut sum = 0.0;
        for i in 0..v.len(){
            sum += v[i].powi(2);
        }

        return sum.sqrt();
    }

    fn SMACOF(d: &Array2<f64>, p: usize, max_iter: usize, tol: f64, random_state: Option<u64>) -> HashMap<u32, (f64, f64)> {
        // 20s, 
        println!("      Initializing SMACOF...");
        let n = d.shape()[0];
        let mut rng = match random_state {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
    
        // Initialize points randomly in p-dimensional space
        let mut x: Array2<f64> = Array::zeros((n, p));
        let step = Uniform::new(0.0, 1.0);
        for i in 0..n {
            for j in 0..p {
                x[[i, j]] = step.sample(&mut rng);
            }
        }
    
        // Add a small epsilon to avoid division by zero
        let epsilon = 1e-10;
        let d = d + epsilon;
        let w = 1.0 / (d.mapv(|v| v.powi(2)));
    
        // Compute stress function
        let compute_stress = |x: &Array2<f64>| -> f64 {
            let mut stress = 0.0;
            for i in 0..n {
                for j in (i + 1)..n {
                    let dist_ij = Coordinates::euclideanNorm(&(&x.row(i) - &x.row(j)));
                    stress += w[[i, j]] * (dist_ij - d[[i, j]]).powi(2);
                }
            }
            stress
        };

        // Majorization loop
        let initial_stress = compute_stress(&x);
        println!("\t\tInitial stress: {}", initial_stress);
        let bar = progress_bar::new(max_iter as u64, "\t\tIterations");
        for _ in 0..max_iter {
            let stress_prev = compute_stress(&x);
    
            // Update each point using the majorization step
            for i in 0..n {
                let mut numerator = Array2::zeros((1, p));
                let mut denominator = 0.0;
                for j in 0..n {
                    if i != j {
                        let dist_ij = Coordinates::euclideanNorm(&(&x.row(i) - &x.row(j)));
                        let dist_ij = if dist_ij == 0.0 { epsilon } else { dist_ij };
                        let term1 = x.row(j).to_owned() + ((d[[i, j]] / dist_ij) * (&x.row(i) - &x.row(j)).to_owned());
                        let contrib = w[[i, j]] * d[[i, j]] * term1;
                        numerator = numerator + contrib.insert_axis(Axis(0));
                        denominator += w[[i, j]] * d[[i, j]];
                    }
                }
                x.row_mut(i).assign(&(numerator.sum_axis(Axis(0)) / denominator));
            }

            bar.inc(1);
    
            // Compute new stress
            let stress_new = compute_stress(&x);
    
            // Check for convergence
            if (stress_prev - stress_new).abs() < tol {
                break;
            }
        }
        bar.finish();
        let final_stress = compute_stress(&x);
        println!();
        println!("\t\tFinal stress: {}", final_stress);
    
        // Convert result to hashmap
        let mut xys: HashMap<u32, (f64, f64)> = HashMap::new();
        for i in 0..n {
            let n_x: f64 = x[[i, 0]];
            let n_y: f64 = x[[i, 1]];
            xys.insert(i as u32, (n_x, n_y));
        }

        return xys;
    }

    fn indexToIdentifier(index: &u32, ordered_visible_identifiers: &Vec<u32>) -> Result<u32, GenericError> {
        return Ok(*ordered_visible_identifiers.get(*index as usize)
            .ok_or(GenericError::new("Identifier not found", file!(), &line!()))?);
    }


    fn scaleCoordinates(xys: &HashMap<u32, (f64, f64)>) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if xys.len() == 0 { return Ok(HashMap::new()); }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        
        for (_, (x, y)) in xys.iter() {
            if x > &x_max { x_max = *x; }
            if x < &x_min { x_min = *x; }

            if y > &y_max { y_max = *y; }
            if y < &y_min { y_min = *y; }
        }

        let x_abs_max = x_max.abs().max(x_min.abs());
        let y_abs_max = y_max.abs().max(y_min.abs());
        x_min /= x_abs_max;
        x_max /= x_abs_max;
        y_min /= y_abs_max;
        y_max /= y_abs_max;

        let xys: HashMap<u32, (f64, f64)> = xys.iter()
            .map(|(i, coords)| (*i, (coords.0.clone() / x_abs_max, coords.1.clone() / y_abs_max)))
            .collect(); // Scale to [-1, 1]

        let left_space = 1.0 + x_min; // This one or bellow is going to be zero
        let right_space = 1.0 - x_max; // This one or bellow is going to be zero

        let top_space = 1.0 - y_max; // This one or bellow is going to be zero
        let bottom_space = 1.0 + y_min; // This one or bellow is going to be zero

        let x_delta = match right_space > left_space {
            true => (left_space + right_space) / 2.0, // Shift to the right
            false => -(left_space + right_space) / 2.0, // Shift to the left
        };

        let y_delta = match top_space > bottom_space {
            true => (top_space + bottom_space) / 2.0, // Shift to the top
            false => -(top_space + bottom_space) / 2.0, // Shift to the bottom
        };

        let x_max = x_max + x_delta; // x_max and x_min should be equal
        let x_min = x_min + x_delta; // x_max and x_min should be equal
        let y_max = y_max + y_delta; // y_max and y_min should be equal
        let y_min = y_min + y_delta; // y_max and y_min should be equal
        let x_scaling_factor = 1.0/x_max; // x_max will be 1 and x_min will be -1
        let y_scaling_factor = 1.0/y_max; // y_max will be 1 and y_min will be -1
    
        let mut scaled_coordinates: HashMap<u32, (f64, f64)> = HashMap::new();
        for (i, (x, y)) in xys.iter(){
            let scaled_x = x_scaling_factor * (*x + x_delta);
            let scaled_y = y_scaling_factor * (*y + y_delta);

            if scaled_x > 1.001 || scaled_y > 1.001 {
                panic!("Scaled coordinates are out of bounds: x: {}, y: {}", scaled_x, scaled_y);
            }

            scaled_coordinates.insert(*i, (scaled_x, scaled_y));
        }

        return Ok(scaled_coordinates);
    }

    fn calculate<T: DistancesTrait>(distances: &T) -> Result<HashMap<u32, (f64, f64)>, GenericError> {
        if distances.get().len() == 0{ // Only one datapoint, no need to calculate MDS
            let mut xys = HashMap::new();
            xys.insert(1, (0.0, 0.0));
            return Ok(xys);
        }

        println!("  Applying Multi Dimensional Scaling...");
        let n: usize = distances.get().len();
        let dissimilarity_matrix: DMatrix<f64> = Coordinates::buildDissimilarityMatrix(distances, n)?;
        let dissimilarity_matrix: Array2<f64> = Array2::from_shape_vec((n, n), dissimilarity_matrix.data.as_vec().clone())
            .map_err(|_| GenericError::new("Error converting dissimilarity matrix to ndarray", file!(), &line!()))?;
        
        let xys: HashMap<u32, (f64, f64)> = Coordinates::SMACOF(&dissimilarity_matrix, 2, 300, 1e-6, Some(42));

        let mut ordered_visible_identifiers: Vec<u32> = distances.get().keys().cloned().collect();
        ordered_visible_identifiers.sort();
        let scaled_xys: HashMap<u32, (f64, f64)> = Coordinates::scaleCoordinates(&xys)?;

        let mut result: HashMap<u32, (f64, f64)> = HashMap::new();
        for entry in scaled_xys.iter(){
            let i = entry.0;
            let identifier = Coordinates::indexToIdentifier(i, &ordered_visible_identifiers)?;

            result.insert(identifier, *entry.1);
        }

        return Ok(result);
    }
}

