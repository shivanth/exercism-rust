
pub fn reply(spoke : &str) -> String {

    if spoke == ""{
        return "Fine. Be that way!".to_string()
    }

    if spoke.chars().last() == Some('?'){
        return "Sure.".to_string()
    }

    if spoke.to_uppercase() == spoke {
        return "Whoa, chill out!".to_string()
    }


    "Whatever.".to_string()
}
