// https://www.sheshbabu.com/posts/rust-module-system/
#![allow(non_snake_case)]
pub mod common;
pub mod controller;
pub mod services;
pub mod model;
pub mod database;
use std::{collections::HashMap, hash::Hash};

use model::analysis::metrics::distances::DistancesTrait;
use nalgebra::DVector;
use nalgebra::{DMatrix, Dynamic, VecStorage, Matrix, SymmetricEigen, MatrixMN};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use rand::{Rng, thread_rng};
use serde_json::Error as SerdeJsonError;


use ndarray::{ArrayView2, Array2, Axis};
use services::application::application_service::ApplicationService;

pub fn main() {
    testDag();
    return;
    let distances: DMatrix<f64> = DMatrix::from_row_slice(10, 10, &[
        0.0, 587.0, 1212.0, 701.0, 1936.0, 604.0, 748.0, 2139.0, 2182.0, 543.0,
        587.0, 0.0, 920.0, 940.0, 1745.0, 1188.0, 713.0, 1858.0, 1737.0, 597.0,
        1212.0, 920.0, 0.0, 879.0, 831.0, 1726.0, 1631.0, 949.0, 1021.0, 1494.0,
        701.0, 940.0, 879.0, 0.0, 1374.0, 968.0, 1420.0, 1645.0, 1891.0, 1220.0,
        1936.0, 1745.0, 831.0, 1374.0, 0.0, 2339.0, 2451.0, 347.0, 959.0, 2300.0,
        604.0, 1188.0, 1726.0, 968.0, 2339.0, 0.0, 1092.0, 2594.0, 2734.0, 923.0,
        748.0, 713.0, 1631.0, 1420.0, 2451.0, 1092.0, 0.0, 2571.0, 2408.0, 205.0,
        2139.0, 1858.0, 949.0, 1645.0, 347.0, 2594.0, 2571.0, 0.0, 678.0, 2442.0,
        2182.0, 1737.0, 1021.0, 1891.0, 959.0, 2734.0, 2408.0, 678.0, 0.0, 2329.0,
        543.0, 597.0, 1494.0, 1220.0, 2300.0, 923.0, 205.0, 2442.0, 2329.0, 0.0]);

    // let mut distances: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
    // distances.insert(4, HashMap::from_iter(vec![(19, 0.1279),(10, 0.2337),(15, 0.0797),(6, 0.234),(12, 0.109),(1, 0.2684),(5, 0.2293),(8, 0.2493),(3, 0.2627),(11, 0.1242),(20, 0.1198),(9, 0.2346),(17, 0.1454),(13, 0.1486),(7, 0.2142),(14, 0.073),(18, 0.1201),(2, 0.259),(16, 0.1176),]));
    // distances.insert(17, HashMap::from_iter(vec![(5, 0.1366),(4, 0.1454),(8, 0.1292),(12, 0.0679),(3, 0.1446),(2, 0.1298),(16, 0.0579),(19, 0.0777),(7, 0.1205),(11, 0.0599),(9, 0.1238),(20, 0.07),(10, 0.1316),(13, 0.0825),(14, 0.0394),(6, 0.136),(15, 0.0454),(1, 0.1605),(18, 0.0681),]));
    // distances.insert(6, HashMap::from_iter(vec![(2, 0.2387),(16, 0.1084),(19, 0.1197),(1, 0.2773),(10, 0.2295),(8, 0.2444),(13, 0.1375),(5, 0.2413),(7, 0.2321),(4, 0.234),(15, 0.082),(9, 0.205),(12, 0.1056),(14, 0.0708),(3, 0.2266),(11, 0.0909),(17, 0.136),(18, 0.1119),(20, 0.1248),]));
    // distances.insert(20, HashMap::from_iter(vec![(11, 0.0527),(7, 0.1211),(10, 0.107),(4, 0.1198),(1, 0.1486),(8, 0.1212),(9, 0.1165),(5, 0.1247),(3, 0.1246),(2, 0.1259),(15, 0.0402),(6, 0.1248),(19, 0.0681),(18, 0.0511),(14, 0.0366),(12, 0.0612),(13, 0.0773),(16, 0.0618),(17, 0.07),]));
    // distances.insert(19, HashMap::from_iter(vec![(8, 0.1214),(9, 0.1209),(11, 0.0532),(12, 0.0607),(10, 0.1018),(16, 0.0643),(15, 0.043),(4, 0.1279),(5, 0.1274),(13, 0.0773),(6, 0.1197),(7, 0.1209),(3, 0.1202),(20, 0.0681),(2, 0.1333),(14, 0.0376),(17, 0.0777),(18, 0.0669),(1, 0.1531),]));
    // distances.insert(11, HashMap::from_iter(vec![(6, 0.0909),(15, 0.0379),(10, 0.0938),(3, 0.1077),(18, 0.0544),(8, 0.1026),(9, 0.0961),(14, 0.0345),(20, 0.0527),(12, 0.0466),(4, 0.1242),(7, 0.1024),(2, 0.1225),(1, 0.1365),(16, 0.0444),(19, 0.0532),(13, 0.0587),(17, 0.0599),(5, 0.1012),]));
    // distances.insert(14, HashMap::from_iter(vec![(13, 0.0402),(7, 0.0658),(3, 0.0745),(20, 0.0366),(5, 0.0709),(18, 0.0369),(8, 0.0708),(4, 0.073),(6, 0.0708),(12, 0.035),(17, 0.0394),(9, 0.067),(1, 0.0849),(11, 0.0345),(10, 0.0555),(16, 0.0369),(2, 0.0783),(19, 0.0376),(15, 0.0258),]));
    // distances.insert(1, HashMap::from_iter(vec![(10, 0.2047),(3, 0.2821),(5, 0.282),(8, 0.247),(6, 0.2773),(12, 0.1313),(17, 0.1605),(20, 0.1486),(19, 0.1531),(16, 0.134),(14, 0.0849),(4, 0.2684),(18, 0.1521),(13, 0.1654),(15, 0.1079),(2, 0.2542),(9, 0.2545),(7, 0.248),(11, 0.1365),]));
    // distances.insert(18, HashMap::from_iter(vec![(4, 0.1201),(8, 0.1274),(14, 0.0369),(7, 0.1235),(10, 0.1204),(13, 0.0794),(19, 0.0669),(3, 0.1219),(5, 0.1248),(16, 0.0594),(9, 0.1091),(17, 0.0681),(20, 0.0511),(11, 0.0544),(12, 0.0469),(1, 0.1521),(6, 0.1119),(2, 0.1271),(15, 0.0442),]));
    // distances.insert(10, HashMap::from_iter(vec![(5, 0.2067),(12, 0.1048),(19, 0.1018),(3, 0.2083),(17, 0.1316),(18, 0.1204),(9, 0.2161),(16, 0.1082),(2, 0.2538),(7, 0.2338),(15, 0.0755),(14, 0.0555),(8, 0.1866),(4, 0.2337),(1, 0.2047),(20, 0.107),(13, 0.1317),(6, 0.2295),(11, 0.0938),]));
    // distances.insert(16, HashMap::from_iter(vec![(10, 0.1082),(19, 0.0643),(9, 0.1094),(1, 0.134),(5, 0.1078),(6, 0.1084),(8, 0.1108),(7, 0.1039),(2, 0.122),(13, 0.077),(3, 0.1082),(4, 0.1082),(11, 0.0444),(12, 0.0444),(14, 0.0369),(15, 0.0369),(17, 0.0369),(18, 0.0594),(20, 0.0618),]));
    // distances.insert(5, HashMap::from_iter(vec![(10, 0.2067),(4, 0.2293),(14, 0.0709),(8, 0.2405),(20, 0.1247),(15, 0.08),(13, 0.1447),(1, 0.282),(9, 0.2138),(2, 0.2596),(7, 0.1959),(11, 0.1012),(17, 0.1366),(19, 0.1274),(3, 0.2016),(6, 0.2413),(16, 0.1078),(12, 0.1081),(18, 0.1248),]));
    // distances.insert(12, HashMap::from_iter(vec![(1, 0.1313),(8, 0.1011),(17, 0.0679),(2, 0.1132),(4, 0.109),(3, 0.1154),(18, 0.0469),(14, 0.035),(16, 0.0546),(10, 0.1048),(19, 0.0607),(13, 0.0609),(5, 0.1081),(6, 0.1056),(9, 0.1076),(20, 0.0612),(7, 0.108),(11, 0.0466),(15, 0.0389),]));
    // distances.insert(15, HashMap::from_iter(vec![(1, 0.1079),(11, 0.0379),(20, 0.0402),(10, 0.0755),(2, 0.0886),(5, 0.08),(13, 0.0459),(3, 0.0979),(18, 0.0442),(7, 0.0767),(16, 0.0411),(8, 0.0877),(4, 0.0797),(12, 0.0389),(9, 0.0772),(6, 0.082),(19, 0.043),(17, 0.0454),(14, 0.0258),]));
    // distances.insert(8, HashMap::from_iter(vec![(3, 0.2388),(5, 0.2405),(15, 0.0877),(17, 0.1292),(14, 0.0708),(18, 0.1274),(20, 0.1212),(10, 0.1866),(7, 0.2445),(12, 0.1011),(11, 0.1026),(1, 0.247),(13, 0.1425),(2, 0.2612),(19, 0.1214),(16, 0.1108),(9, 0.1989),(4, 0.2493),(6, 0.2444),]));
    // distances.insert(13, HashMap::from_iter(vec![(2, 0.1562),(10, 0.1317),(9, 0.1366),(19, 0.0773),(12, 0.0609),(11, 0.0587),(16, 0.054),(14, 0.0402),(5, 0.1447),(15, 0.0459),(3, 0.1462),(17, 0.0825),(20, 0.0773),(18, 0.0794),(1, 0.1654),(4, 0.1486),(7, 0.1262),(6, 0.1375),(8, 0.1425),]));
    // distances.insert(3, HashMap::from_iter(vec![(19, 0.1202),(13, 0.1462),(18, 0.1219),(17, 0.1446),(12, 0.1154),(5, 0.2016),(15, 0.0979),(4, 0.2627),(9, 0.1928),(14, 0.0745),(6, 0.2266),(1, 0.2821),(11, 0.1077),(8, 0.2388),(7, 0.2572),(16, 0.1153),(10, 0.2083),(20, 0.1246),(2, 0.2713),]));
    // distances.insert(9, HashMap::from_iter(vec![(2, 0.2359),(14, 0.067),(3, 0.1928),(12, 0.1076),(4, 0.2346),(5, 0.2138),(7, 0.2402),(16, 0.1094),(13, 0.1366),(1, 0.2545),(10, 0.2161),(18, 0.1091),(17, 0.1238),(6, 0.205),(8, 0.1989),(20, 0.1165),(11, 0.0961),(19, 0.1209),(15, 0.0772),]));
    // distances.insert(2, HashMap::from_iter(vec![(5, 0.2596),(7, 0.2329),(12, 0.1132),(6, 0.2387),(10, 0.2538),(15, 0.0886),(8, 0.2612),(3, 0.2713),(13, 0.1562),(18, 0.1271),(14, 0.0783),(20, 0.1259),(11, 0.1225),(1, 0.2542),(16, 0.122),(17, 0.1298),(9, 0.2359),(4, 0.259),(19, 0.1333),]));
    // distances.insert(7, HashMap::from_iter(vec![(2, 0.2329),(12, 0.108),(14, 0.0658),(19, 0.1209),(10, 0.2338),(5, 0.1959),(8, 0.2445),(20, 0.1211),(15, 0.0767),(6, 0.2321),(13, 0.1262),(11, 0.1024),(1, 0.248),(16, 0.1039),(4, 0.2142),(3, 0.2572),(9, 0.2402),(18, 0.1235),(17, 0.1205),]));

    // let distances: DMatrix<f64> = hashmap_to_dmatrix(distances);

    let result = testMds(distances, 2);

    for i in 0..result.nrows() {
        let point: Vec<f64> = result.row(i).iter().cloned().collect();
        println!("Point {}: {:?}", i, point);
    }
        
}

// fn testMds(distances: ArrayView2<f64>, dimensions: usize) -> Array2<f64> {
//     let n = distances.shape()[0];

//     // Square the distances
//     let mut squared_distances = distances.mapv(|x| x.powi(2));

//     // Compute the double centered matrix
//     let row_means = squared_distances.mean_axis(Axis(1)).unwrap();
//     let col_means = squared_distances.mean_axis(Axis(0)).unwrap();
//     let total_mean = squared_distances.mean().unwrap();
//     for i in 0..n {
//         for j in 0..n {
//             squared_distances[[i, j]] += total_mean;
//             squared_distances[[i, j]] -= row_means[i];
//             squared_distances[[i, j]] -= col_means[j];
//         }
//     }
//     squared_distances /= -2.0;

//     // Convert ndarray to nalgebra matrix for eigen decomposition
//     let matrix: Matrix<f64, Dynamic, Dynamic> = Matrix::from_iterator(n, n, squared_distances.iter().cloned());

//     // Compute the eigen decomposition
//     let SymmetricEigen { eigenvectors, eigenvalues } = SymmetricEigen::new(matrix);

//     // Take the square root of the eigenvalues
//     let eigenvalues_sqrt: MatrixMN<_, U1, Dynamic> = eigenvalues.map(|x| x.sqrt());

//     // Multiply eigenvectors with the square root of the eigenvalues to get the coordinates
//     let coordinates = eigenvectors * eigenvalues_sqrt;

//     // Convert nalgebra matrix back to ndarray
//     let coordinates_ndarray: Array2<_> = Array2::from_shape_vec((n, dimensions), coordinates.column_iter().collect()).unwrap();

//     coordinates_ndarray
// }

#[derive(Serialize, Deserialize)]
struct Record {
    map: HashMap<u32, HashMap<u32, f64>>,
}

fn read_json_to_hashmap(file_path: &str) -> Result<HashMap<u32, HashMap<u32, f64>>> {
    let file = File::open(file_path).map_err(SerdeJsonError::io)?;
    let reader = BufReader::new(file);
    let record: Record = serde_json::from_reader(reader)?;
    Ok(record.map)
}

fn testMds(distances: DMatrix<f64>, dimensions: usize) -> DMatrix<f64> {
    let mut m = distances.map(|x| -0.5 * x.powi(2));

    let row_means = m.row_mean();
    let col_means = m.column_mean();
    let total_mean = m.mean();

    for i in 0..m.nrows() {
        for j in 0..m.ncols() {
            m[(i, j)] += total_mean - row_means[i] - col_means[j];
        }
    }

    let svd = m.svd(true, true);
    let mut singular_values = svd.singular_values.map(|x| x.sqrt());
    let mut u = svd.u.unwrap();

    // Create a vector of indices from 0 to n
    let mut indices: Vec<usize> = (0..singular_values.len()).collect();

    // Sort the indices by singular values in descending order
    indices.sort_by(|&i, &j| singular_values[j].partial_cmp(&singular_values[i]).unwrap());

    // Reorder singular values and U matrix using sorted indices
    let singular_values_vec: Vec<f64> = indices.iter().map(|&i| singular_values[i]).collect();
    singular_values = DVector::from_vec(singular_values_vec);

    let u_vec: Vec<DVector<f64>> = indices.iter().map(|&i| u.column(i).into_owned()).collect();
    u = DMatrix::from_columns(&u_vec);

    let mut result = DMatrix::zeros(u.nrows(), dimensions);

    for i in 0..u.nrows() {
        for j in 0..dimensions {
            result[(i, j)] = u[(i, j)] * singular_values[j];
        }
    }

    result
}


fn hashmap_to_dmatrix(hashmap: HashMap<u32, HashMap<u32, f64>>) -> DMatrix<f64> {
    let keys: Vec<_> = hashmap.keys().cloned().collect();
    let n = keys.len();

    let mut data = Vec::new();
    for key_i in &keys {
        for key_j in &keys {
            let value = hashmap.get(key_i).and_then(|row| row.get(key_j)).cloned().unwrap_or(0.0);
            data.push(value);
        }
    }

    DMatrix::from_row_slice(n, n, &data)
}

fn testDag(){
    // let path = "../tests/test_data/real1.txt".to_owned(); 
    // let path = "../tests/test_data/4k-big-patterns.txt".to_owned(); 
    // let path = "../tests/test_data/9k-small-patterns.txt".to_owned();
    // let path = "../tests/test_data/simple-msuper.txt".to_owned();
    // let path = "../tests/test_data/simple-msub-2.txt".to_owned();
    // // let path = "../tests/test_data/synth-2.txt".to_owned();
    // let path = "../tests/test_data/paf-1.txt".to_owned();
    // let path = "../tests/test_data/paf-1.processed".to_owned();
    // let path = "../tests/test_data/real-1.txt".to_owned();
    // let path = "../tests/test_data/dataset-co16.fuzzy_tensor".to_owned();
    
    // let tensor_path = "../tests/test_data/tensors/4k-big-patterns-fuzzytensor.txt".to_owned();
    // let patterns_path = "../tests/test_data/4k-big-patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/a.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/a_patterns.txt".to_owned();
    
    // let tensor_path = "../tests/test_data/distance_test/b.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/b_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/distance_test/c.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test/c_patterns.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/dataset-co16.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/synth-100-3d-co16.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/retweets-sparser.txt".to_owned();
    // let patterns_path = "../tests/test_data/distance_test_patterns/158-retweets-sparser.txt".to_owned();

    // let tensor_path = "../tests/test_data/tensors/primary-school.txt".to_owned();
    // let patterns_path = "../tests/test_data/other_patterns/paf-1.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets3d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets3d_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets2d_patterns.txt".to_owned();

    let tensor_path = "tests/test_data/tensors/synth_co1.txt".to_owned();
    let patterns_path = "tests/test_data/other_patterns/synth_co1.txt".to_owned();

    // let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();
    // let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

    // let tensor_path = "tests/test_data/tensors/retweets3d.txt".to_owned();
    // let patterns_path = "tests/test_data/other_patterns/retweets3d_patterns.txt".to_owned();

    let mut application_manager = ApplicationService::default();
    application_manager.init(&tensor_path, &patterns_path);

    let rss_evolution = application_manager.getTruncatedRssEvolution();
    // dbg!(rss_evolution);
    // dbg!(rss_evolution.len());

    // application_manager.truncateModel(&100);
    // let rss_evolution = application_manager.getRssEvolution().iter().map(|(_, model_rss)| model_rss.clone()).collect::<Vec<f64>>();
    // dbg!(rss_evolution.len());
    // Starts at pattern 11
    // let mut test: HashSet<Dim<IxDynImpl>> = HashSet::new(); // 2.264
}

// {
//     4: {
//         19: 0.1279,
//         10: 0.2337,
//         15: 0.0797,
//         6: 0.234,
//         12: 0.109,
//         1: 0.2684,
//         5: 0.2293,
//         8: 0.2493,
//         3: 0.2627,
//         11: 0.1242,
//         20: 0.1198,
//         9: 0.2346,
//         17: 0.1454,
//         13: 0.1486,
//         7: 0.2142,
//         14: 0.073,
//         18: 0.1201,
//         2: 0.259,
//         16: 0.1176,
//     },
//     17: {
//         5: 0.1366,
//         4: 0.1454,
//         8: 0.1292,
//         12: 0.0679,
//         3: 0.1446,
//         2: 0.1298,
//         16: 0.0579,
//         19: 0.0777,
//         7: 0.1205,
//         11: 0.0599,
//         9: 0.1238,
//         20: 0.07,
//         10: 0.1316,
//         13: 0.0825,
//         14: 0.0394,
//         6: 0.136,
//         15: 0.0454,
//         1: 0.1605,
//         18: 0.0681,
//     },
//     6: {
//         2: 0.2387,
//         16: 0.1084,
//         19: 0.1197,
//         1: 0.2773,
//         10: 0.2295,
//         8: 0.2444,
//         13: 0.1375,
//         5: 0.2413,
//         7: 0.2321,
//         4: 0.234,
//         15: 0.082,
//         9: 0.205,
//         12: 0.1056,
//         14: 0.0708,
//         3: 0.2266,
//         11: 0.0909,
//         17: 0.136,
//         18: 0.1119,
//         20: 0.1248,
//     },
//     20: {
//         11: 0.0527,
//         7: 0.1211,
//         10: 0.107,
//         4: 0.1198,
//         1: 0.1486,
//         8: 0.1212,
//         9: 0.1165,
//         5: 0.1247,
//         3: 0.1246,
//         2: 0.1259,
//         15: 0.0402,
//         6: 0.1248,
//         19: 0.0681,
//         18: 0.0511,
//         14: 0.0366,
//         12: 0.0612,
//         13: 0.0773,
//         16: 0.0618,
//         17: 0.07,
//     },
//     19: {
//         8: 0.1214,
//         9: 0.1209,
//         11: 0.0532,
//         12: 0.0607,
//         10: 0.1018,
//         16: 0.0643,
//         15: 0.043,
//         4: 0.1279,
//         5: 0.1274,
//         13: 0.0773,
//         6: 0.1197,
//         7: 0.1209,
//         3: 0.1202,
//         20: 0.0681,
//         2: 0.1333,
//         14: 0.0376,
//         17: 0.0777,
//         18: 0.0669,
//         1: 0.1531,
//     },
//     11: {
//         6: 0.0909,
//         15: 0.0379,
//         10: 0.0938,
//         3: 0.1077,
//         18: 0.0544,
//         8: 0.1026,
//         9: 0.0961,
//         14: 0.0345,
//         20: 0.0527,
//         12: 0.0466,
//         4: 0.1242,
//         7: 0.1024,
//         2: 0.1225,
//         1: 0.1365,
//         16: 0.0444,
//         19: 0.0532,
//         13: 0.0587,
//         17: 0.0599,
//         5: 0.1012,
//     },
//     14: {
//         13: 0.0402,
//         7: 0.0658,
//         3: 0.0745,
//         20: 0.0366,
//         5: 0.0709,
//         18: 0.0369,
//         8: 0.0708,
//         4: 0.073,
//         6: 0.0708,
//         12: 0.035,
//         17: 0.0394,
//         9: 0.067,
//         1: 0.0849,
//         11: 0.0345,
//         10: 0.0555,
//         16: 0.0369,
//         2: 0.0783,
//         19: 0.0376,
//         15: 0.0258,
//     },
//     1: {
//         10: 0.2047,
//         3: 0.2821,
//         5: 0.282,
//         8: 0.247,
//         6: 0.2773,
//         12: 0.1313,
//         17: 0.1605,
//         20: 0.1486,
//         19: 0.1531,
//         16: 0.134,
//         14: 0.0849,
//         4: 0.2684,
//         18: 0.1521,
//         13: 0.1654,
//         15: 0.1079,
//         2: 0.2542,
//         9: 0.2545,
//         7: 0.248,
//         11: 0.1365,
//     },
//     18: {
//         4: 0.1201,
//         8: 0.1274,
//         14: 0.0369,
//         7: 0.1235,
//         10: 0.1204,
//         13: 0.0794,
//         19: 0.0669,
//         3: 0.1219,
//         5: 0.1248,
//         16: 0.0594,
//         9: 0.1091,
//         17: 0.0681,
//         20: 0.0511,
//         11: 0.0544,
//         12: 0.0469,
//         1: 0.1521,
//         6: 0.1119,
//         2: 0.1271,
//         15: 0.0442,
//     },
//     10: {
//         5: 0.2067,
//         12: 0.1048,
//         19: 0.1018,
//         3: 0.2083,
//         17: 0.1316,
//         18: 0.1204,
//         9: 0.2161,
//         16: 0.1082,
//         2: 0.2538,
//         7: 0.2338,
//         15: 0.0755,
//         14: 0.0555,
//         8: 0.1866,
//         4: 0.2337,
//         1: 0.2047,
//         20: 0.107,
//         13: 0.1317,
//         6: 0.2295,
//         11: 0.0938,
//     },
//     16: {
//         10: 0.1082,
//         19: 0.0643,
//         9: 0.1094,
//         1: 0.134,
//         5: 0.1078,
//         6: 0.1084,
//         8: 0.1108,
//         7: 0.1039,
//         2: 0.122,
//         13: 0.054,
//         17: 0.0579,
//         4: 0.1176,
//         12: 0.0546,
//         20: 0.0618,
//         18: 0.0594,
//         15: 0.0411,
//         14: 0.0369,
//         3: 0.1153,
//         11: 0.0444,
//     },
//     5: {
//         10: 0.2067,
//         4: 0.2293,
//         14: 0.0709,
//         8: 0.2405,
//         20: 0.1247,
//         15: 0.08,
//         13: 0.1447,
//         1: 0.282,
//         9: 0.2138,
//         2: 0.2596,
//         7: 0.1959,
//         11: 0.1012,
//         17: 0.1366,
//         19: 0.1274,
//         3: 0.2016,
//         6: 0.2413,
//         16: 0.1078,
//         12: 0.1081,
//         18: 0.1248,
//     },
//     12: {
//         1: 0.1313,
//         8: 0.1011,
//         17: 0.0679,
//         2: 0.1132,
//         4: 0.109,
//         3: 0.1154,
//         18: 0.0469,
//         14: 0.035,
//         16: 0.0546,
//         10: 0.1048,
//         19: 0.0607,
//         13: 0.0609,
//         5: 0.1081,
//         6: 0.1056,
//         9: 0.1076,
//         20: 0.0612,
//         7: 0.108,
//         11: 0.0466,
//         15: 0.0389,
//     },
//     15: {
//         1: 0.1079,
//         11: 0.0379,
//         20: 0.0402,
//         10: 0.0755,
//         2: 0.0886,
//         5: 0.08,
//         13: 0.0459,
//         3: 0.0979,
//         18: 0.0442,
//         7: 0.0767,
//         16: 0.0411,
//         8: 0.0877,
//         4: 0.0797,
//         12: 0.0389,
//         9: 0.0772,
//         6: 0.082,
//         19: 0.043,
//         17: 0.0454,
//         14: 0.0258,
//     },
//     8: {
//         3: 0.2388,
//         5: 0.2405,
//         15: 0.0877,
//         17: 0.1292,
//         14: 0.0708,
//         18: 0.1274,
//         20: 0.1212,
//         10: 0.1866,
//         7: 0.2445,
//         12: 0.1011,
//         11: 0.1026,
//         1: 0.247,
//         13: 0.1425,
//         2: 0.2612,
//         19: 0.1214,
//         16: 0.1108,
//         9: 0.1989,
//         4: 0.2493,
//         6: 0.2444,
//     },
//     13: {
//         2: 0.1562,
//         10: 0.1317,
//         9: 0.1366,
//         19: 0.0773,
//         12: 0.0609,
//         11: 0.0587,
//         16: 0.054,
//         14: 0.0402,
//         5: 0.1447,
//         15: 0.0459,
//         3: 0.1462,
//         17: 0.0825,
//         20: 0.0773,
//         18: 0.0794,
//         1: 0.1654,
//         4: 0.1486,
//         7: 0.1262,
//         6: 0.1375,
//         8: 0.1425,
//     },
//     3: {
//         19: 0.1202,
//         13: 0.1462,
//         18: 0.1219,
//         17: 0.1446,
//         12: 0.1154,
//         5: 0.2016,
//         15: 0.0979,
//         4: 0.2627,
//         9: 0.1928,
//         14: 0.0745,
//         6: 0.2266,
//         1: 0.2821,
//         11: 0.1077,
//         8: 0.2388,
//         7: 0.2572,
//         16: 0.1153,
//         10: 0.2083,
//         20: 0.1246,
//         2: 0.2713,
//     },
//     9: {
//         2: 0.2359,
//         14: 0.067,
//         3: 0.1928,
//         12: 0.1076,
//         4: 0.2346,
//         5: 0.2138,
//         7: 0.2402,
//         16: 0.1094,
//         13: 0.1366,
//         1: 0.2545,
//         10: 0.2161,
//         18: 0.1091,
//         17: 0.1238,
//         6: 0.205,
//         8: 0.1989,
//         20: 0.1165,
//         11: 0.0961,
//         19: 0.1209,
//         15: 0.0772,
//     },
//     2: {
//         5: 0.2596,
//         7: 0.2329,
//         12: 0.1132,
//         6: 0.2387,
//         10: 0.2538,
//         15: 0.0886,
//         8: 0.2612,
//         3: 0.2713,
//         13: 0.1562,
//         18: 0.1271,
//         14: 0.0783,
//         20: 0.1259,
//         11: 0.1225,
//         1: 0.2542,
//         16: 0.122,
//         17: 0.1298,
//         9: 0.2359,
//         4: 0.259,
//         19: 0.1333,
//     },
//     7: {
//         2: 0.2329,
//         12: 0.108,
//         14: 0.0658,
//         19: 0.1209,
//         10: 0.2338,
//         5: 0.1959,
//         8: 0.2445,
//         20: 0.1211,
//         15: 0.0767,
//         6: 0.2321,
//         13: 0.1262,
//         11: 0.1024,
//         1: 0.248,
//         16: 0.1039,
//         4: 0.2142,
//         3: 0.2572,
//         9: 0.2402,
//         18: 0.1235,
//         17: 0.1205,
//     },
// }