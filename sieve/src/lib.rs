
pub fn primes_up_to (n: i32) -> Vec<i32> {
    let mut sieve: Vec<i32> = vec![0; (n + 1) as usize];
    let mut primes: Vec<i32> = Vec::new();
    for i in 2..(n + 1) {
        if sieve[(i as usize)] == 0 {
            primes.push(i);
            let mut j = 1;

            while j * i <= n {
                sieve[(j*i) as usize] = 1;
                j += 1;
            }
        }
    }
    //println!("{:?}",sieve);
    primes
}
