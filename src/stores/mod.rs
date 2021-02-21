pub mod lru;
pub mod lfu;

pub trait Cacheable<T: std::fmt::Display + std::clone::Clone> {
    fn put(&mut self, key: String, value: T);
    fn get(&mut self, key: String) -> Option<T>;
    fn delete(&mut self, key: String) -> Result<(), String>;
    fn print_map(&self);
}
