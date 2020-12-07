use std::cmp::Ordering;
use crate::cache::{ICacheItemWrapper, CacheItem};

#[derive(Eq, Clone, Debug)]
pub struct LRUCacheItem(CacheItem);

impl LRUCacheItem {
    pub fn new(cache_item: CacheItem) -> Self {
        Self(cache_item)
    }
}

impl PartialEq for LRUCacheItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.last_used().eq(&other.0.last_used())
    }
}

impl Ord for LRUCacheItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.last_used().cmp(&self.0.last_used())
    }
}

impl PartialOrd for LRUCacheItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ICacheItemWrapper for LRUCacheItem {
    fn get_inner_key(&self) -> usize {
        self.0.key()
    }

    fn into_inner(self) -> CacheItem {
        self.0
    }
}
