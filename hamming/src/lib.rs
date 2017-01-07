pub fn hamming_distance(stringa: &str, stringb : &str) -> Result<i32,&'static str>{
    if stringa.len()!=stringb.len(){
        return Err("")
    }
    let mut distance :i32 = 0;
    for (ai,bi) in stringa.chars().zip(stringb.chars()){
        if ai != bi{
            distance +=1;
        }
    }
    Ok(distance)
}
