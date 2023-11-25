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

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    fn hash(num: &u32) -> usize {
        *num as usize
    }
    fn hash2(num: &u32) -> usize {
        ((*num) * 7) as usize
    }
    fn hash3(num: &u32) -> usize {
        ((*num) * 11 % 5144) as usize
    }

    #[test]
    fn insert() {
        let mut filter = BloomFilter::with_size(32, vec![hash, hash2, hash3]); 
        filter.insert(5); 

        assert_eq!(filter.bits.get(5), Some(true)); 
        assert_eq!(filter.bits.get(3), Some(true)); 
        assert_eq!(filter.bits.get(23), Some(true)); 
    }

    #[test]
    fn contains() {
        let mut filter = BloomFilter::with_size(32, vec![hash, hash2, hash3]); 
        let vals = [5, 25, 33, 44]; 
        
        for val in &vals {
            filter.insert(*val); 
        }

        for val in &vals {
            assert!(filter.contains(*val)); 
        }
    }
}