use plotters::style::RGBColor;

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub identifier: u32,
    pub size: f32,
    pub stroke_width: u32,
    pub color: RGBColor,
    pub x: f32,
    pub y: f32,
}

impl DataPoint {
    pub fn new(identifier: &u32, size: &f32, stroke_width: &u32, color: &RGBColor, x: &f32, y: &f32) -> DataPoint { 
        return DataPoint { identifier: *identifier, x: *x, y: *y , size: *size, stroke_width:*stroke_width, color: *color};
    }

    pub fn toTuple() -> (f32, f32) {
        todo!()
    }
}