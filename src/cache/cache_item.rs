use std::time::Instant;

/// Definition of a cache item
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CacheItem {
    frequency: usize,
    last_used: Instant,
    key: usize,
    value: Vec<u8>
}

/// Implementation of a cache item
impl CacheItem {
    /// Instantiates a new cache item given the key and value
    pub fn new(key: usize, value: Vec<u8>) -> Self {
        Self {
            frequency: 0,
            last_used: Instant::now(),
            key,
            value
        }
    }

    /// Increments the frequency of the item
    /// Updates the last used time to now
    pub fn touch(&mut self) {
        self.frequency += 1;
        self.last_used = Instant::now();
    }

    /// Sets the cache item's value to the one given
    /// Calls the touch method
    pub fn update(&mut self, value: Vec<u8>) {
        self.value = value;
        self.touch();
    }

    /// Getter for frequency
    pub fn frequency(&self) -> usize {
        self.frequency
    }

    /// Getter for last_used
    pub fn last_used(&self) -> Instant {
        self.last_used
    }

    /// Getter for key
    pub fn key(&self) -> usize {
        self.key
    }

    /// Getter for value
    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }

    /// Getter for value
    /// Consumes self
    /// returns actual value vec
    pub fn value_owned(self) -> Vec<u8> {
        self.value
    }
}
