#![allow(non_snake_case)]

mod rss_evolution {
    use boxcluster_visualization::services::application::application_service::ApplicationService;
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn synthTruncated20(){
        let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();
        
        let mut application_manager = ApplicationService::new(&tensor_path, &patterns_path);
        application_manager.init();
        let actual: Vec<u32> = application_manager.getRssEvolution().clone().iter()
                .map(|(identifier, _)| identifier.clone())
                .collect();

        let expected: Vec<u32> = (0..=20).collect();
        assert_eq!(expected, actual);    
    }

    // #[test]
    // fn synthTruncated100(){
    //     let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    //     let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_100_patterns.txt".to_owned();
        
    //     let mut application_manager = ApplicationService::new(&tensor_path, &patterns_path);
    //     application_manager.init();
    //     let actual: Vec<u32> = application_manager.getRssEvolution().clone().iter()
    //             .map(|(identifier, _)| identifier.clone())
    //             .collect();

    //     let expected: Vec<u32> = (0..=100).collect();
    //     assert_eq!(expected, actual);    
    // }
}