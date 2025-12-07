use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut puzzle = Puzzle {
        numbers: vec![],
        operators: vec![],
    };
    puzzle.load("2025/day06/input.txt");

    println!("part1={}", puzzle.part1());
    println!("part2={}", puzzle.part2());
}

#[derive(Debug)]
struct Puzzle {
    numbers: Vec<Vec<i32>>,
    operators: Vec<char>,
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect("unable to open file");
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = vec![];
        for l in reader.lines() {
            let line = l.expect("read error");
            let s = line.trim();
            if s.is_empty() {
                continue;
            }
            lines.push(s.to_string());
        }

        if lines.is_empty() {
            self.numbers.clear();
            self.operators.clear();
            return;
        }

        // last line = operators
        let ops_line = lines.pop().unwrap();
        self.operators = ops_line
            .split_whitespace()
            .filter_map(|tok| tok.chars().next())
            .collect();

        // other lines = numbers
        self.numbers = lines
            .into_iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|tok| tok.parse::<i32>().expect("invalid integer"))
                    .collect()
            })
            .collect();
    }

    fn part1(&self) -> u64 {
        let mut out: u64 = 0;
        for i in 0..self.operators.len() {
            let op = self.operators[i];
            //println!("op={} ", op);
            let mut total: u64 = 0;
            match op {
                '+' => total = 0,
                '*' => total = 1,
                _ => panic!("unknown operator {}", op),
            }
            for j in 0..self.numbers.len() {
                let val = self.numbers[j][i];
                //println!("val={} ", val);
                match op {
                    '+' => total += val as u64,
                    '*' => total *= val as u64,
                    _ => panic!("unknown operator {}", op),
                }
            }

            out += total;
            //println!("total={} ", total);
        }
        out
    }

    fn part2(&self) -> u64 {
        0
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle {
            numbers: vec![],
            operators: vec![],
        };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part1(), 4277556);
    }

    #[test]
    fn test_load() {
        let mut puzzle = Puzzle {
            numbers: vec![],
            operators: vec![],
        };
        puzzle.load("test.txt");
        assert_eq!(puzzle.numbers[0].len(), 4);
        assert_eq!(puzzle.numbers.len(), 3);
        assert_eq!(puzzle.operators.len(), 4);
        assert_eq!(puzzle.numbers[0][0], 123);
        assert_eq!(puzzle.numbers[1][0], 45);
        assert_eq!(puzzle.numbers[2][0], 6);
        assert_eq!(puzzle.operators[0], '*');

        assert_eq!(puzzle.numbers[0][1], 328);
        assert_eq!(puzzle.numbers[1][1], 64);
        assert_eq!(puzzle.numbers[2][1], 98);
        assert_eq!(puzzle.operators[1], '+');
    }
}
