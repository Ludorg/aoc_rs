use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Vec<Range>>,
    maps_name: Vec<String>,
    // seed_to_soil: Vec<Range>,
    // soil_to_fertilizer: Vec<Range>, // ...
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: vec![],
            maps: vec![],
            maps_name: vec![],
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
            } else {
                let map_kw = item.find(" map:");
                if map_kw.is_some() {
                    let name = item[0..map_kw.unwrap()].to_string();
                    self.maps_name.push(name);
                    //println!("[{&name}]/pos={maps_counter}");
                    maps_counter += 1;
                }
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

impl Range {
    fn new(s: &str) -> Self {
        Range {
            destination_start: 1,
            source_start: 2,
            length: 3,
        }
    }
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

    #[test]
    fn test_load_map_names() {
        let mut a = Almanac::new();
        a.load("example.txt");
        assert_eq!(a.maps_name.len(), 7); // 7 maps
        assert_eq!(a.maps_name.len(), 7);
        assert_eq!(a.maps_name.len(), 7);
        assert_eq!(a.maps_name[0], "seed-to-soil");
        assert_eq!(a.maps_name[1], "soil-to-fertilizer");
        assert_eq!(a.maps_name[6], "humidity-to-location");
    }

    #[test]
    fn test_range_new() {
        let r = Range::new("50 98 2");
        // The first line has a destination range start of 50, a source range start of 98, and a range length of 2
        assert_eq!(r.destination_start, 50);
        assert_eq!(r.source_start, 98);
        assert_eq!(r.length, 2);
        
    }
}

fn main() {
    let mut a = Almanac::new();
    a.load("2023/day05/input.txt");
}
