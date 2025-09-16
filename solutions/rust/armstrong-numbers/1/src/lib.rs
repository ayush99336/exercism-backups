pub 
fn is_armstrong_number(num: u32) -> bool {
    let mut x: u64 = 0;
    let mut y: u32 = num;
    let num_digits: u32 = num.to_string().len() as u32;
    loop {
        if y == 0 {
            break;
        }
        x += (y % 10).pow(num_digits) as u64;
        y = y / 10;
    }
    if x == num as u64 {
        return true;
    } else {
        return false;
    }
}
