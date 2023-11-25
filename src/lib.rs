pub mod bloom;
pub mod loglog;

pub fn power_of_two(x: u64) -> bool {
    (x & (x - 1)) == 0
}
