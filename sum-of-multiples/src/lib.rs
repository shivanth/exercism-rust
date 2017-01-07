fn divides(n: i32, numbers: &Vec<i32>) -> bool{
    for i in numbers{
        if n % i == 0 {
            return true;
        }
    }
    false
}

pub fn sum_of_multiples(n: i32, numbers: &Vec<i32>) -> i32{
    let mut sum = 0;
    for i in 0..n{
        if divides(i, numbers){
            sum += i}
    }
    sum
}
