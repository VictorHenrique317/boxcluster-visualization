use plotters::style::RGBColor;

use crate::{model::{identifier_mapper::IdentifierMapper, analysis::metrics::{coordinates::Coordinates, metric::Metric}}, database::{datapoint::DataPoint, pattern::Pattern}};

pub struct DataPointService {}

impl DataPointService {
    fn normalizeSize(size: &u32, min_size: &u32) -> f32 {
        return 2.0 * (*size as f32 / *min_size as f32).ln();
    }

    fn calculateStrokeWidth(normalized_max_size: &f32, normalized_size: &f32) -> u32 {
        // let x = normalized_size / normalized_max_size;
        // let interval = 4.0;
        // let mut stroke_width = interval * (x.powi(2));

        
        // if stroke_width <= interval * 0.25 {
        //     stroke_width = interval * 0.5;
        // }

        // dbg!(&(stroke_width as u32));
        // return stroke_width as u32;
        return 2;
    }

    fn densityToColor(density: &f64) -> RGBColor {
        let r = (density * 255.0) as u8;
        let g = 255 - r;
        let b = 0;
        return RGBColor(r, g, b);
    }

    pub fn createDataPoints(identifier_mapper: &IdentifierMapper, coordinates: &Coordinates) -> Vec<DataPoint> {
        let coordinates = coordinates.get();
        let pattern_representations: Vec<&Pattern> = identifier_mapper.getRepresentations().iter()
            .map(|r| r.asPattern())
            .collect();

        let mut descending_sizes: Vec<&Pattern> = pattern_representations.iter().cloned().collect();
        descending_sizes.sort_by_key(|p| p.identifier);
        let min_size = descending_sizes.get(descending_sizes.len() - 1).unwrap().size;

        let max_size = descending_sizes.get(0).unwrap().size;
        let max_size = DataPointService::normalizeSize(&max_size, &min_size);


        let mut datapoints: Vec<DataPoint> = Vec::new();
        for pattern in pattern_representations {
            let coord = coordinates.get(&pattern.identifier).unwrap();
            let size = DataPointService::normalizeSize(&pattern.size, &min_size);
            let stroke_width = DataPointService::calculateStrokeWidth(&max_size, &size);
            
            let datapoint = DataPoint::new(
                &pattern.identifier,
                &size,
                &stroke_width,
                &DataPointService::densityToColor(&pattern.density),
                &(coord.0 as f32),
                &(coord.1 as f32)
                );
                datapoints.push(datapoint);
        }

        return datapoints;
    }
}