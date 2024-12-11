//! [Advent of Code 2024 Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
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

    fn find_antinode_coord(
        p1: &(isize, isize),
        p2: &(isize, isize),
        h: isize,
        w: isize,
    ) -> Vec<(isize, isize)> {
        let mut ret = Vec::new();

        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        if p1.0 - dx >= 0 && p1.0 - dx < w && p1.1 - dy >= 0 && p1.1 - dy < h {
            ret.push((p1.0 - dx, p1.1 - dy));
        }
        if p2.0 + dx >= 0 && p2.0 + dx < w && p2.1 + dy >= 0 && p2.1 + dy < h {
            ret.push((p2.0 + dx, p2.1 + dy));
        }

        ret
    }

    fn find_antinodes_nb(&self) -> isize {
        let mut antinode_pos = HashMap::new();
        let antennas = self.find_antennas();

        for antenna in &antennas {
            let pos = antenna.1;
            for c in pos.iter().combinations(2) {
                println!("combinaison={:?}", c);

                let p1 = c[0];
                let p2 = c[1];
                let coord = Self::find_antinode_coord(
                    p1,
                    p2,
                    self.grid_height as isize,
                    self.grid_width as isize,
                );
                println!("coord={:?}", coord);
                for a_pos in coord {
                    antinode_pos.insert(a_pos, true);
                }
            }
        }

        antinode_pos.len() as isize
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
