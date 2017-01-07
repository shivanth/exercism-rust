use std::collections::HashMap;
pub fn encode(plaintext: &str) -> String {
    let mut ciphertext: String = String::new();
    let mut map: HashMap<char, char> = HashMap::new();

    for (i, j) in "abcdefghijklmnopqrstuvwxyz123456789"
        .chars()
        .zip("987654321abcdefghijklmnopqrstuvwxyz".chars().rev()) {
        map.insert(i, j);
    }
    let mut count = 0;
    for s in plaintext.to_string().to_lowercase().chars() {
        match map.get(&s) {
            Some(x) => {
                ciphertext.push(*x);
                count += 1;
            }
            None => (),
        };
        if count == 5 {
            ciphertext.push_str(" ");
            count = 0;

        }
    }

    if ciphertext.chars().last().unwrap() == ' '{
        ciphertext.pop();
    }

    ciphertext

}

pub fn decode(plaintext: &str) -> String {
    let mut ciphertext: String = String::new();
    let mut map: HashMap<char, char> = HashMap::new();

    for (i, j) in "abcdefghijklmnopqrstuvwxyz123456789"
        .chars()
        .zip("987654321abcdefghijklmnopqrstuvwxyz".chars().rev()) {
        map.insert(i, j);
    }
    for s in plaintext.chars() {
        match map.get(&s) {
            Some(x) => ciphertext.push(*x),
            None => (),
        };

    }
    ciphertext
}
