use self::constants::{DISCOUNT_RATE, LEARNING_RATE};
use rand::random;
use std::collections::{HashMap, BinaryHeap};
use std::f64::consts::E;
use std::cmp::Ordering;
use crate::cache::{Cache, CacheItem, Caching};
use std::hash::Hash;

mod constants;

/// Controlling struct for the cache
/// Keeps a main cache and several (2+) policy caches
/// Uses a learner to determine which policy cache to utilize
/// TODO: Allow custom policy injection
/// TODO: Allow deserialization for weights probability
pub struct Controller<'a, T: Copy + Eq + Hash> {
    cache: Cache<HashMap<T, Box<dyn CacheItem<T>>>>,
    policies: HashMap<&'a str,  &'a Cache<BinaryHeap<Box<dyn CacheItem<T>>>>>,
    policy_compares: HashMap<&'a str, fn((T, &dyn CacheItem<T>), (T, &dyn CacheItem<T>)) -> Ordering>,
    probabilities: HashMap<&'a str, f64>
}

impl<'a, T: Copy + Eq + Hash> Controller<'a, T> {
    /// Instantiates new new Controller given the cache sizes for each cache
    pub fn new(cache_size: usize, policies: Vec<(&'a str, usize, fn((T, &dyn CacheItem<T>), (T, &dyn CacheItem<T>)) -> Ordering)>) -> Self {
        let mut policies_map = HashMap::with_capacity(policies.len());
        let mut policy_compare_map = HashMap::with_capacity(policies.len());
        let mut probabilities_map = HashMap::with_capacity(policies.len());
        let probability = 1.0 / policies.len() as f64;
        policies.iter().for_each(|(name, size, compare_function)| {
            policies_map.insert(*name, &Cache::new(*size));
            policy_compare_map.insert(*name, *compare_function);
            probabilities_map.insert(*name, probability);
        });

        Self {
            cache: Cache::new(cache_size),
            policies: policies_map,
            policy_compares: policy_compare_map,
            probabilities: probabilities_map
        }
    }

    fn get_policy(&self) -> &str {
        let random_prob = random::<f64>();
        let mut cumulative_prob = 0.0;
        let mut last_policy = "";

        for (name, probability) in self.probabilities {
            cumulative_prob += probability;
            last_policy = name;

            if random_prob <= cumulative_prob {
                return name;
            }
        }

        last_policy
    }

    fn update_weights(&mut self, time_duration: f64, miss_from: &str) {
        let reward = DISCOUNT_RATE.powf(time_duration);

        if let Some(mut miss_probability) = self.probabilities.get_mut(miss_from) {
            *miss_probability *= E.powf(-(LEARNING_RATE * reward));
        }

        let mut probability_sum = 0.0;
        self.probabilities.iter().for_each(|(&_name, probability)| {
            probability_sum += probability;
        });

        self.probabilities.iter_mut().for_each(|(&_name, mut probability)| {
            *probability = probability / probability_sum;
        });
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
        // HIT
        if let Some(item) = self.cache.get(key) {
            Some(item.value().clone())
        // MISS - Check Policy Caches
        } else if let Some((ejected_item, time_duration, old_policy)) = self.find_key_in_policy_caches(key) {
            self.update_weights(time_duration, old_policy);
            let value_to_return = ejected_item.value().clone();
            let policy = self.get_policy();
            let maybe_cache_item = self.cache.insert_with_policy(ejected_item, self.policy_compares.get(&policy).unwrap());

            self.insert_into_policy_cache(
                maybe_cache_item,
                policy
            );

            Some(value_to_return)
        // MISS
        } else {
            None
        }
    }

    /// Inserts an item into the cache
    /// It first looks into the policy caches for the item
    /// If found, then ejects it, updates it, and inserts it into the main cache
    /// Otherwise it tries to find the item in the main cache
    /// If it does, it then updates the value
    /// Otherwise it inserts the item and ejects another item via a given policy from the learner
    /// It then inserts that ejected item into a policy cache which will eject an item if full
    pub fn insert(&mut self, key: T, value: Vec<u8>) {
        // Ejected cache item from either the LFU or the LRU, if it exists in either
        if let Some((mut ejected_item, time_duration, old_policy)) = self.find_key_in_policy_caches(key) {
            // If cache item existed in policy caches
            // Update it
            // Insert into main cache given the new policy
            ejected_item.update(value);

            self.update_weights(time_duration, old_policy);
            let policy = self.get_policy();

            let maybe_cache_item = self.cache.insert_item_maybe_eject_with_policy(ejected_item, self.policy_compares.get(policy).unwrap());

            self.insert_into_policy_cache(
                maybe_cache_item,
                policy
            )
        } else {
            // Cache item was not found in the policy caches
            // Add it to cache
            let policy = self.get_policy();
            let maybe_cache_item = self.cache.insert_item_maybe_eject_with_policy(CacheItem::new(key, value), self.policy_compares.get(policy).unwrap());

            self.insert_into_policy_cache(
                maybe_cache_item,
                policy
            )
        }
    }

    /// Given a cache item and a policy, insert into the given policy cache
    fn insert_into_policy_cache(&mut self, maybe_cache_item: Option<Box<dyn CacheItem<T>>>, policy: &str) {
        if let Some(cache_item) = maybe_cache_item {
            let test: &mut &Cache<_> = self.policies.get_mut(&policy).unwrap();
        }
    }

    /// Given a key, find an item in a policy cache if it exists in one
    fn find_key_in_policy_caches(&mut self, key: usize) -> Option<(Box<dyn CacheItem<T>>, f64, &str)> {
        for (&policy_name, mut policy) in &self.policies {
            if let Some(cache_item) = policy.maybe_eject_key(key) {
                return Some(cache_item.into_inner());
            }
        }

        None
    }

    /// Returns a tuple of the current sizes of each cache
    pub fn len(&self) -> Vec<(&'a str, usize)> {
        let mut size_vec = Vec::with_capacity(self.policies.len() + 1);
        size_vec.push(("self", self.cache.len()));
        self.policies.iter().for_each(|(&name, &cache)| {
            size_vec.push((name, cache.len()));
        });

        size_vec
    }
}
