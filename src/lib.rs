use std::collections::HashMap;
use std::sync::{ Arc, RwLock };

pub struct Node<T: std::fmt::Display + std::clone::Clone> {
    key: String,
    value: T,
    prev: Option<Arc<RwLock<Node<T>>>>,
    next: Option<Arc<RwLock<Node<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Node<T> {
    pub fn new(k: String, v: T) -> Node<T> {
        Node { key: k, value: v, prev: None, next: None }
    }
    fn set_next(&mut self, n: Option<&Arc<RwLock<Node<T>>>>) {
        if let Some(u_n) = n {
            self.next = Some(Arc::clone(u_n));
        } else {
            self.next = None;
        }
    }

    fn set_prev(&mut self, n: Option<&Arc<RwLock<Node<T>>>>) {
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

struct List<T: std::fmt::Display + std::clone::Clone> {
    head: Option<Arc<RwLock<Node<T>>>>,
    end: Option<Arc<RwLock<Node<T>>>>,
    size: u8
}

struct ListIterator<T: std::fmt::Display + std::clone::Clone> {
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

impl<T: std::fmt::Display + std::clone::Clone> List<T> {
    fn new() -> List<T> {
        List { head: None, end: None, size: 0 }
    }

    fn iter(&mut self) -> ListIterator<T> {
        if let Some(h) = self.head.take() {
            let itr = ListIterator { ptr: Some(Arc::clone(&h)) };
            self.head = Some(h);
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

    fn append(&mut self, n: Arc<RwLock<Node<T>>>) {
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

    fn move_to_front(&mut self, n: Arc<RwLock<Node<T>>>) {
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

    fn remove_last_node(&mut self) -> Option<Arc<RwLock<Node<T>>>> {
        if let Some(e) = self.end.take() {
            self.end = Some(Arc::clone(e.read().unwrap().prev.as_ref().unwrap()));
            self.size -= 1;
            Some(Arc::clone(&e))
        } else {
            None
        }
    }
}

pub struct LRUCache<T: std::fmt::Display + std::clone::Clone> {
  pub map: HashMap<String, Arc<RwLock<Node<T>>>>,
  list: List<T>,
  max_length: u8
}

impl<T: std::fmt::Display + std::clone::Clone> LRUCache<T> {
  pub fn new() -> LRUCache<T> {
    LRUCache { map: HashMap::new(), list: List::new(), max_length: 3 }
  }

  pub fn put(&mut self, key: String, value: T) {
      if !self.map.contains_key(&key) {
          let new_node = Arc::new(RwLock::new(Node::new(key.clone(), value)));
          if self.list.size >= self.max_length {
              if let Some(rn) = self.list.remove_last_node() {
                  self.map.remove(&rn.read().unwrap().key);
              }
          }
          self.list.append(Arc::clone(&new_node));
          self.map.insert(key.clone(), Arc::clone(&new_node));
      }
  }

  pub fn get(&mut self, key: String) -> Option<T> {
    if let Some(node) = self.map.get(&key) {
        self.list.move_to_front(Arc::clone(node));
        return Some(node.read().unwrap().value.clone());
    } else {
        return None;
    }
  }

  pub fn print_list(&mut self) {
      for n in self.list.iter() {
          println!("{}", n.read().unwrap().value);
      }
  }

  pub fn print_map(&self) {
      for (k, v) in &self.map {
          println!("{}: {}", k, v.read().unwrap().value);
      }
  }
}

// fn main() {
//     let mut cache = LRUCache::new();
//     cache.put("first_name".to_string(), "Akshay".to_string());
//     cache.put("middle_name".to_string(), "Vijay".to_string());
//     cache.put("last_name".to_string(), "Takkar".to_string());
//     cache.put("age".to_string(), 28.to_string());
//     cache.put("gender".to_string(), "male".to_string());
//     println!("\n");
//     cache.print_list();
//     cache.print_map();
// }
