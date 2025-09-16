pub fn square(s: u32) -> u64 {
    let number_of_grains_at_square= 2u64.pow(s-1);
    return number_of_grains_at_square;
}

pub fn total() -> u64 {
     return u64::max_value() ;
}
