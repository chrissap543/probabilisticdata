extern crate bit_vec;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bit_vec::BitVec;

use crate::{calculate_hash, convert_to_bv};

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

    pub fn size(&self) -> usize {
        self.buckets.len()
    }

    pub fn insert<T: Hash>(&mut self, val: &T) {
        let hashed = calculate_hash(val); 
        let bv = convert_to_bv(hashed); 
        let idx = self.get_idx(bv.clone()); 
        
        let mut count = 0; 
        let mut max_count = 0; 
        println!("{:?}", bv); 
        for b in bv {
            if !b {
                count += 1; 
            } else {
                if count > max_count {
                    max_count = count; 
                }
                count = 0; 
            }
        }
        
        self.buckets[idx as usize] = count; 
    }

    fn get_idx(&self, mut bv: BitVec) -> u64 {
        bv.truncate(self.bits as usize); 
        let mut be_arr = [0; 8]; 
        let be_vec = bv.to_bytes(); 
        for i in 0..be_vec.len() {
            be_arr[7-i] = be_vec[be_vec.len()-1-i]; 
        }
        u64::from_be_bytes(be_arr) 
    }
}