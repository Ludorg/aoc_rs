use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<Range>>,
    maps_name: Vec<String>,
}

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    fn new(s: &str) -> Self {
        let v = s.split(' ');
        let mut rv = [0, 0, 0];
        let mut idx_rv = 0;

        for r in v {
            if !r.trim().is_empty() {
                rv[idx_rv] = r.parse::<u64>().unwrap();
                idx_rv += 1;
            }
        }

        Range {
            destination_start: rv[0],
            source_start: rv[1],
            length: rv[2],
        }
    }
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: vec![],
            maps: vec![],
            maps_name: vec![],
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
                if let Some(sub_idx) = map_kw {
                    maps_counter += 1;
                    let name = item[0..sub_idx].to_string();
                    println!("[{}]/pos={maps_counter}", &name);
                    self.maps_name.push(name);
                    self.maps.push(vec![]);
                } else if !item.is_empty() {
                    self.maps[maps_counter - 1].push(Range::new(&item)); // to start at index 0
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
                let val: u64 = n.parse::<u64>().unwrap();
                self.seeds.push(val);
            }
        }
    }

    fn print(&self) {
        print!("seeds:");
        for s in self.seeds.iter() {
            print!(" {}", &s);
        }
        println!();
        println!();
        let mut idx = 0;
        for m in self.maps.iter() {
            println!("{} map:", self.maps_name[idx]);
            for r in m {
                println!("{} {} {}", r.destination_start, r.source_start, r.length);
            }
            idx += 1;
            if idx < self.maps_name.len() {
                println!();
            }
        }
    }
    fn min_location(&self) -> u64 {
        let mut min = u64::MAX;
        for seed in self.seeds.clone() {
            let mut current = seed;

            'maps: for map in &self.maps {
                for range in map {
                    let range_end = range.source_start + range.length;
                    let range_source = range.source_start;
                    let range_destination = range.destination_start;
                    if current >= range_source && current <= range_end {
                        let offset = current - range_source;
                        current = range_destination + offset;
                        continue 'maps;
                    }
                }
            }
            min = min.min(current);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_location() {
        let mut a = Almanac::new();
        a.load("example.txt");
        assert_eq!(a.min_location(), 35);
    }

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
        assert_eq!(a.maps_name[0], "seed-to-soil");
        assert_eq!(a.maps_name[1], "soil-to-fertilizer");
        assert_eq!(a.maps_name[6], "humidity-to-location");
    }

    #[test]
    fn test_overflow() {
        let a: u32 = 4102106917;
        let b: u32 = 192860379;

        // let c: u32 = a + b;
        // c as u32 causes error
        // attempt to compute `4102106917_u32 + 192860379_u32`, which would overflow
        let c: u64 = (a as u64) + (b as u64);
        println!("{c}");
    }

    #[test]
    fn test_load_map_ranges() {
        let mut a = Almanac::new();
        a.load("example.txt");

        assert_eq!(a.maps.len(), 7);
        assert_eq!(a.maps[0][0].destination_start, 50);
        assert_eq!(a.maps[1][0].destination_start, 0);
        assert_eq!(a.maps[2][0].destination_start, 49);
        assert_eq!(a.maps[2][1].destination_start, 0);
        assert_eq!(a.maps[2][2].destination_start, 42);

        a.print(); // used to check if identical with example file

        println!("seeds {:?}", &a.seeds);
        println!("maps {:?}", &a.maps);
        println!("almanac {:?}", &a);
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

    println!(
        "0/0_dst={}, 0/0_src={}, 0/0_len={}",
        a.maps[0][0].destination_start, a.maps[0][0].source_start, a.maps[0][0].length
    );
    a.print(); // to check if identical with input file

    println!("day5/part1={}", a.min_location());
}
