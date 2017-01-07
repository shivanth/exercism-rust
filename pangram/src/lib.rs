pub fn is_pangram(sentence :&str)->bool{
    for c in "abcdefghijklmnopqrstuvwxyz".chars(){
        if !sentence.to_lowercase().contains(c){
            return false;
        }
    }
    true
}
