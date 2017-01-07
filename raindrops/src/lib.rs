pub fn raindrops(num :i32)-> String {
    let mut result: String ="".to_string();

    if num % 3 == 0 {
        result = result + "Pling";
    }
    if num % 5 == 0 {
        result = result + "Plang";
    }
    if num % 7 == 0 {
        result = result + "Plong";
    }
    if result == ""{
        result = format!("{}",num);
    }
    result
}
