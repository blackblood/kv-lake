use std::sync::{ Arc, RwLock };
use super::my_node;

pub struct List<T: std::fmt::Display + std::clone::Clone> {
    pub head: Option<Arc<RwLock<my_node::Node<T>>>>,
    pub end: Option<Arc<RwLock<my_node::Node<T>>>>,
    pub size: u32
}

pub struct ListIterator<T: std::fmt::Display + std::clone::Clone> {
    ptr: Option<Arc<RwLock<my_node::Node<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Iterator for ListIterator<T> {
    type Item = Arc<RwLock<my_node::Node<T>>>;

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

impl<T: std::fmt::Display + std::clone::Clone> List<T> {
    pub fn new() -> List<T> {
        List { head: None, end: None, size: 0 }
    }

    pub fn iter(&self) -> ListIterator<T> {
        if let Some(h) = self.head.as_ref() {
            let itr = ListIterator { ptr: Some(Arc::clone(h)) };
            return itr;
        } else {
            return ListIterator { ptr: None }
        }
    }

    fn is_empty(&mut self) -> bool {
        if self.head.is_none() && self.end.is_none() {
            true
        } else {
            false
        }
    }

    pub fn append(&mut self, n: Arc<RwLock<my_node::Node<T>>>) {
        if self.is_empty() {
            self.head = Some(Arc::clone(&n));
            self.end = Some(Arc::clone(&n));
        } else {
            if let Some(e) = self.end.take() {
                let mut n_lock = n.write().unwrap();
                n_lock.prev = Some(Arc::clone(&e));
                let mut e_lock = e.write().unwrap();
                e_lock.next = Some(Arc::clone(&n));
                self.end = Some(Arc::clone(&n));
            } else {
                panic!("List is in illegal state");
            }
        }
        self.size += 1;
    }

    pub fn move_to_front(&mut self, n: Arc<RwLock<my_node::Node<T>>>) {
        if let Some(h) = self.head.take() {
            if let Some(n_prv) = n.read().unwrap().prev.as_deref() {
                n_prv.write().unwrap().set_next(n.read().unwrap().next.as_ref());
            } else {
                self.head = Some(h);
                return ();
            }
            if let Some(n_nxt) = n.read().unwrap().next.as_deref() {
                n_nxt.write().unwrap().set_prev(n.read().unwrap().prev.as_ref());
            } else {
                if let Some(end) = n.read().unwrap().prev.as_ref() {
                    self.end = Some(Arc::clone(end));
                }
            }
            {
                let mut m = n.write().unwrap();
                m.set_next(Some(&h));
            }
            {
                let mut m = n.write().unwrap();
                m.set_prev(None);
            }
            {
                let mut m = h.write().unwrap();
                m.set_prev(Some(&n));
            }
            self.head = Some(n);
        }
    }

    pub fn remove_last_node(&mut self) -> Option<Arc<RwLock<my_node::Node<T>>>> {
        if let Some(e) = self.end.take() {
            self.end = Some(Arc::clone(e.read().unwrap().prev.as_ref().unwrap()));
            self.size -= 1;
            Some(Arc::clone(&e))
        } else {
            None
        }
    }
}
