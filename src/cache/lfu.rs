use std::cmp::Ordering;
use crate::cache::{ICacheItemWrapper, CacheItem};

/// Wrapper struct for CacheItem to implement different PartialEq, PartialOrd, and Ord
#[derive(Eq, Clone, Debug)]
pub struct LFUCacheItem(CacheItem);

/// Implementation of the LFUCacheItem
impl LFUCacheItem {
    /// Returns a wrapped CacheItem
    pub fn new(cache_item: CacheItem) -> Self {
        Self(cache_item)
    }
}

/// Implements PartialEq for LFU use
impl PartialEq for LFUCacheItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.frequency() == other.0.frequency()
    }
}

/// Implements Ord for LFU use
impl Ord for LFUCacheItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.frequency().cmp(&self.0.frequency())
    }
}

/// Implements PartialOrd for LFU use
impl PartialOrd for LFUCacheItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ICacheItemWrapper for LFUCacheItem {
    fn get_inner_key(&self) -> usize {
        self.0.key()
    }

    fn into_inner(self) -> CacheItem {
        self.0
    }
}
