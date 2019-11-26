use std::collections::HashMap;

pub struct School {
    grades : HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v : Vec<u32> = self.grades.keys().cloned().collect();
        v.sort();
        v
    }

    pub fn grade(&self, n : u32) -> Option<&Vec<String>> {
        self.grades.get(&n)
    }

    pub fn add(&mut self, n : u32, s : &str) {
        let v = self.grades.entry(n).or_insert(vec![]);
        v.push(s.to_string());
        v.sort();
    }
}