use std::sync::{ Arc, RwLock };
use super::my_list;

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

    pub fn join_neighbours(&mut self, list: &mut my_list::List<T>) {
        if let Some(n_prv) = self.prev.as_deref() {
            let mut mut_n_prv = n_prv.write().unwrap();
            if let Some(p) = self.next.as_ref() {
                mut_n_prv.next = Some(Arc::clone(p));
            } else {
                mut_n_prv.next = None;
                list.end = Some(Arc::clone(self.prev.as_ref().unwrap()));
            }
        }
        if let Some(n_nxt) = self.next.as_deref() {
            let mut mut_n_nxt = n_nxt.write().unwrap();
            if let Some(p) = self.prev.as_ref() {
                mut_n_nxt.prev = Some(Arc::clone(p));
            } else {
                mut_n_nxt.prev = None;
                list.head = Some(Arc::clone(self.next.as_ref().unwrap()));
            }
        }
        if self.prev.is_none() && self.next.is_none() {
            list.head = None;
            list.end = None;
        }
        list.size -= 1;
    }
}

impl<T: std::fmt::Display + std::clone::Clone> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node: {}", self.value);
    }
}
