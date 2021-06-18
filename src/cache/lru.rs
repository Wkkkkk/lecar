use std::cmp::Ordering;
use crate::cache::{CacheItem, Policy};
use std::time::Instant;

pub struct LRUCacheItem<T> {
    frequency: usize,
    last_used: Instant,
    key: T,
    value: Vec<u8>
}

impl<T: Copy> CacheItem<T> for LRUCacheItem<T> {
    fn new(key: T, value: Vec<u8>) -> Self {
        Self {
            frequency: 0,
            last_used: Instant::now(),
            key,
            value
        }
    }

    fn from_boxed_type(other_type: Box<dyn CacheItem<T>>) -> Box<Self> {
        Box::new(
            Self {
                frequency: other_type.get_frequency(),
                last_used: other_type.get_last_used(),
                key: *other_type.get_key(),
                value: other_type.into_value()
            }
        )
    }

    fn get_duration(&self) -> f64 {
        self.last_used.elapsed().as_secs_f64()
    }

    fn get_frequency(&self) -> usize {
        self.frequency
    }

    fn get_key(&self) -> &T {
        &self.key
    }

    fn get_last_used(&self) -> Instant {
        self.last_used
    }

    fn get_name(&self) -> &'static str {
        "lru"
    }

    fn get_value(&self) -> &Vec<u8> {
        &self.value
    }

    fn into_value(self) -> Vec<u8> {
        self.value
    }

    fn touch(&mut self) {
        self.frequency += 1;
        self.last_used = Instant::now();
    }

    fn update(&mut self, value: Vec<u8>) {
        self.value = value;
        self.touch();
    }
}

// /// Implementation of the LRUCacheItem
// impl LRUCacheItem {
//     /// Returns a wrapped CacheItem
//     pub fn new(cache_item: CacheItem) -> Self {
//         Self(cache_item, Instant::now())
//     }
// }
//
// /// Implements PartialEq for LRU use
// impl PartialEq for LRUCacheItem {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.last_used().eq(&other.0.last_used())
//     }
// }
//
// /// Implements Ord for LRU use
// impl Ord for LRUCacheItem {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.0.last_used().cmp(&self.0.last_used())
//     }
// }
//
// /// Implements PartialOrd for LRU use
// impl PartialOrd for LRUCacheItem {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
