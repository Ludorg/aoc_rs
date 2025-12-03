// 2025 - Day 3: Lobby
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_DIGITS: usize = 12;

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

    fn part2(&self) -> u64 {
        let mut out: u64 = 0;
        for bank in &self.data {
            out += largest_joltage_p2(bank.to_vec());
        }
        out
    }
}

fn largest_joltage_p1(bank: Vec<char>) -> u32 {
    let ten = bank[0..bank.len() - 1].iter().max().unwrap();
    let ten_pos = bank.iter().position(|x| x == ten).unwrap();
    let unit = bank[ten_pos + 1..bank.len()].iter().max().unwrap();

    ten.to_digit(10).unwrap() * 10 + unit.to_digit(10).unwrap()
}

fn digits_to_number(digits: &[u8]) -> u64 {
    let mut number: u64 = 0;
    for &d in digits {
        number = number * 10 + d as u64;
    }
    number
}

fn max_in_bank_digit_n(bank: &Vec<char>, n: usize, start: usize) -> (u8, usize) {
    let len = bank.len();
    if start >= len {
        return (0, len - 1);
    }
    let last_allowed = if MAX_DIGITS >= n {
        len - (MAX_DIGITS - n)
    } else {
        len
    } - 1;

    let mut best_digit: u8 = 0;
    let mut best_pos: usize = start;
    let end = min(last_allowed, len - 1);
    for i in start..=end {
        let d = bank[i].to_digit(10).unwrap() as u8;
        if d > best_digit {
            best_digit = d;
            best_pos = i;
        }
    }
    (best_digit, best_pos)
}

fn largest_joltage_p2(bank: Vec<char>) -> u64 {
    let mut digits: [u8; MAX_DIGITS] = [0; MAX_DIGITS];

    let mut offset: usize = 0;
    for i in 0..MAX_DIGITS {
        let d = max_in_bank_digit_n(&bank, i + 1, offset);
        offset = d.1 + 1;
        digits[i] = d.0;
    }

    digits_to_number(&digits)
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

    #[test]
    fn test_largest_joltage_p2() {
        let chars: Vec<char> = "987654321111111".chars().collect();
        assert_eq!(largest_joltage_p2(chars), 987654321111);

        let chars: Vec<char> = "811111111111119".chars().collect();
        assert_eq!(largest_joltage_p2(chars), 811111111119);

        let chars: Vec<char> = "234234234234278".chars().collect();
        assert_eq!(largest_joltage_p2(chars), 434234234278);

        let chars: Vec<char> = "818181911112111".chars().collect();
        assert_eq!(largest_joltage_p2(chars), 888911112111);
    }

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part2(), 3121910778619);
    }

    #[test]
    fn test_digits_to_number() {
        let digits: Vec<u8> = vec![1, 2, 3, 4, 5];
        assert_eq!(digits_to_number(&digits), 12345);

        let digits: Vec<u8> = vec![0, 0, 1, 2];
        assert_eq!(digits_to_number(&digits), 12);
    }
}
