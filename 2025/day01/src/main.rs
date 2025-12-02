// 2025 - Day 1: Secret Entrance
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut puzzle = Puzzle { data: vec![] };
    puzzle.load("2025/day01/input.txt");

    println!("part1={}", puzzle.part1());
    println!("part2={}", puzzle.part2());
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

fn next_l(current: i32, v: i32) -> (i32, i32) {
    // new position (handles negative correctly)
    let new_pos = (current - v).rem_euclid(100);

    // count how many times the dial points at 0 during the left rotation
    // For current == 0, hits are floor(v / 100).
    // Otherwise, if v < current: no hit. If v >= current: floor((v - current) / 100) + 1
    let nb_clicks = if current == 0 {
        v / 100
    } else if v < current {
        0
    } else {
        (v - current) / 100 + 1
    };

    (new_pos, nb_clicks)
}

fn next_r(current: i32, v: i32) -> (i32, i32) {
    ((current + v) % 100, (current + v) / 100)
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
                current = next_l(current, d.value).0;
            } else {
                current = next_r(current, d.value).0;
            }
            if current == 0 {
                nb_zeros += 1;
            }
        }
        nb_zeros
    }
    fn part2(&self) -> i32 {
        let mut current = 50;
        let mut nb_zeros = 0;
        for d in &self.data {
            if d.direction == 'L' {
                let n = next_l(current, d.value);
                current = n.0;
                nb_zeros += n.1;
            } else {
                let n = next_r(current, d.value);
                current = n.0;
                nb_zeros += n.1;
            }
        }
        nb_zeros
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_next_func_example_p1() {
        // The dial starts by pointing at 50.
        assert_eq!(next_l(50, 68).0, 82); // The dial is rotated L68 to point at 82.
        assert_eq!(next_l(82, 30).0, 52); // The dial is rotated L30 to point at 52.
        assert_eq!(next_r(52, 48).0, 0); // The dial is rotated R48 to point at 0.
        assert_eq!(next_l(0, 5).0, 95); // The dial is rotated L5 to point at 95.
        assert_eq!(next_r(95, 60).0, 55); // The dial is rotated R60 to point at 55.
        assert_eq!(next_l(55, 55).0, 0); // The dial is rotated L55 to point at 0.
        assert_eq!(next_l(0, 1).0, 99); // The dial is rotated L1 to point at 99.
        assert_eq!(next_l(99, 99).0, 0); // The dial is rotated L99 to point at 0.
        assert_eq!(next_r(0, 14).0, 14); // The dial is rotated R14 to point at 14.
        assert_eq!(next_l(14, 82).0, 32); // The dial is rotated L82 to point at 32.
    }

    #[test]
    fn test_next_func_gt100_p1() {
        assert_eq!(next_l(0, 138).0, 62);
        assert_eq!(next_l(0, 38).0, 62);
        assert_eq!(next_l(0, 438).0, 62);
        assert_eq!(next_l(0, 769).0, 31);
    }

    #[test]
    fn test_next_func_example_p2() {
        // The dial starts by pointing at 50.
        assert_eq!(next_l(50, 68).1, 1); // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        assert_eq!(next_l(82, 30).1, 0); // The dial is rotated L30 to point at 52.
        assert_eq!(next_r(52, 48).1, 1); // The dial is rotated R48 to point at 0.
        assert_eq!(next_l(0, 5).1, 0); // The dial is rotated L5 to point at 95.
        assert_eq!(next_r(95, 60).1, 1); // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        assert_eq!(next_l(55, 55).1, 1); // The dial is rotated L55 to point at 0.
        assert_eq!(next_l(0, 1).1, 0); // The dial is rotated L1 to point at 99.
        assert_eq!(next_l(99, 99).1, 1); // The dial is rotated L99 to point at 0.
        assert_eq!(next_r(0, 14).1, 0); // The dial is rotated R14 to point at 14.
        assert_eq!(next_l(14, 82).1, 1); // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
    }

    #[test]
    fn test_next_func_gt100_p2() {
        assert_eq!(next_r(50, 1000), (50, 10));
        assert_eq!(next_l(50, 1000), (50, 10));
    }

    #[test]
    fn test_next_func_p2() {
        assert_eq!(next_r(50, 50), (0, 1));

        assert_eq!(next_l(50, 50), (0, 1));

        assert_eq!(next_l(50, 50).1, 1);
        assert_eq!(next_l(50, 50).0, 0);

        assert_eq!(next_l(0, 1).1, 0);
        assert_eq!(next_l(0, 1).0, 99);

        assert_eq!(next_r(0, 1).1, 0);
        assert_eq!(next_r(0, 1).0, 1);

        assert_eq!(next_r(1, 100), (1, 1));

        assert_eq!(next_l(1, 100), (1, 1));

        assert_eq!(next_r(99, 101), (0, 2));

        assert_eq!(next_l(99, 101), (98, 1));

        assert_eq!(next_r(1, 101), (2, 1));
    }

    #[test]
    fn test_load() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        for d in puzzle.data.iter() {
            println!("{:?}", d);
        }
        assert_eq!(puzzle.data.len(), 10);
    }

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part1(), 3);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part2(), 6);
    }
}
