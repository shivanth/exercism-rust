extern crate regex;
use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let mut abbrev = String::new();
    let re = Regex::new(r"(?P<s>[a-z])(?P<b>[A-Z]+)").unwrap();
    let phrase2 =&re.replace(phrase,"$s $b").into_owned();
    println!("{}", phrase2);
    let c = |c| !char::is_alphanumeric(c);
    for s in phrase2.to_uppercase().split(c) {
        match s.chars().next() {
            Some(d) => abbrev.push(d),
            None => (),
        }

    }
    abbrev
}
