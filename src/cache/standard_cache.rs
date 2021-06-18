use std::collections::HashMap;
use crate::cache::{CacheItem, Caching};
use std::cmp::Ordering;
use std::time::Instant;
use std::hash::Hash;

pub struct StandardCacheItem<T> {
    frequency: usize,
    last_used: Instant,
    key: T,
    value: Vec<u8>
}

impl<T: Copy> CacheItem<T> for StandardCacheItem<T> {
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
        "standard"
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

pub struct Cache<T> {
    capacity: usize,
    data: HashMap<T, Box<dyn CacheItem<T>>>
}

impl<T: Copy + Eq + Hash> Caching<T> for Cache<T> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: HashMap::with_capacity(capacity)
        }
    }

    fn contains(&self, key: T) -> bool {
        self.data.contains_key(&key)
    }

    fn get_capacity(&self) -> usize {
        self.capacity
    }

    fn get_item_ref(&self, key: T) -> Option<&Box<dyn CacheItem<T>>> {
        self.data.get(&key)
    }

    fn insert_item_maybe_eject(&mut self, item: Box<dyn CacheItem<T>>) -> Option<Box<dyn CacheItem<T>>> {
        unimplemented!()
    }

    fn insert_item_maybe_eject_with_policy(&mut self, item: Box<dyn CacheItem<T>>, policy: fn((T, &dyn CacheItem<T>), (T, &dyn CacheItem<T>)) -> Ordering) -> Option<Box<dyn CacheItem<T>>> {
        let maybe_key_to_remove = self.data.iter().min_by(
            |(&left_key, &left_item), (&right_key, &right_item)| {
                policy((left_key, &*left_item), (right_key, &*right_item))
            }
        );

        let mut removed_item = None;
        if let Some((&key_to_remove, _)) = maybe_key_to_remove {
            removed_item = self.data.remove(&key_to_remove);
        }

        self.data.insert(*item.get_key(), item);
        removed_item
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
