use std::collections::HashMap;

use ndarray::{Dim, IxDynImpl};

use crate::model::analysis::metrics::metric::Metric;

pub struct PredictionMatrix{
    value: HashMap<Dim<IxDynImpl>, f64>,
}

impl Metric<HashMap<Dim<IxDynImpl>, f64>> for PredictionMatrix{
    fn get(&self) -> &HashMap<Dim<IxDynImpl>, f64>{
        return &self.value;
    }
}

impl PredictionMatrix{
    pub fn new(value: HashMap<Dim<IxDynImpl>, f64>) -> PredictionMatrix{
        return PredictionMatrix{
            value: value,
        };
    }

    pub fn insert(&mut self, index: Dim<IxDynImpl>, value: f64){
        self.value.insert(index, value);
    }

    pub fn getValue(&self, value: &Dim<IxDynImpl>) -> Option<&f64>{
        return self.value.get(value);
    }

    pub fn getMutValue(&mut self, value: &Dim<IxDynImpl>) -> Option<&mut f64>{
        return self.value.get_mut(value);
    }
}