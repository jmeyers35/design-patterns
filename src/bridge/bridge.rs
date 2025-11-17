use std::collections::HashMap;
use std::hash::Hash;

// This is the abstraction trait that clients use.
pub trait KvStore<K, V>
where
    K: Eq + Hash,
{
    fn put(&mut self, key: K, val: V);
    fn get(&self, key: K) -> Option<&V>;
}

// KvStore implementation clients will use.
pub struct GenericStore<K, V>
where
    K: Eq + Hash,
{
    store_impl: Box<dyn StorageEngine<K, V>>,
}

// TODO: maybe clean up these static lifetimes later.
impl<K, V> GenericStore<K, V>
where
    K: Eq + Hash + 'static,
    V: 'static,
{
    fn new() -> Self {
        Self {
            store_impl: Box::new(MemoryStore::new()),
        }
    }
}

impl<K, V> KvStore<K, V> for GenericStore<K, V>
where
    K: Eq + Hash + 'static,
    V: 'static,
{
    fn put(&mut self, key: K, val: V) {
        self.store_impl.put(key, val);
    }

    fn get(&self, key: K) -> Option<&V> {
        self.store_impl.get(key)
    }
}

// This is the implementation trait that KvStore implementations will use under the hood.
trait StorageEngine<K, V>
where
    K: Eq + Hash,
{
    fn put(&mut self, key: K, val: V);
    fn get(&self, key: K) -> Option<&V>;
}

// Implements StorageEngine with an in-memory map.
struct MemoryStore<K, V>
where
    K: Eq + Hash,
{
    mem_map: HashMap<K, V>,
}

impl<K, V> MemoryStore<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        let hashmap = HashMap::new();
        Self { mem_map: hashmap }
    }
}

impl<K, V> StorageEngine<K, V> for MemoryStore<K, V>
where
    K: Eq + Hash,
{
    fn put(&mut self, key: K, val: V) {
        self.mem_map.insert(key, val);
    }

    fn get(&self, key: K) -> Option<&V> {
        self.mem_map.get(&key)
    }
}

#[cfg(test)]

mod tests {
    use super::{GenericStore, KvStore};

    #[test]
    fn test_set_and_get() {
        let mut store = GenericStore::new();
        store.put("hello", 42);
        assert_eq!(store.get("hello"), Some(42).as_ref())
    }
}
