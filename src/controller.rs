use crate::cache::{Cache, CacheItem, LFUCacheItem, LRUCacheItem, ICache, IPolicy, ICacheItemWrapper};
use std::collections::{HashMap, BinaryHeap};
use crate::enums::Policy;

/// Controlling struct for the cache
/// Keeps a main cache and several (2+) policy caches
/// Uses a learner to determine which policy cache to utilize
#[derive(Debug)]
pub struct Controller {
    cache: Cache<HashMap<usize, CacheItem>>,
    lfu: Cache<BinaryHeap<LFUCacheItem>>,
    lru: Cache<BinaryHeap<LRUCacheItem>>
}

impl Controller {
    /// Instantiates new new Controller given the cache sizes for each cache
    pub fn new(cache_size: usize, lfu_cache_size: usize, lru_cache_size: usize) -> Self {
        Self {
            cache: Cache::new(cache_size),
            lfu: Cache::new(lfu_cache_size),
            lru: Cache::new(lru_cache_size)
        }
    }

    /// Retrieves an item from the cache
    /// Looks into the main cache and retrieves in O(1) time
    /// If it does not exist in the main cache it then tries to find the item in the policy caches
    /// If it does, it ejects the item from the policy cache and inserts it into main cache
    /// Main cache ejects an item if it is full based on a given policy in O(n)
    /// If an item is ejected from the main cache, it then inserts it into a policy cache
    /// Policy cache ejects an item depending on its policy if it is full in O(1) time
    /// Returns the found item or None
    pub fn get(&mut self, key: usize) -> Option<Vec<u8>> {
        let policy = Policy::from_bool(rand::random());

        match self.cache.get(key) {
            Some(item) => Some(item.value().clone()),
            None => {
                match self.find_key_in_policy_caches(key) {
                    Some(ejected_item) => {
                        let value_to_return = ejected_item.value().clone();
                        let maybe_cache_item = self.cache.insert_with_policy(ejected_item, policy);

                        self.insert_into_policy_cache(
                            maybe_cache_item,
                            policy
                        );

                        Some(value_to_return)
                    },
                    None => None
                }
            }
        }
    }

    /// Inserts an item into the cache
    /// It first looks into the policy caches for the item
    /// If found, then ejects it, updates it, and inserts it into the main cache
    /// Otherwise it tries to find the item in the main cache
    /// If it does, it then updates the value
    /// Otherwise it inserts the item and ejects another item via a given policy from the learner
    /// It then inserts that ejected item into a policy cache which will eject an item if full
    pub fn insert(&mut self, key: usize, value: Vec<u8>) {
        let policy = Policy::from_bool(rand::random());

        // Ejected cache item from either the LFU or the LRU, if it exists in either
        match self.find_key_in_policy_caches(key) {
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

    /// Given a cache item and a policy, insert into the given policy cache
    fn insert_into_policy_cache(&mut self, maybe_cache_item: Option<CacheItem>, policy: Policy) {
        if let Some(cache_item) = maybe_cache_item {
            match policy {
                Policy::LFU => self.lfu.insert(LFUCacheItem::new(cache_item)),
                Policy::LRU => self.lru.insert(LRUCacheItem::new(cache_item))
            }
        }
    }

    /// Given a key, find an item in a policy cache if it exists on one
    fn find_key_in_policy_caches(&mut self, key: usize) -> Option<CacheItem> {
        self.lfu
            .maybe_eject_key(key)
            .and_then(|cache_item| Some(cache_item.into_inner()))
            .or_else(|| self.lru
                .maybe_eject_key(key)
                .and_then(|cache_item| Some(cache_item.into_inner()))
            )
    }

    /// Returns a tuple of the current sizes of each cache
    pub fn len(&self) -> (usize, usize, usize) {
        (self.cache.len(), self.lfu.len(), self.lru.len())
    }
}
