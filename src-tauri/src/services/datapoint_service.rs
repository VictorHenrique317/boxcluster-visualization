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

    fn densityToColor(density: &f64) -> (u32, u32, u32, f32) {
        let r = 255 as u32;
        let g = 0;
        let b = 0 as u32;
        // let a = (255 as f64 * density) as u32;
        let a = *density as f32;

        return (r, g, b, a);
    }

    pub fn createDataPoints(identifier_mapper: &IdentifierMapper, coordinates: &Coordinates) -> Result<Vec<DataPoint>, GenericError> {
        let coordinates = coordinates.get();

        let mut pattern_representations: Vec<&Pattern> = Vec::new();
        for (identifier, _) in coordinates{
            let pattern = identifier_mapper.getRepresentation(identifier)?.asPattern()?;
            pattern_representations.push(pattern);
        }

        let mut datapoints: Vec<DataPoint> = Vec::new();
        let dimension = pattern_representations.get(0)
            .ok_or(GenericError::new("Could not get dimension", file!(), &line!()))?
            .dims_values.len() as u32;

        for pattern in pattern_representations {
            let coord = coordinates.get(&pattern.identifier)
                .ok_or(GenericError::new(format!("Could not get coordinate: {}", &pattern.identifier).as_str(), file!(), &line!()))?;
            
            let size = DataPointService::normalizeSize(&pattern.size, &dimension);
            let density = pattern.density as f32;
            // let stroke_width = DataPointService::calculateStrokeWidth(&max_size, &size);
            let stroke_width = 2;
            let color = DataPointService::densityToColor(&pattern.density);
            
            let x = coord.0 as f32;
            // let x = f32::round(100.0 * x) / 100.0;
            
            let y = coord.1 as f32;
            // let y = f32::round(100.0 * y) / 100.0;
            
            let datapoint = DataPoint::new(
                &pattern.identifier,
                &size,
                &pattern.size,
                &density,
                &stroke_width,
                &x,
                &y,
                &color.0,
                &color.1,
                &color.2,
                &color.3
                );

            datapoints.push(datapoint);
        }

        return Ok(datapoints);
    }
}