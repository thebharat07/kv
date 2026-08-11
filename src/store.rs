use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, RwLock};

const NUM_SHARDS: u64 = 16;

#[derive(Clone)]
pub struct ShardedStore {
    map_shards: Arc<Vec<Arc<RwLock<BTreeMap<String, String>>>>>,
}

impl ShardedStore {
    pub fn new() -> Self {
        let map_shards: Vec<Arc<RwLock<BTreeMap<String, String>>>> = (0..NUM_SHARDS)
            .map(|_| Arc::new(RwLock::new(BTreeMap::new())))
            .collect();

        let map_shards = Arc::new(map_shards);

        ShardedStore { map_shards }
    }

    pub fn shard_index(key: &str) -> usize {
        let mut hash_function = DefaultHasher::new();

        key.hash(&mut hash_function);

        let hash_of_key = hash_function.finish();

        (hash_of_key % NUM_SHARDS) as usize
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let idx = Self::shard_index(key);

        self.map_shards[idx].read().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: &str, val: &str){
        let idx = Self::shard_index(key);

        self.map_shards[idx].write().unwrap().insert(String::from(key), String::from(val));
    }
}
