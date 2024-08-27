#![allow(non_snake_case)]
mod distances {

    use std::collections::HashMap;

    use boxcluster_visualization::services::application::application_service::ApplicationService;

    // #[test]
    // fn retweets158(){
    //     let tensor_path = "tests/test_data/tensors/retweets-sparser.txt".to_owned();
    //     let patterns_path = "tests/test_data/distance_test/158-retweets-sparser.txt".to_owned();
        
    //     let mut application_manager = ApplicationService::new(&tensor_path, &patterns_path);
    //     application_manager.init();
    //     let mut actual: HashMap<String, HashMap<String, String>> = HashMap::new();
    //     for (x, x_distances) in application_manager.getDistances(){
    //         let mut string_x_distances: HashMap<String, String> = HashMap::new();

    //         for (y, distance) in x_distances {
    //             string_x_distances.insert(y.to_string(), distance.to_string());
    //         }

    //         actual.insert(x.to_string(), string_x_distances);
    //     }

    //     let file = File::open("tests/test_data/distance_test/retweets-158-distances-strings.json").unwrap();
    //     let expected: HashMap<String, HashMap<String, String>> = serde_json::from_reader(file).unwrap();
        
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn manualA(){
        let tensor_path = "tests/test_data/distance_test/a.txt".to_owned();
        let patterns_path = "tests/test_data/distance_test/a_patterns.txt".to_owned();
        
        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");
        let actual = model_manager.getDistances().expect("Test should not fail").clone();

        let mut expected: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        expected.insert(1, HashMap::from_iter(vec![(2, 0.0039)].into_iter()));
        expected.insert(2, HashMap::from_iter(vec![(1, 0.0039)].into_iter()));

        assert_eq!(expected, actual);    
    }

    #[test]
    fn manualB(){
        let tensor_path = "tests/test_data/distance_test/b.txt".to_owned();
        let patterns_path = "tests/test_data/distance_test/b_patterns.txt".to_owned();
        
        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");
        let actual = model_manager.getDistances().expect("Test should not fail").clone();

        let mut expected: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        expected.insert(1, HashMap::from_iter(vec![(2, 0.7665)].into_iter()));
        expected.insert(2, HashMap::from_iter(vec![(1, 0.7665)].into_iter()));

        assert_eq!(expected, actual);    
    }

    #[test]
    fn manualC(){
        let tensor_path = "tests/test_data/distance_test/c.txt".to_owned();
        let patterns_path = "tests/test_data/distance_test/c_patterns.txt".to_owned();
        
        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");
        let actual = model_manager.getDistances().expect("Test should not fail").clone();

        let mut expected: HashMap<u32, HashMap<u32, f64>> = HashMap::new();
        expected.insert(1, HashMap::from_iter(vec![(2, 0.0474)].into_iter()));
        expected.insert(2, HashMap::from_iter(vec![(1, 0.0474)].into_iter()));

        assert_eq!(expected, actual);    
    }

   
}