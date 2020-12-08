use std::cmp::Ordering;
use crate::cache::{ICacheItemWrapper, CacheItem};

/// Wrapper struct for CacheItem to implement different PartialEq, PartialOrd, and Ord
#[derive(Eq, Clone, Debug)]
pub struct LRUCacheItem(CacheItem);

/// Implementation of the LRUCacheItem
impl LRUCacheItem {
    /// Returns a wrapped CacheItem
    pub fn new(cache_item: CacheItem) -> Self {
        Self(cache_item)
    }
}

/// Implements PartialEq for LRU use
impl PartialEq for LRUCacheItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.last_used().eq(&other.0.last_used())
    }
}

/// Implements Ord for LRU use
impl Ord for LRUCacheItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.last_used().cmp(&self.0.last_used())
    }
}

/// Implements PartialOrd for LRU use
impl PartialOrd for LRUCacheItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements ICacheItemWrapper for LRU use
impl ICacheItemWrapper for LRUCacheItem {
    fn get_inner_key(&self) -> usize {
        self.0.key()
    }

    fn into_inner(self) -> CacheItem {
        self.0
    }
}
