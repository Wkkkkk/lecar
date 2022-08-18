use crate::cache::{Cache, CacheItem, LFUCacheItem, LRUCacheItem, ICache, IPolicy, ICacheItemWrapper, Policy};
use self::constants::{DISCOUNT_RATE, LEARNING_RATE};
use rand::random;
use rand::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::default::Default;
use std::collections::BinaryHeap;
use indexmap::IndexMap;
use std::f64::consts::E;
use std::io::Write;
use std::fs::OpenOptions;

mod constants;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Counter {
    // Cache 
    pub size: u64,
    pub num_queries: u64,
    pub hits: u64,
    pub misses: u64,

    // Size
    pub raw_messsages_size: u64,
    pub compressed_size: u64,
    pub raw_len: u64,
    pub encoded_len: u64,

    // Time
    pub compression_time: u64,
    pub decompression_time: u64,
    pub updating_time: u64,

    // Memory
    pub memory_size: u64,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{},{},{},{},{},{},{},{}, {}\n", self.size, self.num_queries, self.hits, self.misses, 
            self.raw_messsages_size, self.compressed_size, self.raw_len, self.encoded_len, self.compression_time, self.decompression_time, self.updating_time, self.memory_size)
    }
}

impl Counter {
    pub fn try_write_to_file(&mut self, path: &str) {
        if self.num_queries < 1000 { return; }

        let output_str = format!("{}", self);
        let mut f = OpenOptions::new()
            .append(true)
            .create(true) // Optionally create the file if it doesn't already exist
            .open(path)
            .expect("Unable to create file");

        f.write_all(output_str.as_bytes()).expect("Unable to write data");
        
        self.reset();
    }

    pub fn reset(&mut self) {
        self.num_queries = 0;
        self.hits = 0;
        self.misses = 0;
        self.raw_messsages_size = 0;
        self.compressed_size = 0;
        self.raw_len = 0;
        self.encoded_len = 0;
        self.compression_time = 0;
        self.decompression_time = 0;
        self.updating_time = 0;
        self.memory_size = 0;
    }
}

/// Controlling struct for the cache
/// Keeps a main cache and several (2+) policy caches
/// Uses a learner to determine which policy cache to utilize
/// TODO: Allow custom policy injection
/// TODO: Allow deserialization for weights probability
#[derive(Serialize, Deserialize, Debug)]
pub struct Controller {
    cache: Cache<IndexMap<String, CacheItem>>,
    lfu: Cache<BinaryHeap<LFUCacheItem>>,
    lru: Cache<BinaryHeap<LRUCacheItem>>,
    lfu_prob: f64,
    rng: ChaCha8Rng,
    pub counter: Counter
}

impl Controller {
    /// Instantiates new new Controller given the cache sizes for each cache
    pub fn new(cache_size: usize, lfu_cache_size: usize, lru_cache_size: usize) -> Self {
        Self {
            cache: Cache::new(cache_size),
            lfu: Cache::new(lfu_cache_size),
            lru: Cache::new(lru_cache_size),
            lfu_prob: 0.5,
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(10),
            counter: Default::default()
        }
    }

    fn get_policy(&mut self) -> Policy {
        let r = self.rng.next_u32();
        let p = r as f64 / u32::MAX as f64;
        // if random::<f64>() <= self.lfu_prob {
        if p <= self.lfu_prob {
            Policy::LFU
        } else {
            Policy::LRU
        }
        // Policy::LFU
    }

    fn update_weights(&mut self, time_duration: f64, miss_from: Policy) {
        let reward = DISCOUNT_RATE.powf(time_duration);
        let mut new_lfu_prob = self.lfu_prob;
        let mut new_lru_prob = 1.0 - self.lfu_prob;

        match miss_from {
            Policy::LFU => new_lru_prob = new_lru_prob * E.powf(LEARNING_RATE * reward),
            Policy::LRU => new_lfu_prob = new_lfu_prob * E.powf(LEARNING_RATE * reward)
        };

        self.lfu_prob = new_lfu_prob / (new_lfu_prob + new_lru_prob);
    }

    /// Retrieves an item from the cache
    /// Looks into the main cache and retrieves in O(1) time
    /// If it does not exist in the main cache it then tries to find the item in the policy caches
    /// If it does, it ejects the item from the policy cache and inserts it into main cache
    /// Main cache ejects an item if it is full based on a given policy in O(n)
    /// If an item is ejected from the main cache, it then inserts it into a policy cache
    /// Policy cache ejects an item depending on its policy if it is full in O(1) time
    /// Returns the found item or None
    pub fn get(&mut self, key: &str) -> Option<String> {
        match self.cache.get(key) {
            // HIT
            Some(item) => Some(item.value().to_string()),
            // MISS
            None => {
                match self.find_key_in_policy_caches(key) {
                    Some((ejected_item, time_duration, old_policy)) => {
                        self.update_weights(time_duration, old_policy);
                        let value_to_return = ejected_item.value().to_string();
                        let policy = self.get_policy();
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

    pub fn get_index(&self, index: usize) -> Option<&CacheItem> {
        self.cache.get_index(index)
    }

    /// Retrieves the index of an item from the cache
    pub fn get_index_of(&self, key: &str) -> Option<usize> {
        self.cache.get_index_of(key)
    }

    /// Inserts an item into the cache
    /// It first looks into the policy caches for the item
    /// If found, then ejects it, updates it, and inserts it into the main cache
    /// Otherwise it tries to find the item in the main cache
    /// If it does, it then updates the value
    /// Otherwise it inserts the item and ejects another item via a given policy from the learner
    /// It then inserts that ejected item into a policy cache which will eject an item if full
    pub fn insert(&mut self, key: &str, value: String) {
        // Ejected cache item from either the LFU or the LRU, if it exists in either
        match self.find_key_in_policy_caches(key) {
            // If cache item existed in policy caches
            // Update it
            // Insert into main cache given the new policy
            Some((mut ejected_item, time_duration, old_policy)) => {
                ejected_item.update(value);

                self.update_weights(time_duration, old_policy);
                let policy = self.get_policy();

                let maybe_cache_item = self.cache.insert_with_policy(ejected_item, policy);

                self.insert_into_policy_cache(
                    maybe_cache_item,
                    policy
                )
            },
            // Cache item was not found in the policy caches
            // Add it to cache
            None => {
                let policy = self.get_policy();
                let maybe_cache_item = self.cache.insert_with_policy(CacheItem::new(key.to_string(), value), policy);

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
    fn find_key_in_policy_caches(&mut self, key: &str) -> Option<(CacheItem, f64, Policy)> {
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

    pub fn full(&self) -> bool {
        self.cache.len() == self.cache.capacity
    }

    pub fn print_size(&self) -> usize {
        let serialized = serde_json::to_string(&self).unwrap();

        serialized.len()
    }
}
