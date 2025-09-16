pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    let mut is_leap:bool = false;
    if year%4==0{
        is_leap= true;
        if year % 100 == 0{
          is_leap=false;
            if year% 400==0{
            is_leap=true;
        }  
        } 
    }    
    return is_leap;
}
