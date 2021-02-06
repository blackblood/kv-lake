use std::sync::{ Arc, RwLock };
use super::frequency_node::FrequencyNode;

pub struct Node<T: std::fmt::Display + std::clone::Clone> {
    pub key: String,
    pub value: T,
    pub prev: Option<Arc<RwLock<Node<T>>>>,
    pub next: Option<Arc<RwLock<Node<T>>>>,
    pub freq_node: Arc<RwLock<FrequencyNode<T>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Node<T> {
    pub fn new(k: String, v: T, freq_node: &Arc<RwLock<FrequencyNode<T>>>) -> Node<T> {
        Node { key: k, value: v, prev: None, next: None, freq_node: Arc::clone(freq_node) }
    }

    pub fn join_neighbours(&mut self) {
        if let Some(n_prv) = self.prev.as_deref() {
            let mut mut_n_prv = n_prv.write().unwrap();
            if let Some(p) = self.next.as_ref() {
                mut_n_prv.next = Some(Arc::clone(p));
            } else {
                mut_n_prv.next = None;
                let mut freq_n = self.freq_node.write().unwrap();
                freq_n.list.end = Some(Arc::clone(self.prev.as_ref().unwrap()));
            }
        }
        if let Some(n_nxt) = self.next.as_deref() {
            let mut mut_n_nxt = n_nxt.write().unwrap();
            if let Some(p) = self.prev.as_ref() {
                mut_n_nxt.prev = Some(Arc::clone(p));
            } else {
                mut_n_nxt.prev = None;
                let mut freq_n = self.freq_node.write().unwrap();
                freq_n.list.head = Some(Arc::clone(self.next.as_ref().unwrap()));
            }
        }
        if self.prev.is_none() && self.next.is_none() {
            let mut freq_n = self.freq_node.write().unwrap();
            freq_n.list.head = None;
            freq_n.list.end = None;
        }
    }
}

impl<T: std::fmt::Display + std::clone::Clone> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node: {}", self.value);
    }
}
