//! [Advent of Code 2024 Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    numbers: VecDeque<i64>,
}
#[derive(Debug)]
struct Puzzle {
    data: Vec<Equation>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn compute(o: &Operation, a: &i64, b: &i64) -> i64 {
    match o {
        Operation::Add => a + b,
        Operation::Mul => a * b,
        Operation::Concat => {
            let mut digits = 1;
            let mut temp = *b;
            while temp >= 10 {
                temp /= 10;
                digits *= 10;
            }
            a * (digits * 10) + b
        }
    }
}

impl Puzzle {
    fn load(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        self.data = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            println!("{:?}", parts);
            let t = parts[0].replace(':', " ");

            let test_value: i64 = t.trim().parse().unwrap();
            let mut numbers: VecDeque<i64> = VecDeque::new();
            for p in parts.iter().skip(1) {
                numbers.push_back(p.parse().unwrap());
            }

            self.data.push(Equation {
                test_value,
                numbers,
            });
        }
        Ok(())
    }

    fn new() -> Self {
        Self { data: vec![] }
    }

    fn part1(&self) -> i64 {
        let mut r = 0;
        for e in &self.data {
            if Self::is_equation_valid(e, false) {
                r += e.test_value;
            }
        }
        r
    }

    fn part2(&self) -> i64 {
        let mut r = 0;
        for e in &self.data {
            if Self::is_equation_valid(e, true) {
                r += e.test_value;
            }
        }
        r
    }

    fn is_equation_valid(e: &Equation, part2: bool) -> bool {
        // n is length of numbers in equation
        // there is 2^n-1 combinations to test in part 1
        // 3^n-1 in part2
        let n = e.numbers.len();
        let nb_op = if !part2 { 2_u32 } else { 3_u32 };
        let max = nb_op.pow(n as u32 - 1);
        println!("testing {max} combinations");

        let mut operations: Vec<Vec<Operation>> = vec![];

        for mut i in 0..max {
            let mut c: Vec<Operation> = vec![];
            for _ in 0..e.numbers.len() - 1 {
                if i % nb_op == 0 {
                    // print!("+");
                    c.push(Operation::Add);
                } else if i % nb_op == 1 {
                    // print!("*");
                    c.push(Operation::Mul);
                } else {
                    // print!("|");
                    c.push(Operation::Concat);
                }
                i /= nb_op;
            }
            // println!("");
            operations.push(c);
        }
        // println!("{:?}", operations);

        for ops in operations.iter_mut().take(max as usize) {
            let mut nums = e.numbers.clone();
            let mut res = compute(
                &ops.pop().unwrap(),
                &nums.pop_front().unwrap(),
                &nums.pop_front().unwrap(),
            );
            loop {
                if nums.is_empty() {
                    break;
                }
                res = compute(&ops.pop().unwrap(), &res, &nums.pop_front().unwrap());
            }
            // println!("res={res}");
            if res == e.test_value {
                // println!("found");
                return true;
            }
        }

        false
    }
}

fn main() {
    //let filename = "2024/day07/test.txt";
    let filename = "2024/day07/input.txt";
    let mut p: Puzzle = Puzzle::new();

    let _ = p.load(filename);
    println!("{:?}", p);

    println!("part1 = {:?}", p.part1());
    println!("part2 = {:?}", p.part2());
}
