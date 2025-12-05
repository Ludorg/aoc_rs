// 2025 - Day 4: Printing Department
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut puzzle = Puzzle { data: vec![] };
    puzzle.load("2025/day04/input.txt");

    println!("part1={}", puzzle.part1());
    println!("part2={}", puzzle.part2());

    puzzle.count_neighbouring_rolls(0, 0);
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

    // TODO use fmt::Debug
    fn print(&self) {
        for (x, row) in self.data.iter().enumerate() {
            for y in 0..row.len() {
                print!("{}", self.data[x][y]);
            }
            println!("");
        }
    }

    fn count_neighbouring_rolls(&self, x: isize, y: isize) -> u32 {
        let height = self.data.len() as isize;
        if height == 0 {
            return 0;
        }
        if x < 0 || y < 0 || x >= height {
            return 0;
        }

        let mut count: u32 = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= height || ny < 0 {
                    continue;
                }
                let nx_us = nx as usize;
                let ny_us = ny as usize;
                let row = &self.data[nx_us];
                if ny_us >= row.len() {
                    continue;
                }
                if row[ny_us] != '.' {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_removable_rolls(&self) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = vec![];
        for (x, row) in self.data.iter().enumerate() {
            for y in 0..row.len() {
                if self.data[x][y] == '@' {
                    let neighbors = self.count_neighbouring_rolls(x as isize, y as isize);
                    if neighbors < 4 {
                        out.push((x, y));
                    }
                }
            }
        }
        out
    }

    fn remove_rolls(&mut self, rolls: Vec<(usize, usize)>) {
        for r in rolls {
            self.data[r.0][r.1] = '.';
        }
    }

    fn part1(&self) -> u32 {
        let mut count: u32 = 0;
        for (x, row) in self.data.iter().enumerate() {
            for y in 0..row.len() {
                if self.data[x][y] == '@' {
                    let neighbors = self.count_neighbouring_rolls(x as isize, y as isize);
                    if neighbors < 4 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn part2(&mut self) -> usize {
        let mut out = 0;
        loop {
            let removable = self.get_removable_rolls();
            if removable.len() == 0 {
                break;
            }
            out += removable.len();
            self.remove_rolls(removable);
        }
        out
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part2(), 43);
    }

    #[test]
    fn test_get_removable_rolls() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        puzzle.print();

        let removable = puzzle.get_removable_rolls();
        assert_eq!(removable.len(), 13);
        assert_eq!(removable[0], (0, 2));

        puzzle.remove_rolls(removable);
        println!("AFTER REMOVE");
        puzzle.print();
    }

    #[test]
    fn test_count_neighbouring_rolls() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        puzzle.print();

        assert_eq!(puzzle.count_neighbouring_rolls(0, 0), 2);
        assert_eq!(puzzle.count_neighbouring_rolls(1, 1), 6);
        assert_eq!(puzzle.count_neighbouring_rolls(0, 1), 4);
        assert_eq!(puzzle.count_neighbouring_rolls(1, 0), 3);
    }

    #[test]
    fn test_part1() {
        let mut puzzle = Puzzle { data: vec![] };
        puzzle.load("test.txt");
        assert_eq!(puzzle.part1(), 13);
    }
}
