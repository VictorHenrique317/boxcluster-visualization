use crate::{model::{identifier_mapper::IdentifierMapper, analysis::metrics::{coordinates::Coordinates, metric::Metric}}, database::{datapoint::DataPoint, pattern::Pattern}, common::generic_error::GenericError};

pub struct DataPointService {}

impl DataPointService {
    fn normalizeSize(size: &u32, dimension: &u32) -> f32 {
        // let size_multiplier = 1.0;
        // let normalized_size = size_multiplier * (*size as f32 / *min_size as f32).ln();

        // if normalized_size == 0.0 {
        //     return size_multiplier;
        // }
        // return normalized_size;
        return (*size as f32).powf(1.0 / *dimension as f32);
        
    }

    fn densityToColor(density: &f64) -> (u32, u32, u32) {
        let r = (density * 255.0) as u32;
        let g = (255 - r) as u32;
        let b = 0 as u32;

        return (r, g, b);
    }

    pub fn createDataPoints(identifier_mapper: &IdentifierMapper, coordinates: &Coordinates) -> Result<Vec<DataPoint>, GenericError> {
        let coordinates = coordinates.get();
        let mut pattern_representations: Vec<&Pattern> = Vec::new();
        for r in identifier_mapper.getRepresentations().iter() {
            match r.asPattern() {
                Ok(pattern) => pattern_representations.push(pattern),
                Err(e) => return Err(e),
            }
        }

        let mut datapoints: Vec<DataPoint> = Vec::new();
        let dimension = pattern_representations.get(0)
            .ok_or(GenericError::new("Could not get dimension"))?
            .dims_values.len() as u32;

        for pattern in pattern_representations {
            let coord = coordinates.get(&pattern.identifier)
                .ok_or(GenericError::new("Could not get coordinates"))?;
            
            let size = DataPointService::normalizeSize(&pattern.size, &dimension);
            // let stroke_width = DataPointService::calculateStrokeWidth(&max_size, &size);
            let stroke_width = 2;
            let color = DataPointService::densityToColor(&pattern.density);
            
            let datapoint = DataPoint::new(
                &pattern.identifier,
                &size,
                &stroke_width,
                &(coord.0 as f32),
                &(coord.1 as f32),
                &color.0,
                &color.1,
                &color.2,
                );
            datapoints.push(datapoint);
        }

        return Ok(datapoints);
    }
}