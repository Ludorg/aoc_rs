// 2025 - Day 2: Gift Shop
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut puzzle = Puzzle { data: vec![] };
    puzzle.load("2025/day02/input.txt");

    println!("part1={}", puzzle.part1());
    println!("part2={}", puzzle.part2());
}

#[derive(Debug)]
struct Data {
    first: u64,
    last: u64,
}

#[derive(Debug)]

struct Puzzle {
    data: Vec<Data>,
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for l in reader.lines() {
            let line = l.as_ref().unwrap();

            for token in line.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                let parts: Vec<&str> = token.split('-').map(str::trim).collect();
                if parts.len() == 2 {
                    let first = parts[0].parse::<u64>().unwrap();
                    let last = parts[1].parse::<u64>().unwrap();
                    let d = Data { first, last };
                    self.data.push(d);
                }
            }
        }
    }
    fn part1(&self) -> u64 {
        let mut out = 0;
        for d in &self.data {
            for n in d.first..=d.last {
                if is_repeated_twice(n) {
                    out += n;
                }
            }
        }
        out
    }
    fn part2(&self) -> u64 {
        let mut out: u64 = 0;
        for d in &self.data {
            for n in d.first..=d.last {
                if is_repeated_any(n) {
                    out += n;
                }
            }
        }
        out
    }
}

fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 || len == 0 {
        return false;
    }
    let half = len / 2;
    &s[..half] == &s[half..]
}

fn is_repeated_any(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len < 2 {
        return false;
    }
    // try every possible block length h that divides len
    // k = len/h >= 2
    for h in 1..=(len / 2) {
        if len % h != 0 {
            continue;
        }
        let k = len / h;
        if k < 2 {
            continue;
        }
        // no leading-zero blocks allowed
        if (&s[..h]).starts_with('0') {
            continue;
        }

        let chunk = &s[..h];
        // build repeated string by checking subsequent chunks
        let mut ok = true;
        for i in 1..k {
            if &s[i * h..(i + 1) * h] != chunk {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.data.len(), 11);
        assert_eq!(puzzle.part1(), 1227775554);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part2(), 4174379265);
    }

    #[test]
    fn test_is_repeated_twice() {
        assert!(is_repeated_twice(11));
        assert!(is_repeated_twice(22));
        assert!(is_repeated_twice(99));
        assert!(is_repeated_twice(1010));
        assert!(is_repeated_twice(1188511885));
        assert!(is_repeated_twice(222222));
        assert!(is_repeated_twice(38593859));
    }

    #[test]
    fn test_is_repeated_any() {
        assert!(is_repeated_any(12341234));

        assert!(is_repeated_any(123123123));
        assert!(is_repeated_any(1212121212));
        assert!(is_repeated_any(1111111));

        assert!(is_repeated_any(11));
        assert!(is_repeated_any(22));

        assert!(is_repeated_any(99));
        assert!(is_repeated_any(111));

        assert!(is_repeated_any(999));
        assert!(is_repeated_any(1010));

        assert!(is_repeated_any(1188511885));
        assert!(is_repeated_any(38593859));

        assert!(is_repeated_any(565656));

        assert!(is_repeated_any(824824824));

        assert!(is_repeated_any(2121212121));
    }
}
