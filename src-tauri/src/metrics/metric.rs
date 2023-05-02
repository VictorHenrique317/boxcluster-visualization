pub trait Metric<T>{
    fn get(&self) -> &T;
}