use std::collections::BTreeMap;

pub fn transform(map: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut rtn : BTreeMap<String,i32> = BTreeMap::new();
    for (count,vector) in map{
        for c in vector{
            rtn.insert(c.to_lowercase(), *count);
        }
    }
    rtn
}
