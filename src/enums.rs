/// Represents how the cache is being accessed
pub enum AccessMethod {
    READ,
    WRITE
}

impl AccessMethod {
    #[cfg(test)]
    pub fn from_bool(x: bool) -> Self {
        match x {
            true => Self::READ,
            false => Self::WRITE
        }
    }
}

#[derive(Copy, Clone)]
pub enum Policy {
    LFU,
    LRU
}

impl Policy {
    /// TODO: Will change when the ML learner is implemented
    pub fn from_bool(x: bool) -> Self {
        match x {
            true => Self::LFU,
            false => Self::LRU
        }
    }
}
