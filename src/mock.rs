use std::{hash::Hash, sync::Arc};

#[repr(transparent)]
#[derive(Clone)]
pub struct Db<K, V> {
    // concurrent hashmap
    map: Arc<dashmap::DashMap<K, V>>,
}

unsafe impl<K, V> Send for Db<K, V> {}

unsafe impl<K, V> Sync for Db<K, V> {}

impl<K: Eq + Hash, V> Db<K, V> {
    pub fn init() -> Db<K, V> {
        Db {
            map: Arc::new(dashmap::DashMap::new()),
        }
    }
}

// implement CRUD
impl<K: Eq + PartialEq + Hash, V: Clone> Db<K, V> {
    pub fn create(&self, key: K, value: V) -> bool {
        if self.map.contains_key(&key) {
            false
        } else {
            self.map.insert(key, value);
            true
        }
    }

    pub fn read(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|r| r.value().clone())
    }

    pub fn update(&self, key: &K, value: V) -> Option<V> {
        self.map
            .get_mut(key)
            .map(|mut val| std::mem::replace(val.value_mut(), value))
    }

    pub fn delete(&self, name: &K) -> Option<V> {
        self.map.remove(name).map(|(_, val)| val)
    }
}
