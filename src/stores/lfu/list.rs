use std::sync::{ Arc, RwLock };
use super::my_node::Node;

pub struct List<T: std::fmt::Display + std::clone::Clone> {
    pub head: Option<Arc<RwLock<Node<T>>>>,
    pub end: Option<Arc<RwLock<Node<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> List<T> {
    pub fn new() -> List<T> {
        List { head: None, end: None }
    }

    pub fn prepend(&mut self, n: Arc<RwLock<Node<T>>>) {
        let mut m_n = n.write().unwrap();
        m_n.prev = None;
        m_n.next = None;
        if let Some(h) = self.head.take() {
            m_n.next = Some(Arc::clone(&h));
            let mut h2 = h.write().unwrap();
            h2.prev = Some(Arc::clone(&n));
            self.head = Some(Arc::clone(&n));
        } else {
            self.head = Some(Arc::clone(&n));
        }
        if self.end.is_none() {
            self.end = Some(Arc::clone(self.head.as_ref().unwrap()));
        }
    }

    pub fn iter(&mut self) -> ListIterator<T> {
        if let Some(h) = self.head.take() {
            let itr = ListIterator { ptr: Some(Arc::clone(&h)) };
            self.head = Some(h);
            return itr;
        } else {
            return ListIterator { ptr: None }
        }
    }
}

pub struct ListIterator<T: std::fmt::Display + std::clone::Clone> {
    ptr: Option<Arc<RwLock<Node<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Iterator for ListIterator<T> {
    type Item = Arc<RwLock<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr_ptr) = self.ptr.take() {
            if let Some(right_ptr) = curr_ptr.read().unwrap().next.as_ref() {
                self.ptr = Some(Arc::clone(right_ptr));
            }
            Some(curr_ptr)
        } else {
            None
        }
    }
}
