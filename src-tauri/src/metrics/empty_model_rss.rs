use crate::tensor::tensor::Tensor;
use super::metric::Metric;

#[derive(Default)]
pub struct EmptyModelRss{
    value: f64, 
}

#[allow(non_camel_case_types)]
impl Metric<f64> for EmptyModelRss{
    fn get(&self) -> &f64 {
        return &self.value;
    }
}

impl EmptyModelRss{
    pub fn new(tensor: &Tensor) -> EmptyModelRss {
        println!("  Empty model RSS...");
        return EmptyModelRss { value: EmptyModelRss::calculate(tensor) }
    }

    fn calculate(tensor: &Tensor) -> f64{
        let mut rss = 0.0;
        for actual_value in tensor.dims_values.iter(){
            rss += (actual_value - tensor.density).powi(2);
        }
        return rss;
    }
}
    
    