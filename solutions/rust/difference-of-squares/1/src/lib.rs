pub fn square_of_sum(n: u32) -> u64 {
    // todo!("square of sum of 1...{n}")
    let mut x :u64= (n as u64 *(n as u64 +1))/2;
    println!("{}" ,x.pow(3));
    return x.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    let mut x  = (n*(n+1)*(2*n+1))/6;
    return x;
}

pub fn difference(n: u32) -> u64 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    return (square_of_sum(n) as u64-sum_of_squares(n) as u64);
    
}
