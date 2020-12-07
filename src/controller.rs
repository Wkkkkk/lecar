use crate::cache::{Cache, CacheItem, LFUCacheItem, LRUCacheItem, ICache, IPolicy, ICacheItemWrapper};
use std::collections::{HashMap, BinaryHeap};
use crate::enums::Policy;

#[derive(Debug)]
pub struct Controller {
    cache: Cache<HashMap<usize, CacheItem>>,
    lfu: Cache<BinaryHeap<LFUCacheItem>>,
    lru: Cache<BinaryHeap<LRUCacheItem>>
}

impl Controller {
    pub fn new(cache_size: usize, lfu_cache_size: usize, lru_cache_size: usize) -> Self {
        Self {
            cache: Cache::new(cache_size),
            lfu: Cache::new(lfu_cache_size),
            lru: Cache::new(lru_cache_size)
        }
    }

    pub fn get(&mut self, key: usize) -> Option<Vec<u8>> {
        let policy = Policy::from_bool(rand::random());

        match self.cache.get(key) {
            Some(item) => Some(item.value().clone()),
            None => {
                match self.lfu.maybe_eject_key(key) {
                    Some(lfu_item) => {
                        let item = lfu_item.into_inner();
                        let item_value = item.value().clone();

                        let maybe_cache_item = self.cache.insert_with_policy(item, policy);

                        self.insert_into_policy_cache(
                            maybe_cache_item,
                            policy
                        );

                        Some(item_value)
                    },
                    None => None
                }
            }
        }
    }

    pub fn insert(&mut self, key: usize, value: Vec<u8>) {
        let policy = Policy::from_bool(rand::random());

        // Ejected cache item from either the LFU or the LRU, if it exists in either
        let maybe_cache_item = self.lfu
            .maybe_eject_key(key)
            .and_then(|cache_item| Some(cache_item.into_inner()))
            .or_else(|| self.lru
                .maybe_eject_key(key)
                .and_then(|cache_item| Some(cache_item.into_inner()))
            );

        match maybe_cache_item {
            // If cache item existed in policy caches
            // Update it
            // Insert into main cache given the new policy
            Some(mut ejected_item) => {
                ejected_item.update(value);

                let maybe_cache_item = self.cache.insert_with_policy(ejected_item, policy);

                self.insert_into_policy_cache(
                    maybe_cache_item,
                    policy
                )
            },
            // Cache item was not found in the policy caches
            // Add it to cache
            None => {
                let maybe_cache_item = self.cache.insert_with_policy(CacheItem::new(key, value), policy);

                self.insert_into_policy_cache(
                    maybe_cache_item,
                    policy
                )
            }
        }
    }

    fn insert_into_policy_cache(&mut self, maybe_cache_item: Option<CacheItem>, policy: Policy) {
        if let Some(cache_item) = maybe_cache_item {
            match policy {
                Policy::LFU => self.lfu.insert(LFUCacheItem::new(cache_item)),
                Policy::LRU => self.lru.insert(LRUCacheItem::new(cache_item))
            }
        }
    }

    pub fn len(&self) -> (usize, usize, usize) {
        (self.cache.len(), self.lfu.len(), self.lru.len())
    }
}
