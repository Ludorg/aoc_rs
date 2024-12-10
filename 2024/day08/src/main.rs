//! [Advent of Code 2024 Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug)]
struct Puzzle {
    grid_data: Vec<Vec<char>>,
    grid_width: usize,
    grid_height: usize,
}

impl Puzzle {
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        self.grid_data = Vec::new();
        for l in reader.lines() {
            self.grid_data.push(l.unwrap().chars().collect());
        }
        self.grid_width = self.grid_data[0].len();
        self.grid_height = self.grid_data.len();
    }

    fn find_antennas(&self) -> HashMap<char, Vec<(isize, isize)>> {
        let mut ret = HashMap::new();
        let mut i = 0;
        let mut j = 0;
        for l in &self.grid_data {
            i += 1;
            for c in l {
                j += 1;
                if *c == '.' {
                    continue;
                } else {
                    ret.entry(*c).or_insert_with(Vec::new).push((i - 1, j - 1));
                }
            }
            j = 0;
        }
        ret
    }

    fn find_antinodes_nb(&self) -> isize {
        let v = self.find_antennas();
        // for c in v {
        //     let diff = c.1
        // }
        

        v.len() as isize
    }

    fn part1(&self) -> isize {
        self.find_antinodes_nb()
    }

    fn new() -> Self {
        Self {
            grid_data: vec![],
            grid_width: 0,
            grid_height: 0,
        }
    }
}

fn main() {
    //let filename = "2024/day08/input.txt";
    let filename = "2024/day08/test.txt";
    let mut p: Puzzle = Puzzle::new();
    p.load(filename);
    println!("{:?}", p);
    println!("{:?}", p.find_antennas());
    println!("part1={:?}", p.part1());
}
