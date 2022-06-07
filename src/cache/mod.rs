use std::collections::BinaryHeap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

mod cache_item;
mod enums;
mod lfu;
mod lru;

pub use self::{
    cache_item::CacheItem,
    enums::Policy,
    lfu::LFUCacheItem,
    lru::LRUCacheItem
};

/// Trait which gives an interface for retrieving the wrapped item's key
/// Or converting the item into the wrapped item
pub trait ICacheItemWrapper: Clone + Eq + PartialEq + Ord + PartialOrd {
    fn get_inner_key(&self) -> &str;
    fn get_duration(&self) -> f64;
    fn into_inner(self) -> (CacheItem, f64, Policy);
}

/// Trait to enforce interface for caches
pub trait ICache {
    fn new(capacity: usize) -> Self;
    fn contains(&self, key: &str) -> bool;
    fn len(&self) -> usize;
    fn full(&self) -> bool;
}

/// Trait to give a policy to a cache
/// Tells the cache HOW to to evict and insert items
pub trait IPolicy<I: ICacheItemWrapper> {
    fn eject(&mut self);
    fn maybe_eject_key(&mut self, key: &str) -> Option<I>;
    fn insert(&mut self, cache_item: I);
}

/// Basic struct for caches
#[derive(Serialize, Deserialize, Debug)]
pub struct Cache<C> {
    pub capacity: usize,
    cache: C
}

/// Implementation of ICache for a BinaryHeap (priority queue) Cache
impl<I: ICacheItemWrapper> ICache for Cache<BinaryHeap<I>> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: BinaryHeap::with_capacity(capacity)
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.cache.iter().any(|i| i.get_inner_key() == key)
    }

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn full(&self) -> bool {
        self.capacity == self.cache.len()
    }
}

/// Implementation of ICache for a IndexMap cache
impl ICache for Cache<IndexMap<String, CacheItem>> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: IndexMap::with_capacity(capacity)
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn full(&self) -> bool {
        self.capacity == self.cache.len()
    }
}

/// Implementation of a key / value cache with a max capacity
impl Cache<IndexMap<String, CacheItem>> {
    /// Retrieves a cached item and updates it before returning it
    pub fn get(&mut self, key: &str) -> Option<&CacheItem> {
        self.cache.get_mut(key).and_then(|ci| {
            ci.touch();
            Some(&*ci)
        })
    }

    pub fn get_index_of(&self, key: &str) -> Option<usize> {
        self.cache.get_index_of(key)
    }

    pub fn get_index(&self, index: usize) -> Option<&CacheItem> {
        match self.cache.get_index(index) {
            Some((_key, item)) => {
                Some(item)
            }
            None => None
        }
    }

    /// Inserts an item to the cache
    /// Updates an already existing item
    /// Fails an returns item if the cache is full
    #[allow(dead_code)]
    pub fn insert(&mut self, new_item: CacheItem) -> Option<CacheItem> {
        if self.full() && !self.cache.contains_key(new_item.key()) {
            return Some(new_item);
        }

        self.cache.insert(new_item.key().to_string(), new_item);
        None
    }

    /// Inserts an item to the cache
    /// Updates an already existing item
    /// If the cache is full, given a policy, ejects an item that matches the policy
    pub fn insert_with_policy(&mut self, new_item: CacheItem, policy: Policy) -> Option<CacheItem> {
        match self.cache.get_mut(new_item.key()) {
            Some(item) => {
                item.update(new_item.value_owned());
                None
            },
            None => {
                if self.capacity > self.cache.len() {
                    self.cache.insert(new_item.key().to_string(), new_item);
                    None
                } else {
                    let item_to_remove = self.cache
                        .iter()
                        .min_by(|(_lk, li), (_rk, ri)| {
                            match policy {
                                Policy::LFU => li.frequency().cmp(&ri.frequency()),
                                Policy::LRU => li.last_used().cmp(&ri.last_used())
                            }
                        })
                        .unwrap()
                        .0
                        .to_string();

                    let lfu_item = Some(self.cache.remove(&item_to_remove).unwrap());
                    self.cache.insert(new_item.key().to_string(), new_item);

                    lfu_item
                }
            }
        }
    }
}

/// Implementation of IPolicy for BinaryHeap (priority queue) caches
/// The ordering of the BinaryHeap depends on the generic I item
impl<I: ICacheItemWrapper> IPolicy<I> for Cache<BinaryHeap<I>> {
    /// Ejects the first ordered item in the BinaryHeap
    fn eject(&mut self) {
        self.cache.pop();
    }

    /// Iterates over the BinaryHeap to find an item given a key
    /// If found, ejects, reorders BinaryHeap, and returns the item
    fn maybe_eject_key(&mut self, key: &str) -> Option<I> {
        match self.cache.iter().filter(|item| item.get_inner_key() == key).next() {
            Some(t) => {
                let cloned_item = t.clone();
                self.cache.retain(|item| item.get_inner_key() != key);
                Some(cloned_item)
            },
            None => None
        }
    }

    /// If the cache is full, eject an item from the cache
    /// Then insert the given item into the cache
    fn insert(&mut self, cache_item: I) {
        if self.full() {
            self.eject();
        }

        self.cache.push(cache_item);
    }
}
