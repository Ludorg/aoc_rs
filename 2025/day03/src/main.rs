// 2025 - Day 3: Lobby
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut puzzle = Puzzle { data: vec![] };
    puzzle.load("2025/day03/input.txt");

    println!("part1={}", puzzle.part1());
    println!("part2={}", puzzle.part2());
}

#[derive(Debug)]
struct Puzzle {
    data: Vec<Vec<char>>,
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for l in reader.lines() {
            let line = l.as_ref().unwrap();
            let chars: Vec<char> = line.chars().collect();
            self.data.push(chars);
        }
    }

    fn part1(&self) -> u32 {
        let mut out: u32 = 0;
        for bank in &self.data {
            out += largest_joltage_p1(bank.to_vec());
        }
        out
    }

    fn part2(&self) -> u32 {
        0
    }
}

fn largest_joltage_p1(bank: Vec<char>) -> u32 {
    let ten = bank[0..bank.len() - 1].iter().max().unwrap();
    let ten_pos = bank.iter().position(|x| x == ten).unwrap();
    let unit = bank[ten_pos + 1..bank.len()].iter().max().unwrap();

    ten.to_digit(10).unwrap() * 10 + unit.to_digit(10).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.data.len(), 4);
    }

    #[test]
    fn test_largest_joltage_p1() {
        let chars: Vec<char> = "987654321111111".chars().collect();
        assert_eq!(largest_joltage_p1(chars), 98);

        let chars: Vec<char> = "811111111111119".chars().collect();
        assert_eq!(largest_joltage_p1(chars), 89);

        let chars: Vec<char> = "234234234234278".chars().collect();
        assert_eq!(largest_joltage_p1(chars), 78);

        let chars: Vec<char> = "818181911112111".chars().collect();
        assert_eq!(largest_joltage_p1(chars), 92);
    }

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part1(), 357);
    }
}
