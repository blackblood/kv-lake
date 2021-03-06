use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
pub mod my_node;
pub mod my_list;

pub struct LRUCache<T: std::fmt::Display + std::clone::Clone> {
  pub map: HashMap<String, Arc<RwLock<my_node::Node<T>>>>,
  list: my_list::List<T>,
  max_length: u32
}

impl<T: std::fmt::Display + std::clone::Clone> LRUCache<T> {
  pub fn new(queue_size: u32) -> LRUCache<T> {
    LRUCache { map: HashMap::new(), list: my_list::List::new(), max_length: queue_size }
  }
}

impl<T: std::fmt::Display + std::clone::Clone> super::Cacheable<T> for LRUCache<T> {
    fn put(&mut self, key: String, value: T) {
        if !self.map.contains_key(&key) {
            let new_node = Arc::new(RwLock::new(my_node::Node::new(key.clone(), value)));
            if self.list.size >= self.max_length {
                if let Some(rn) = self.list.remove_last_node() {
                    self.map.remove(&rn.read().unwrap().key);
                }
            }
            self.list.append(Arc::clone(&new_node));
            self.map.insert(key.clone(), Arc::clone(&new_node));
        }
    }

    fn get(&mut self, key: String) -> Option<T> {
      if let Some(node) = self.map.get(&key) {
          self.list.move_to_front(Arc::clone(node));
          return Some(node.read().unwrap().value.clone());
      } else {
          return None;
      }
    }

    fn delete(&mut self, key: String) -> Result<(), String> {
        if let Some(node) = self.map.get(&key) {
            node.write().unwrap().join_neighbours(&mut self.list);
            self.map.remove(&key);
            return Ok(());
        } else {
            return Err("key not found".to_string());
        }
    }

    fn print_map(&self) {
        for (k, v) in &self.map {
            println!("{}: {}", k, v.read().unwrap().value);
        }
    }

    fn print_list(&self) {
        for n in self.list.iter() {
            println!("{}", n.read().unwrap().value);
        }
    }
}
