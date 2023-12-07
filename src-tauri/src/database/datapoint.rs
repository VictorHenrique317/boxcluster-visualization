use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub identifier: u32,
    pub size: f32,
    pub stroke_width: u32,

    pub x: f32,
    pub y: f32,

    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl Serialize for DataPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {
        let mut state = serializer.serialize_struct("DataPoint", 8)?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("stroke_width", &self.stroke_width)?;
        
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;

        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;
        state.end()
    }
}

impl DataPoint {
    pub fn new(identifier: &u32, size: &f32, stroke_width: &u32, x: &f32, y: &f32, r: &u32, g: &u32, b: &u32, a: &u32) -> DataPoint { 
        return DataPoint { identifier: *identifier, 
            x: *x, 
            y: *y , 
            size: *size, 
            stroke_width:*stroke_width, 
            r: *r,  
            g: *g,
            b: *b,
            a: *a,
            };
    }
}