use crate::controller::Controller;
use rand::prelude::*;

/// Represents how the cache is being accessed
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

#[test]
fn test_controller() {
    const CACHE_SIZE: usize = 200;
    const LFU_CACHE_SIZE: usize = 20;
    const LRU_CACHE_SIZE: usize = 20;

    let mut cache_controller = Controller::new(CACHE_SIZE, LFU_CACHE_SIZE, LRU_CACHE_SIZE);

    let mut rng = rand::thread_rng();

    for _ in 0..rng.gen_range(1_000, 2_000) {
        let key = rng.gen_range(0, 300);
        let key = key.to_string();

        match AccessMethod::from_bool(rng.gen_bool(0.10)) {
            AccessMethod::READ => {
                cache_controller.get(&key);
            },
            AccessMethod::WRITE => {
                let data = "abcdefg";
                cache_controller.insert(&key, data.to_string());
            }
        }
    }

    let lengths = cache_controller.len();
    assert_eq!(lengths.0, CACHE_SIZE);
    // TODO: Find better assertions for probabilistic outcomes
    // assert_eq!(lengths.1, LFU_CACHE_SIZE);
    // assert_eq!(lengths.2, LRU_CACHE_SIZE);
}
