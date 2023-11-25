pub mod bloom;

use crate::bloom::BloomFilter; 

fn main() {
    let mut hash_fns: Vec<fn(&u32) -> usize> = vec![];
    hash_fns.push(hash);  
    hash_fns.push(hash2); 
    let mut filter: BloomFilter<u32> = bloom::BloomFilter::with_size(20, hash_fns); 

    let vals = vec![1, 3, 5]; 
    for val in &vals {
        filter.insert(*val); 
    }

    let universe = 10; 

    for val in 0..universe+1 {
        println!("{}", filter.contains(val)); 
    }
}

fn hash(num: &u32) -> usize {
    *num as usize
}
fn hash2(num: &u32) -> usize {
    ((*num) * 7) as usize
}