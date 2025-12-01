use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut puzzle = Puzzle { data: vec![] };
    puzzle.load("2025/day01/input.txt");
    println!("part1={}", puzzle.part1());
}
#[derive(Debug)]
struct Data {
    direction: char,
    value: i32,
}

#[derive(Debug)]

struct Puzzle {
    data: Vec<Data>,
}

fn next_l(current: i32, v: i32) -> i32 {
    let mut diff = current - v;
    while diff < 0 {
        diff = 100 + diff;
    }
    diff % 100
}

fn next_r(current: i32, v: i32) -> i32 {
    (current + v) % 100
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for l in reader.lines() {
            let d = Data {
                direction: l.as_ref().unwrap().chars().next().unwrap(),
                value: l.as_ref().unwrap()[1..].trim().parse::<i32>().unwrap(),
            };
            self.data.push(d);
        }
    }
    fn part1(&self) -> i32 {
        let mut current = 50;
        let mut nb_zeros = 0;
        for d in &self.data {
            if d.direction == 'L' {
                current = next_l(current, d.value);
            } else {
                current = next_r(current, d.value);
            }
            if current == 0 {
                nb_zeros += 1;
            }
        }
        nb_zeros
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_next_func_example() {
        // The dial starts by pointing at 50.
        assert_eq!(next_l(50, 68), 82); // The dial is rotated L68 to point at 82.
        assert_eq!(next_l(82, 30), 52); // The dial is rotated L30 to point at 52.
        assert_eq!(next_r(52, 48), 0); // The dial is rotated R48 to point at 0.
        assert_eq!(next_l(0, 5), 95); // The dial is rotated L5 to point at 95.
        assert_eq!(next_r(95, 60), 55); // The dial is rotated R60 to point at 55.
        assert_eq!(next_l(55, 55), 0); // The dial is rotated L55 to point at 0.
        assert_eq!(next_l(0, 1), 99); // The dial is rotated L1 to point at 99.
        assert_eq!(next_l(99, 99), 0); // The dial is rotated L99 to point at 0.
        assert_eq!(next_r(0, 14), 14); // The dial is rotated R14 to point at 14.
        assert_eq!(next_l(14, 82), 32); // The dial is rotated L82 to point at 32.
    }

    #[test]
    fn test_next_func_gt100() {
        assert_eq!(next_l(0, 138), 62);
        assert_eq!(next_l(0, 38), 62);
        assert_eq!(next_l(0, 438), 62);
        assert_eq!(next_l(0, 769), 31);
    }

    #[test]
    fn test_load() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        for d in puzzle.data.iter() {
            println!("{:?}", d);
        }
    }

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part1(), 3);
    }
}
