use std::sync::{ Arc, RwLock };
use super::list::List;

pub struct FrequencyNode<T: std::fmt::Display + std::clone::Clone> {
    pub frequency: u32,
    pub list: List<T>,
    pub next: Option<Arc<RwLock<FrequencyNode<T>>>>,
    pub prev: Option<Arc<RwLock<FrequencyNode<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> FrequencyNode<T> {
    pub fn new(frequency: u32) -> FrequencyNode<T> {
        FrequencyNode {
            frequency,
            list: List::new(),
            next: None,
            prev: None
        }
    }
}
