use bit_vec::BitVec;

extern crate bit_vec;

pub struct BloomFilter<T> {
    bits: BitVec,
    hash_fns: Vec<fn(&T) -> usize>,
}

impl<T> BloomFilter<T> {
    pub fn with_size(size: usize, hash_fns: Vec<fn(&T) -> usize>) -> BloomFilter<T> {
        BloomFilter {
            bits: BitVec::from_elem(size, false),
            hash_fns: hash_fns,
        }
    }

    pub fn insert(&mut self, val: T) {
        for hash_fn in &self.hash_fns {
            let idx = hash_fn(&val) % self.bits.len(); 
            match self.bits.get(idx) {
                Some(_) => (), 
                None => panic!("Hash out of bounds in insert"),
            }
            self.bits.set(idx, true); 
        }
    }

    pub fn contains(&self, val: T) -> bool {
        let mut is_in = true; 
        for hash_fn in &self.hash_fns {
            let idx = hash_fn(&val) % self.bits.len(); 
            match self.bits.get(idx) {
                Some(x) => {
                    if !x {
                        is_in = false; 
                        break; 
                    }
                }, 
                None => panic!("Hash out of bounds in contains"),
            }
        }
        is_in
    }
}
