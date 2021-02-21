pub mod frequency_node;
pub mod list;
pub mod my_node;
use std::collections::HashMap;
use std::sync::{ Arc, RwLock };

pub struct LFUCache<T: std::fmt::Display + std::clone::Clone> {
    pub map: HashMap<String, Arc<RwLock<my_node::Node<T>>>>,
    frequency_node: Arc<RwLock<frequency_node::FrequencyNode<T>>>,
    max_length: u32,
    total_node_count: u32
}

pub struct FrequencyListIterator<T: std::fmt::Display + std::clone::Clone> {
    ptr: Option<Arc<RwLock<frequency_node::FrequencyNode<T>>>>
}

impl<T: std::fmt::Display + std::clone::Clone> Iterator for FrequencyListIterator<T> {
    type Item = Arc<RwLock<frequency_node::FrequencyNode<T>>>;

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

impl<T: std::fmt::Display + std::clone::Clone> LFUCache<T> {
    pub fn frequency_list_iter(&self) -> FrequencyListIterator<T> {
        FrequencyListIterator { ptr: Some(Arc::clone(&self.frequency_node)) }
    }

    pub fn new(queue_size: u32) -> LFUCache<T> {
        LFUCache {
            map: HashMap::new(),
            frequency_node: Arc::new(RwLock::new(frequency_node::FrequencyNode::new(1))),
            max_length: queue_size,
            total_node_count: 0
        }
    }

    fn get_next_frequency_node(n: &Arc<RwLock<my_node::Node<T>>>) -> Option<Arc<RwLock<frequency_node::FrequencyNode<T>>>> {
        let curr_node = n.read().unwrap();
        let curr_freq_node = curr_node.freq_node.read().unwrap();
        // println!("curr_freq_node.frequency = {}", curr_freq_node.frequency);
        if let Some(next_freq_node) = curr_freq_node.next.as_ref() {
            if next_freq_node.read().unwrap().frequency == curr_freq_node.frequency + 1 {
                return Some(Arc::clone(next_freq_node));
            }
        }
        return None;
    }

    fn move_to_higher_frequency(current_node: Arc<RwLock<my_node::Node<T>>>) {
        {
            let mut curr_node = current_node.write().unwrap();
            curr_node.join_neighbours();
        }
        if let Some(next_freq_node) = LFUCache::get_next_frequency_node(&current_node) {
            let mut next_freq_node_mut = next_freq_node.write().unwrap();
            // println!("next_freq_node.frequency = {}", next_freq_node_mut.frequency);
            next_freq_node_mut.list.prepend(Arc::clone(&current_node));
            let mut curr_node = current_node.write().unwrap();
            curr_node.freq_node = Arc::clone(&next_freq_node);
            return ();
        }

        let new_freq_node_ptr;
        {
            let curr_node = current_node.write().unwrap();
            let mut curr_freq_node = curr_node.freq_node.write().unwrap();
            let new_freq_node = frequency_node::FrequencyNode::new(curr_freq_node.frequency + 1);
            new_freq_node_ptr = Arc::new(RwLock::new(new_freq_node));
            if let Some(nxt_freq_node) = curr_freq_node.next.as_ref() {
                let mut new_fn = new_freq_node_ptr.write().unwrap();
                new_fn.next = Some(Arc::clone(nxt_freq_node));
                new_fn.prev = Some(Arc::clone(&curr_node.freq_node));
                nxt_freq_node.write().unwrap().prev = Some(Arc::clone(&new_freq_node_ptr));
                curr_freq_node.next = Some(Arc::clone(&new_freq_node_ptr));
            } else {
                let mut new_fn = new_freq_node_ptr.write().unwrap();
                new_fn.prev = Some(Arc::clone(&curr_node.freq_node));
                curr_freq_node.next = Some(Arc::clone(&new_freq_node_ptr));
            }
        }

        {
            let curr_node = current_node.write().unwrap();
            let mut freq_n = curr_node.freq_node.write().unwrap();
            if freq_n.list.head.is_none() && freq_n.list.end.is_none() {
                if let Some(next) = freq_n.next.as_ref() {
                    if let Some(prev) = freq_n.prev.as_ref() {
                        next.write().unwrap().prev = Some(Arc::clone(prev));
                    }
                }
                if let Some(prev) = freq_n.prev.as_ref() {
                    if let Some(next) = freq_n.next.as_ref() {
                        prev.write().unwrap().next = Some(Arc::clone(next));
                    }
                }
                freq_n.next = None;
                freq_n.prev = None;
            }
        }

        {
            let mut curr_node = current_node.write().unwrap();
            curr_node.freq_node = Arc::clone(&new_freq_node_ptr);
        }

        {
            let mut new_freq_node_mut = new_freq_node_ptr.write().unwrap();
            new_freq_node_mut.list.prepend(Arc::clone(&current_node));
        }
    }
}

impl<T: std::fmt::Display + std::clone::Clone> super::Cacheable<T> for LFUCache<T> {
    fn put(&mut self, key: String, value: T) {
        if !self.map.contains_key(&key) {
            let new_node = Arc::new(RwLock::new(my_node::Node::new(key.clone(), value, &self.frequency_node)));
            {
                let mut freq_n = self.frequency_node.write().unwrap();
                if self.total_node_count >= self.max_length {
                    if let Some(e) = freq_n.list.end.take() {
                        self.map.remove(&e.read().unwrap().key);
                        let mut list_end = e.write().unwrap();
                        if let Some(new_e) = list_end.prev.take() {
                            new_e.write().unwrap().next = None;
                            freq_n.list.end = Some(Arc::clone(&new_e));
                        }
                        std::mem::drop(&list_end.freq_node);
                    }
                    self.total_node_count -= 1;
                }
                freq_n.list.prepend(Arc::clone(&new_node));
                self.map.insert(key.clone(), Arc::clone(&new_node));
                self.total_node_count += 1;
            }
        } else {
            if let Some(node) = self.map.get(&key) {
                let mut node_w = node.write().unwrap();
                node_w.value = value;
            }
        }
    }

    fn get(&mut self, key: String) -> Option<T> {
        if let Some(node) = self.map.get(&key) {
            LFUCache::move_to_higher_frequency(Arc::clone(&node));
            return Some(node.read().unwrap().value.clone());
        } else {
            return None;
        }
    }

    fn delete(&mut self, key: String) -> Result<(), String> {
        if let Some(node) = self.map.get(&key) {
            node.write().unwrap().join_neighbours();
            self.map.remove(&key);
            self.total_node_count -= 1;
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
        for fr_n in self.frequency_list_iter() {
            println!("frequency_node: {}", fr_n.read().unwrap().frequency);
            for n in fr_n.write().unwrap().list.iter() {
                println!("{}", n.read().unwrap().value);
            }
        }
    }
}
