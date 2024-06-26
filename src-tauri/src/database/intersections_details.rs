use serde::{ser::SerializeStruct, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct IntersectionsDetails{
    pub identifier: u32,
    pub total_untouched_percentage: f64,
    pub total_intersection_percentage: f64,

    pub intersections: HashMap<u32, (f64, Vec<Vec<String>>)>, // Identifier, (percentage, raw_dims)
}

impl IntersectionsDetails{
    pub fn new(identifier: u32, total_untouched_percentage: f64, total_intersection_percentage: f64, 
                intersections: HashMap<u32, (f64, Vec<Vec<String>>)>) -> IntersectionsDetails{
        
        return IntersectionsDetails{
            identifier: identifier,
            total_untouched_percentage: total_untouched_percentage,
            total_intersection_percentage: total_intersection_percentage,
            intersections: intersections,
        };
    }
}

impl Serialize for IntersectionsDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("IntersectionsDetails", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("total_untouched_percentage", &self.total_untouched_percentage)?;
        state.serialize_field("total_intersection_percentage", &self.total_intersection_percentage)?;
        state.serialize_field("intersections", &self.intersections)?;
        state.end()
    }
}