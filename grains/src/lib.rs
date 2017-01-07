//use std::i64;
//use std::num::Int;

pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64{
        panic!("Square must be between 1 and 64")
    }
    let x :u64 =2;
    x.pow((s - 1)) as u64
}

pub fn total() -> u64 {
    let mut sum:u64 = 0;
    for i in 1..65{
        sum += square(i)
    }
    sum
}
