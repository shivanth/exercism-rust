use std::collections::HashMap;
use std::ascii::AsciiExt;

pub fn score (string: &str) -> i32{
    let mut scores: HashMap<char,i32> = HashMap::new();
    scores.insert('A',1);
    scores.insert('E',1);
    scores.insert('I',1);
    scores.insert('O',1);
    scores.insert('U',1);
    scores.insert('L',1);
    scores.insert('N',1);
    scores.insert('R',1);
    scores.insert('S',1);
    scores.insert('T',1);
    scores.insert('D',2);
    scores.insert('G',2);
    scores.insert('B',3);
    scores.insert('C',3);
    scores.insert('M',3);
    scores.insert('P',3);
    scores.insert('F',4);
    scores.insert('H',4);
    scores.insert('V',4);
    scores.insert('W',4);
    scores.insert('Y',4);
    scores.insert('K',5);
    scores.insert('J',8);
    scores.insert('X',8);
    scores.insert('Q',10);
    scores.insert('Z',10);
    let mut score:i32 = 0;
    for c in string.chars() {
        match scores.get(&c.to_ascii_uppercase()){
            Some(x) =>  score += *x,
            None => println!("Didn't find "),
        }
    }
    score
}
