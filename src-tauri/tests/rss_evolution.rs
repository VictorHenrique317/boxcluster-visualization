#![allow(non_snake_case)]

mod rss_evolution {
    use boxcluster_visualization::services::application::application_service::ApplicationService;

    #[test]
    fn rss20Values(){
        let tensor_path = "tests/test_data/rss_evolution_test/synth_co1.txt".to_owned();
        let patterns_path = "tests/test_data/rss_evolution_test/synth_co1_truncated_20_patterns.txt".to_owned();

        let mut model_manager = ApplicationService::default();
        model_manager.init(&tensor_path, &patterns_path);

        let mut raw_rss_s: Vec<f64> = model_manager.getFullRssEvolution().clone().iter()
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
        model_manager.init(&tensor_path, &patterns_path);

        let mut raw_rss_s: Vec<f64> = model_manager.getFullRssEvolution().clone().iter()
                .map(|rss| rss.clone())
                .collect();

        let mut actual: Vec<f64> = Vec::new();

        for raw_rss in raw_rss_s{
            let rounded_rss = raw_rss.floor();
            actual.push(rounded_rss);
        }

        let expected: Vec<f64> = vec![
            39415.79927452727,
            37281.86840568112,
            36373.82807445479,
            35558.4302566538,
            34845.85259582544,
            34184.853727295835,
            33612.34282714797,
            33073.9128031597,
            32912.294951937045,
            32765.690254702346,
            32661.876331456177,
            32561.30668048013,
            32481.47680384525,
            32444.036313458233,
            32412.0004508452,
            32388.05356916288,
            32367.10004769085,
            32349.139886429108,
            32331.557449568274,
            32314.595075043297,
            32301.614588057964,
            32290.638933953567,
            32284.523046443413,
        ];

        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate(){
            assert_eq!(&expected.floor(), actual, "Failed at index {}", i);
        }
    }
}