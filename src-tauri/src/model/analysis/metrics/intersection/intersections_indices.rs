use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct IntersectionsIndices{
    value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>,
}

impl Metric<HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>> for IntersectionsIndices{
    fn get(&self) -> &HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>
    {
        return &self.value;
    }
}

impl IntersectionsIndices{
    pub fn new(value: HashMap<u32, HashMap<u32, Vec<Dim<IxDynImpl>>>>) -> IntersectionsIndices{
        return IntersectionsIndices{
            value: value,
        };
    }

    pub fn getValue(&self, value: &u32) -> Option<&HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &u32) -> Option<&mut HashMap<u32, Vec<Dim<IxDynImpl>>>>{
        return self.value.get_mut(value);
    }
}