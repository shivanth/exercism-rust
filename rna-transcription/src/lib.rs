use std::collections::HashMap;
#[derive(PartialEq, PartialOrd,Debug)]
pub struct RibonucleicAcid {
    string: String,
}

impl RibonucleicAcid {
    pub fn new(name: &str) -> RibonucleicAcid {
        RibonucleicAcid { string: name.to_string() }
    }
}


pub struct DeoxyribonucleicAcid {
    string: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(name: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { string: name.to_string() }
    }

    pub fn to_rna(self) -> RibonucleicAcid {
        let mut complement: HashMap<char, char> = HashMap::new();
        for (c, v) in "GCTA".chars().zip("CGAU".chars()) {
            complement.insert(c, v);
        }
        let mut compl = String::new();
        for s in self.string.chars() {
            let d = complement.get(&s).unwrap();
            compl.push(*d);
        }
        RibonucleicAcid::new(&compl)
    }
}
