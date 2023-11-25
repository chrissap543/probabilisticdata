extern crate probalistic_data;

use probalistic_data::loglog::*;

fn main() {
    let loglog = loglog::new(8);
    println!("loglog size: {}", loglog.size());
}
