
pub fn verse(n : i32)->String{
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }

    else if n ==1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    else if n ==2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n , n - 1)
}

    else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n , n - 1)}
}


pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for i in (end..start+1).rev(){
        //println!("{}\n", i);
        song.push_str(&verse(i));
        song.push_str("\n")
    }
    song.pop();
    song
}
