extern crate bit_vec;

use std::{
    collections::hash_map::DefaultHasher,
    default,
    hash::{Hash, Hasher},
};

pub struct loglog {
    buckets: Vec<u8>,
    bits: u32,
}

impl loglog {
    pub fn new(size: u32) -> loglog {
        loglog {
            buckets: vec![0; (1 << size) - 1],
            bits: size,
        }
    }

    pub fn size(self) -> usize {
        self.buckets.len()
    }

    pub fn insert<T: Hash>(&mut self, val: &T) {
        let mut hasher = DefaultHasher::new();
    }
}
