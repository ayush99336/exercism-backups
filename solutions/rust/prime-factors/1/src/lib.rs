pub fn factors(n: u64) -> Vec<u64> {
    let mut n=n;
    let mut ans= Vec::new();
    while n%2==0{
        ans.push(2);
        n=n/2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            ans.push(i);
            n /= i;
        }
        i += 2;
    }
        if n > 2 {
        ans.push(n);
    }
    ans
}
