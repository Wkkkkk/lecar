use std::cmp::Ordering;
use std::time::Instant;
use crate::cache::CacheItem;

pub struct LFUCacheItem<T> {
    frequency: usize,
    last_used: Instant,
    key: T,
    value: Vec<u8>
}

impl<T: Copy> CacheItem<T> for LFUCacheItem<T> {
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
        "lfu"
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

// /// Implements PartialEq for LFU use
// impl PartialEq for LFUCacheItem {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.frequency() == other.0.frequency()
//     }
// }
//
// /// Implements Ord for LFU use
// impl Ord for LFUCacheItem {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.0.frequency().cmp(&self.0.frequency())
//     }
// }
//
// /// Implements PartialOrd for LFU use
// impl PartialOrd for LFUCacheItem {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
