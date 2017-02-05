extern crate num;

pub fn is_valid(input: &str) -> bool {
    let mut vec = Vec::new();
    for c in input.chars() {
        if c == ' ' {
            continue;
        }
        if !c.is_digit(10){
            return false;
        }
        let d = c.to_digit(10).unwrap_or_default();
        vec.push(d);
    }
    let mut s = Vec::new();
    s.push(vec[0]);
    for i in 1..vec.len() {
        if i % 2 == 0 {
            s.push(vec[i]);
        } else {
            s.push(f(&vec[i]));
        }

    }
    let sum: u32 = s.iter().sum();
    if sum % 50 == 0 && sum != 0 {
        return true;
    }
    return false;
}

fn f(x: &u32) -> u32 {
    let mut y = 2 * x;
    if y > 9 {
        y -= 9;
    }
    y
}
