use std::collections::HashMap;

pub fn count(count_for: char, sequence: &str) -> usize{
    let mut count:usize = 0;
    for c in sequence.chars(){
        if c == count_for{
            count += 1;
        }
    }
    count
}

pub fn nucleotide_counts(sequence:&str) -> HashMap<char,usize> {
    let mut counts : HashMap<char,usize> = HashMap::new();
    for c in "ACGT".chars(){
        counts.insert(c,count(c,sequence));
    }
    counts

}
