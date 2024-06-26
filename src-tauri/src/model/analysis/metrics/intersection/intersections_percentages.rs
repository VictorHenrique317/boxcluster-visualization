use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsPercentages{
    value: HashMap<u32, HashMap<u32, f64>>,
}

impl Metric<HashMap<u32, HashMap<u32, f64>>> for IntersectionsPercentages{
    fn get(&self) -> &HashMap<u32, HashMap<u32, f64>>{
        return &self.value;
    }
}

impl IntersectionsPercentages {
    pub fn new(value: HashMap<u32, HashMap<u32, f64>>) -> IntersectionsPercentages{
        return IntersectionsPercentages{
            value: value,
        };
    }
}