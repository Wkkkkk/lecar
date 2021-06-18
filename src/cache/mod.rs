use std::collections::{BinaryHeap, HashMap};

mod cache_item;
mod enums;
mod lfu;
mod lfu_cache;
mod lru;
mod standard_cache;

pub use self::{
    cache_item::CacheItem,
    enums::Policy,
    lfu::LFUCacheItem,
    lru::LRUCacheItem,
    standard_cache::Cache
};
use std::cmp::Ordering;
use std::hash::Hash;

pub trait Caching<T: Copy + Eq + Hash> {
    fn new(capacity: usize) -> Self where Self: Sized;
    fn contains(&self, key: T) -> bool;
    fn get_capacity(&self) -> usize;
    fn get_item_ref(&self, key: T) -> Option<&Box<dyn CacheItem<T>>>;
    fn insert_item_maybe_eject(&mut self, item: Box<dyn CacheItem<T>>) -> Option<Box<dyn CacheItem<T>>>;
    fn insert_item_maybe_eject_with_policy(&mut self, item: Box<dyn CacheItem<T>>, policy: fn((T, &dyn CacheItem<T>), (T, &dyn CacheItem<T>)) -> Ordering) -> Option<Box<dyn CacheItem<T>>>;
    fn len(&self) -> usize;
}

// /// Trait which gives an interface for retrieving the wrapped item's key
// /// Or converting the item into the wrapped item
// pub trait ICacheItemWrapper<'a> {
//     fn get_inner_key(&self) -> usize;
//     fn get_duration(&self) -> f64;
//     fn get_name(&self) -> &'a str;
//     fn into_inner(self) -> (CacheItem, f64, &'a str);
// }
//
// /// Trait to enforce interface for caches
// pub trait ICache {
//     fn new(capacity: usize) -> Self;
//     fn contains(&self, key: usize) -> bool;
//     fn len(&self) -> usize;
// }
//
// /// Trait to give a policy to a cache
// /// Tells the cache HOW to to evict and insert items
// pub trait IPolicy<'a, I: ICacheItemWrapper<'a>> {
//     fn eject(&mut self);
//     fn maybe_eject_key(&mut self, key: usize) -> Option<I>;
//     fn insert(&mut self, cache_item: I);
// }
//
// /// Basic struct for caches
// #[derive(Debug)]
// pub struct Cache<C> {
//     capacity: usize,
//     cache: C
// }
//
// /// Implementation of ICache for a BinaryHeap (priority queue) Cache
// impl<'a, I: ICacheItemWrapper<'a> + Ord> ICache for Cache<&'a BinaryHeap<&'a I>> {
//     fn new(capacity: usize) -> Self {
//         Self {
//             capacity,
//             cache: &BinaryHeap::with_capacity(capacity)
//         }
//     }
//
//     fn contains(&self, key: usize) -> bool {
//         self.cache.iter().any(|i| i.get_inner_key() == key)
//     }
//
//     fn len(&self) -> usize {
//         self.cache.len()
//     }
// }
//
// /// Implementation of ICache for a HashMap cache
// impl ICache for Cache<HashMap<usize, CacheItem>> {
//     fn new(capacity: usize) -> Self {
//         Self {
//             capacity,
//             cache: HashMap::with_capacity(capacity)
//         }
//     }
//
//     fn contains(&self, key: usize) -> bool {
//         self.cache.contains_key(&key)
//     }
//
//     fn len(&self) -> usize {
//         self.cache.len()
//     }
// }
//
// /// Implementation of a key / value cache with a max capacity
// impl Cache<HashMap<usize, CacheItem>> {
//     /// Retrieves a cached item and updates it before returning it
//     pub fn get(&mut self, key: usize) -> Option<&CacheItem> {
//         self.cache.get_mut(&key).and_then(|ci| {
//             ci.touch();
//             Some(&*ci)
//         })
//     }
//
//     /// Inserts an item to the cache
//     /// Updates an already existing item
//     /// Fails an returns item if the cache is full
//     #[allow(dead_code)]
//     pub fn insert(&mut self, new_item: CacheItem) -> Option<CacheItem> {
//         if self.cache.len() == self.capacity && !self.cache.contains_key(&new_item.key()) {
//             return Some(new_item);
//         }
//
//         self.cache.insert(new_item.key(), new_item);
//         None
//     }
//
//     /// Inserts an item to the cache
//     /// Updates an already existing item
//     /// If the cache is full, given a policy, ejects an item that matches the policy
//     pub fn insert_with_policy(&mut self, new_item: CacheItem, policy_compare: &'a dyn Fn(&CacheItem, &CacheItem) -> Ordering) -> Option<CacheItem> {
//         match self.cache.get_mut(&new_item.key()) {
//             Some(item) => {
//                 item.update(new_item.value_owned());
//                 None
//             },
//             None => {
//                 if self.capacity > self.cache.len() {
//                     self.cache.insert(new_item.key(), new_item);
//                     None
//                 } else {
//                     let item_to_remove = *self.cache
//                         .iter()
//                         .min_by(|(_lk, li), (_rk, ri)| {
//                             (*policy_compare)(&(**li), &(**ri))
//                             // match policy {
//                             //     Policy::LFU => li.frequency().cmp(&ri.frequency()),
//                             //     Policy::LRU => li.last_used().cmp(&ri.last_used())
//                             // }
//                         })
//                         .unwrap()
//                         .0;
//
//                     let lfu_item = Some(self.cache.remove(&item_to_remove).unwrap());
//                     self.cache.insert(new_item.key(), new_item);
//
//                     lfu_item
//                 }
//             }
//         }
//     }
// }
//
// /// Implementation of IPolicy for BinaryHeap (priority queue) caches
// /// The ordering of the BinaryHeap depends on the generic I item
// impl<'a, I: ICacheItemWrapper<'a> + Ord> IPolicy<'a, I> for Cache<BinaryHeap<I>> {
//     /// Ejects the first ordered item in the BinaryHeap
//     fn eject(&mut self) {
//         self.cache.pop();
//     }
//
//     /// Iterates over the BinaryHeap to find an item given a key
//     /// If found, ejects, reorders BinaryHeap, and returns the item
//     fn maybe_eject_key(&mut self, key: usize) -> Option<I> {
//         match self.cache.iter().filter(|item| item.get_inner_key() == key).next() {
//             Some(t) => {
//                 let cloned_item = t.clone();
//                 self.cache.retain(|item| item.get_inner_key() != key);
//                 Some(*cloned_item)
//             },
//             None => None
//         }
//     }
//
//     /// If the cache is full, eject an item from the cache
//     /// Then insert the given item into the cache
//     fn insert(&mut self, cache_item: I) {
//         if self.capacity == self.cache.len() {
//             self.eject();
//         }
//
//         self.cache.push(cache_item);
//     }
// }
