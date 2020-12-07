use crate::cache::{Cache, CacheItem, ICache};
use crate::controller::Controller;
use crate::enums::AccessMethod;
use rand::prelude::*;

#[test]
fn test_controller() {
    const CACHE_SIZE: usize = 20_000;
    const LFU_CACHE_SIZE: usize = 2_000;
    const LRU_CACHE_SIZE: usize = 2_000;

    let mut cache_controller = Controller::new(CACHE_SIZE, LFU_CACHE_SIZE, LRU_CACHE_SIZE);

    let mut rng = rand::thread_rng();

    for _ in 0..rng.gen_range(100_000, 1_000_000) {
        let key = rng.gen_range(0, 200_000);

        match AccessMethod::from_bool(rng.gen_bool(0.10)) {
            AccessMethod::READ => {
                cache_controller.get(key);
            },
            AccessMethod::WRITE => {
                let mut data = [0u8; 409_600];
                rng.fill_bytes(&mut data);
                cache_controller.insert(key, data.to_vec());
            }
        }
    }

    let lengths = cache_controller.len();
    assert_eq!(lengths.0, CACHE_SIZE);
    assert_eq!(lengths.1, LFU_CACHE_SIZE);
    assert_eq!(lengths.2, LRU_CACHE_SIZE);
}
