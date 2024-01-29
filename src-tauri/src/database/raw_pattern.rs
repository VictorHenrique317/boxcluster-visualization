use serde::{ser::SerializeStruct, Serialize};

#[derive(Clone, Debug)]
pub struct RawPattern {
    pub identifier: u32,
    pub dims_values: Vec<Vec<String>>,
    pub density: f64,
    pub size: u32,
}

impl RawPattern {
    pub fn new<'a>(identifier: &u32, dims_values: &Vec<Vec<String>>, density: &f64, size: &u32) -> RawPattern {

        return RawPattern {
            identifier: *identifier,
            dims_values: dims_values.clone(),
            density: *density,
            size: *size,
        };
    }
}

impl Serialize for RawPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("RawPattern", 4)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("dims_values", &self.dims_values)?;
        state.serialize_field("density", &self.density)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}