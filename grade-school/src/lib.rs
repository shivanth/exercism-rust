use std::collections::HashMap;
pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { students: HashMap::new() }
    }
    pub fn grades(&self) -> Vec<u32> {
        let mut ret = self.students.keys().cloned().collect::<Vec<u32>>();
        ret.sort();
        ret

    }
    pub fn add(&mut self, grade:u32,name:&str){
        let mut res = self.students.entry(grade).or_insert(Vec::new());
        res.push(name.to_owned());
        res.sort()
    }

    pub fn grade(&self, grade:u32) ->  Option<&Vec<String>> {
        self.students.get(&grade)
    }
}
