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

    fn count_xmas(self) -> i32 {
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == 'X' {
                    count += self.count_straight(i, j);
                    count += self.count_diagonal(i, j);
                }
            }
        }
        count
    }

    fn count_straight(&self, i: usize, j: usize) -> i32 {
        let mut straight = 0;
        if j + 3 < self.width && self.data[i][j + 1] == 'M'
                && self.data[i][j + 2] == 'A' && self.data[i][j + 3] == 'S' {
            straight += 1
        }

        if j >= 3 && self.data[i][j - 1] == 'M'
                && self.data[i][j - 2] == 'A' && self.data[i][j - 3] == 'S' {
            straight += 1
        }

        if i + 3 < self.height && self.data[i + 1][j] == 'M'
                && self.data[i + 2][j] == 'A' && self.data[i + 3][j] == 'S' {
            straight += 1
        }

        if i >= 3 && self.data[i - 1][j] == 'M'
                && self.data[i - 2][j] == 'A' && self.data[i - 3][j] == 'S' {
            straight += 1
        }

        straight
    }

    fn count_diagonal(&self, i: usize, j: usize) -> i32 {
        let mut diagonal = 0;
        if i + 3 < self.height && j + 3 < self.width && self.data[i + 1][j + 1] == 'M'
                && self.data[i + 2][j + 2] == 'A' && self.data[i + 3][j + 3] == 'S' {
            diagonal += 1
        }

        if i >= 3 && j >= 3 && self.data[i - 1][j - 1] == 'M'
                && self.data[i - 2][j - 2] == 'A' && self.data[i - 3][j - 3] == 'S' {
            diagonal += 1
        }

        if i + 3 < self.height && j >= 3 && self.data[i + 1][j - 1] == 'M'
                && self.data[i + 2][j - 2] == 'A' && self.data[i + 3][j - 3] == 'S' {
            diagonal += 1
        }

        if i >= 3 && j + 3 < self.width && self.data[i - 1][j + 1] == 'M'
                && self.data[i - 2][j + 2] == 'A' && self.data[i - 3][j + 3] == 'S' {
            diagonal += 1
        }

        diagonal
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
    let filename = "2024/day04/input.txt";
    //let filename = "2024/day04/test.txt";
    let mut p: Puzzle = Puzzle::new();
    p.load(filename);
    println!("{:?}", p);

    println!("XMAS count={:?}", p.count_xmas());
}
