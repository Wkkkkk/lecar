pub enum AccessMethod {
    READ,
    WRITE
}

impl AccessMethod {
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
    pub fn from_bool(x: bool) -> Self {
        match x {
            true => Self::LFU,
            false => Self::LRU
        }
    }
}
