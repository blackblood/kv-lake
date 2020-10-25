use std::collections::HashMap;
use std::collections::VecDeque;

pub struct LRUCache<T> {
  pub map: HashMap<String, T>,
  que: VecDeque<String>,
  max_length: u8
}

fn index_of(key: &String, que: &VecDeque<String>) -> Option<usize> {
  for (i, el) in que.iter().enumerate() {
    if key.as_str() == el {
      return Some(i);
    }
  }
  return None;
}

impl<T> LRUCache<T> {
  pub fn new() -> LRUCache<T> {
    LRUCache { map: HashMap::new(), que: VecDeque::new(), max_length: 3 }
  }

  pub fn put(&mut self, key: String, value: T) {
    self.map.insert(key.clone(), value);
    if let Some(del_index) = index_of(&key, &self.que) {
      self.que.remove(del_index);
    }
    self.que.push_front(key.clone());
    if (self.que.len() as u8) > self.max_length {
      if let Some(last_el) = self.que.pop_back() {
        self.map.remove(&last_el);
      }
    }
  }

  pub fn get(&mut self, key: String) -> Option<&T> {
    if let Some(del_index) = index_of(&key, &self.que) {
      self.que.remove(del_index);
      self.que.push_front(key.clone());
    }
    self.map.get(&key)
  }
}