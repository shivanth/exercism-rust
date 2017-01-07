use std::collections::HashMap;

pub fn word_count(string: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for s in string.split(" ") {
        let not_is_alphanumeric = |c:char| !char::is_alphanumeric(c);

        let word = s.trim_right_matches(not_is_alphanumeric).to_string().to_lowercase();
            if word == ""{
                continue;
            }
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}
