extern crate probalistic_data;

use probalistic_data::bloom::*;
fn main() {
    let mut filter = BloomFilter::with_size(64, vec![hash, hash2, hash3]);
    let vals = vec![25, 2, 37, 4, 3, 7, 12];
    for val in &vals {
        filter.insert(*val);
    }

    let universe = 1..101;
    let (mut tp, mut fp, mut tn, mut r#fn) = (0, 0, 0, 0);
    for num in universe {
        let contained = filter.contains(num);

        if contained {
            if vals.contains(&num) {
                tp += 1;
            } else {
                fp += 1;
            }
        } else {
            if vals.contains(&num) {
                r#fn += 1;
            } else {
                tn += 1;
            }
        }
    }

    println!("True positives: {}", tp);
    println!("False positives: {}", fp);
    println!("True negatives: {}", tn);
    println!("False negatives: {}", r#fn);
    println!("False positive rate: {}", (fp as f32) / ((fp + tn) as f32));
}
fn hash(num: &u32) -> usize {
    *num as usize
}
fn hash2(num: &u32) -> usize {
    ((*num) * 7) as usize
}
fn hash3(num: &u32) -> usize {
    ((*num) * 11 % 5144) as usize
}
