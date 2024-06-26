#![allow(non_snake_case)]

mod rss_evolution {
    use boxcluster_visualization::services::application::application_service::ApplicationService;

    #[test]
    fn rss20Values(){
        let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");

        let mut raw_rss_s: Vec<f64> = model_manager.getFullRssEvolution().expect("Test should not fail").clone().iter()
                .map(|rss| rss.clone())
                .collect();

        raw_rss_s.remove(0);

        let mut actual: Vec<f64> = Vec::new();

        for raw_rss in raw_rss_s{
            let rounded_rss = raw_rss.floor();
            actual.push(rounded_rss);
        }

        let expected: Vec<f64> = vec![55563.0, 55548.0, 55534.0, 55519.0, 55505.0, 
        55492.0, 55479.0, 55467.0, 55454.0, 55443.0, 55432.0, 55423.0, 55413.0, 55403.0, 
        55394.0, 55384.0, 55375.0, 55366.0, 55357.0, 55347.0];

        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate(){
            assert_eq!(expected, actual, "Failed at index {}", i);
        }
    }

    #[test]
    fn rssRetweets3d(){
        let tensor_path = "tests/test_data/tensors/retweets3d.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/retweets3d_patterns.txt".to_owned();

        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");

        let raw_rss_s: Vec<f64> = model_manager.getFullRssEvolution().expect("Test should not fail").clone().iter()
                .map(|rss| rss.clone())
                .collect();

        let mut actual: Vec<f64> = Vec::new();

        for raw_rss in raw_rss_s{
            let rounded_rss = raw_rss.floor();
            actual.push(rounded_rss);
        }

        let expected: Vec<f64> = vec![
            39415.79927452727,
            37275.81230600515,
            36367.77197477747,
            35552.37415697502,
            34839.796496148076,
            34178.79762762713,
            33601.67738319933,
            33063.247359212066,
            32901.62950798889,
            32755.02481075411,
            32651.210887508034,
            32550.641236532894,
            32470.811359898205,
            32433.37086951121,
            32401.33500689821,
            32377.38812521592,
            32356.434603743914,
            32338.474442482195,
            32320.892005621412,
            32303.92963109645,
            32290.94914411113,
            32279.97349000675,
            32273.85760249662,
        ];

        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate(){
            assert_eq!(&expected.floor(), actual, "Failed at index {}", i);
        }
    }

    #[test]
    fn rssRetweets2d(){
        let tensor_path = "tests/test_data/tensors/retweets2d.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/retweets2d_patterns.txt".to_owned();

        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path).expect("Test should not fail");

        let raw_rss_s: Vec<f64> = model_manager.getFullRssEvolution().expect("Test should not fail").clone().iter()
                .map(|rss| rss.clone())
                .collect();

        let mut actual: Vec<f64> = Vec::new();

        for raw_rss in raw_rss_s{
            let rounded_rss = raw_rss.floor();
            actual.push(rounded_rss);
        }

        let expected: Vec<f64> = vec![
            3912.143858984434,
            3545.1035100228373,
            3340.8235042141255,
            3151.558870759262,
            2964.3160995763415,
            2809.7095969131565,
            2664.7349061057785,
            2532.1601962607906,
            2426.8181667673275,
            2342.5661142617405,
            2298.0227991037286,
            2285.048142554222,
        ];

        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate(){
            assert_eq!(&expected.floor(), actual, "Failed at index {}", i);
        }
    }
}