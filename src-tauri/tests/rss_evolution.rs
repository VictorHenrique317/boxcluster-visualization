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
            39432.31305445463,
            37308.02018238809,
            36401.93216846844,
            35586.53118722249,
            34873.951634953664,
            34214.69790811278,
            33648.704744556635,
            33110.27318648956,
            32948.65485578645,
            32802.04973211843,
            32699.30132007819,
            32598.731223012004,
            32518.901217912535,
            32481.460646074098,
            32449.424730185154,
            32425.477810893863,
            32404.52425651398,
            32386.56406704551,
            32368.98159415492,
            32352.01919299025,
            32339.03868563339,
            32328.063014291547,
            32321.947112684986,
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
            3931.5276374855307,
            3563.8851838689857,
            3357.9863863837854,
            3169.792482333475,
            2982.55193358002,
            2825.542938965483,
            2680.5693069741756,
            2547.9717449606687,
            2442.630377523928,
            2358.3795285786514,
            2313.8365373185993,
            2300.5783719300357,
        ];

        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate(){
            assert_eq!(&expected.floor(), actual, "Failed at index {}", i);
        }
    }
}