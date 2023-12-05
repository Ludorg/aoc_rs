use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<u32, Vec<Range>>,
    maps_name: HashMap<u32, Vec<String>>,
    // seed_to_soil: Vec<Range>,
    // soil_to_fertilizer: Vec<Range>, // ...
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: vec![],
            maps: HashMap::new(),
            maps_name: HashMap::new(),
            // seed_to_soil: vec![],
            // soil_to_fertilizer: vec![],
        }
    }
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut maps_counter = 0;
        for (index, line) in reader.lines().enumerate() {
            let item = line.unwrap();
            if index == 0 {
                self.load_seeds(&item);
            }
        }
    }
    fn load_seeds(&mut self, s: &str) {
        let idx = s.find("seeds:").unwrap() + "seeds:".len() + 1;
        let v = s[idx..].split(' ');
        for n in v {
            if !n.trim().is_empty() {
                println!("seed={n}");
                let val: u32 = n.parse::<u32>().unwrap();
                self.seeds.push(val);
            }
        }
    }
}

struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_seeds() {
        let mut a = Almanac::new();
        a.load("example.txt");
        assert_eq!(a.seeds.len(), 4); // 4 seeds in file example.txt
        assert_eq!(a.seeds[0], 79);
        assert_eq!(a.seeds[1], 14);
        assert_eq!(a.seeds[3], 13);
    }
}

fn main() {
    println!("Hello, world!");
}
