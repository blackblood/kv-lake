use std::sync::{ Arc, RwLock };

pub struct Node<T: std::fmt::Display + std::clone::Clone> {
    pub key: String,
    pub value: T,
    pub prev: Option<Arc<RwLock<Node<T>>>>,
    pub next: Option<Arc<RwLock<Node<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Node<T> {
    pub fn new(k: String, v: T) -> Node<T> {
        Node { key: k, value: v, prev: None, next: None }
    }
    pub fn set_next(&mut self, n: Option<&Arc<RwLock<Node<T>>>>) {
        if let Some(u_n) = n {
            self.next = Some(Arc::clone(u_n));
        } else {
            self.next = None;
        }
    }

    pub fn set_prev(&mut self, n: Option<&Arc<RwLock<Node<T>>>>) {
        if let Some(n_n) = n {
            self.prev = Some(Arc::clone(n_n));
        } else {
            self.prev = None;
        }
    }
}

impl<T: std::fmt::Display + std::clone::Clone> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node: {}", self.value);
    }
}
