use probalistic_data::convert_to_bv; 
fn main() {
    let bv = convert_to_bv(9000000000000000000); 
        
    let mut idx_bv = bv.clone(); 
    idx_bv.truncate(8 as usize); 
    println!("{:?}", idx_bv); 
    println!("{}", idx_bv.to_bytes().len()); 
    let mut be_arr = [0; 8]; 
    let be_vec = idx_bv.to_bytes(); 
    for i in 0..be_vec.len() {
        be_arr[7-i] = be_vec[be_vec.len()-1-i]; 
    }
    let idx = u64::from_be_bytes(be_arr); 
    println!("{}", idx); 
}
