
use std::collections::BTreeMap;


macro_rules! btreemap {
    ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::BTreeMap::new();
    $( map.insert($key, $val); )*
        map
}}
}




pub struct Roman {
    val: i32,
}

impl Roman {
    pub fn from(n: i32) -> Roman {
        Roman { val: n }
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        let mut roman: String = String::new();
        let map: BTreeMap<i32,&str> = btreemap![1000 => "M", 900 => "CM",
                  500 => "D", 400 => "CD",
                  100 => "C", 90 => "XC",
                  50 => "L",  40 => "XL",
                  10 => "X",  9 => "IX",
                  5 => "V",  4 => "IV",
                  1 => "I"];
        let mut temp = self.val;
        for (k, v) in map.iter().rev() {

            while temp >= *k {
                temp -= *k;
                roman.push_str(v);
            }
        }
        roman
    }
}
