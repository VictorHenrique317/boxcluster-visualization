use std::collections::HashMap;

use crate::model::analysis::metrics::metric::Metric;

pub struct UntouchedDeltaRss{
    value: HashMap<u32, (u32, f64)>,
}

impl Metric<HashMap<u32, (u32, f64)>> for UntouchedDeltaRss{
    fn get(&self) -> &HashMap<u32, (u32, f64)>{
        return &self.value;
    }
}

impl UntouchedDeltaRss{
    pub fn new(value: HashMap<u32, (u32, f64)>) -> UntouchedDeltaRss{
        return UntouchedDeltaRss{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&(u32, f64)>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut (u32, f64)>{
        return self.value.get_mut(value);
    }
}