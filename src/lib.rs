use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

use bit_vec::BitVec; 

pub mod bloom;
pub mod loglog;

pub fn power_of_two(x: u64) -> bool {
    (x & (x - 1)) == 0
}

fn calculate_hash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new(); 
    val.hash(&mut hasher); 
    hasher.finish()
}

pub fn convert_to_bv(val: u64) -> BitVec {
    let mut slices: Vec<u8> = vec![];
    for i in 0..8 {
        let tmp = (val << (i * 8) >> (i * 8)) >> ((7-i) * 8); 
        slices.push(match tmp.try_into() {
            Ok(x) => x, 
            Err(e) => panic!("{}: {}", i, e)
        });
    } 

    BitVec::from_bytes(&slices)
}