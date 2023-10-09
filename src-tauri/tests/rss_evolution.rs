#![allow(non_snake_case)]

mod rss_evolution {
    use boxcluster_visualization::services::application::application_service::ApplicationService;

    #[test]
    fn rss20Values(){
        let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

        let application_manager = ApplicationService::new(&tensor_path, &patterns_path);
        let mut raw_rss_s: Vec<f64> = application_manager.getRssEvolution().clone().iter()
                .map(|(_, rss)| rss.clone())
                .collect();

        raw_rss_s.remove(0);

        let mut actual: Vec<f64> = Vec::new();

        for raw_rss in raw_rss_s{
            let rounded_rss = raw_rss.floor();
            actual.push(rounded_rss);
        }

        let expected: Vec<f64> = vec![55563.0, 55548.0, 55534.0, 55519.0, 55505.0, 
        55492.0, 55479.0, 55466.0, 55454.0, 55442.0, 55432.0, 55422.0, 55412.0, 55402.0, 
        55392.0, 55382.0, 55373.0, 55364.0, 55354.0, 55345.0];

        assert_eq!(expected, actual);
    }

    // #[test]
    // fn rss648Values(){
    //     let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
    //     let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_patterns.txt".to_owned();

    //     let application_manager = ApplicationService::new(&tensor_path, &patterns_path);
    //     let raw_rss_s: Vec<f64> = application_manager.getRssEvolution().clone().iter()
    //             .map(|(_, rss)| rss.clone())
    //             .collect();

    //     let mut actual: Vec<f64> = Vec::new();

    //     for raw_rss in raw_rss_s{
    //         let rounded_rss = raw_rss.floor();
    //         actual.push(rounded_rss);
    //     }

    //     let actual = &actual[1..21];

    //     let expected: Vec<f64> = vec![55563.0, 55548.0, 55534.0, 55519.0, 55505.0, 
    //     55492.0, 55479.0, 55466.0, 55454.0, 55442.0, 55432.0, 55422.0, 55412.0, 55402.0, 
    //     55392.0, 55382.0, 55373.0, 55364.0, 55354.0, 55345.0];

    //     assert_eq!(expected, actual);
    // }
}