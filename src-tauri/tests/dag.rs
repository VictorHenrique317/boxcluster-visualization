#![allow(non_snake_case)]
mod dag {
    use boxcluster_visualization::services::application::application_service::ApplicationService;
    use itertools::Itertools;
    use std::collections::HashMap;

    fn sortHashMap(hashmap: &HashMap<u32, Vec<u32>>) -> HashMap<u32, Vec<u32>> {
        let mut sorted_hashmap: HashMap<u32, Vec<u32>> = HashMap::new();

        for key in hashmap.keys().sorted() {
            let sorted_value: Vec<u32> = hashmap
                .get(key)
                .unwrap()
                .into_iter()
                .sorted()
                .map(|i| i.clone())
                .collect();
            sorted_hashmap.insert(*key, sorted_value);
        }

        return sorted_hashmap;
    }

    // fn getPatterns(path: String) -> Vec<Pattern>{
    //     let pattern_reader = PatternReader::new(&path);
    //     let patterns = pattern_reader.getPatternsCopy();
    //     return patterns;
    // }

    #[test]
    fn testSimpleOverlap() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-overlap.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-overlap.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleOverlap2() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-overlap-2.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-overlap-2.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testDoubleDiffOverlap() {
        let patterns_path = "tests/test_data/dag_test_patterns/double-diff-overlap.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/double-diff-overlap.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testTripleDiffOverlap() {
        let patterns_path = "tests/test_data/dag_test_patterns/triple-diff-overlap.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/triple-diff-overlap.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3, 4]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![1]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testQuadrupleDiffOverlap() {
        let patterns_path = "tests/test_data/dag_test_patterns/quadruple-diff-overlap.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/quadruple-diff-overlap.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3, 4, 5]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);
        expected_subs.insert(5, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![1]);
        expected_supers.insert(5, vec![1]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSub() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-msub.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-msub.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSub2() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-msub-2.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-msub-2.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![4]);
        expected_subs.insert(4, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![3]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testComplexMSub() {
        let patterns_path = "tests/test_data/dag_test_patterns/complex-msub.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/complex-msub.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 5, 6]);
        expected_subs.insert(2, vec![3, 4]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);
        expected_subs.insert(5, vec![]);
        expected_subs.insert(6, vec![7]);
        expected_subs.insert(7, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![2]);
        expected_supers.insert(5, vec![1]);
        expected_supers.insert(6, vec![1]);
        expected_supers.insert(7, vec![6]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSuper() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-msuper.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-msuper.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2,4]);
        expected_subs.insert(2, vec![3,5]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![5]);
        expected_subs.insert(5, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![1]);
        expected_supers.insert(5, vec![2, 4]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMRoot() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-mroot.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-mroot.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2,5]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![5]);
        expected_subs.insert(5, vec![6]);
        expected_subs.insert(6, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![]);
        expected_supers.insert(5, vec![1,4]);
        expected_supers.insert(6, vec![5]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleLine() {
        let patterns_path = "tests/test_data/dag_test_patterns/simple-line.txt".to_owned();
        let tensor_path = "tests/test_data/dag_test_patterns/simple-line.txt".to_owned();
        
        let model_manager = ApplicationService::new(&tensor_path, &patterns_path);

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![4]);
        expected_subs.insert(4, vec![5]);
        expected_subs.insert(5, vec![6]);
        expected_subs.insert(6, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![3]);
        expected_supers.insert(5, vec![4]);
        expected_supers.insert(6, vec![5]);

        let r_subs = sortHashMap(&model_manager.getFlattenedSubs());
        let r_supers = sortHashMap(&model_manager.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }
}