//! [Advent of Code 2024 Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug)]
struct Puzzle {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        self.data = Vec::new();
        for l in reader.lines() {
            self.data.push(l.unwrap().chars().collect());
        }
        self.width = self.data[0].len();
        self.height = self.data.len();
    }

    fn new() -> Self {
        Self {
            data: vec![],
            width: 0,
            height: 0,
        }
    }
}

fn main() {
    //let filename = "2024/day08/input.txt";
    let filename = "2024/day08/test.txt";
    let mut p: Puzzle = Puzzle::new();
    p.load(filename);
    println!("{:?}", p);
}
