//! [Advent of Code 2024 Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug, Clone)]
struct Puzzle {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Exit,
}

fn char_to_direction(c: &char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => Direction::Exit,
    }
}

fn direction_offset(d: &Direction) -> (isize, isize) {
    match d {
        Direction::Up => (-1, 0),
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Exit => (0, 0),
    }
}

fn rotate_right(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Exit => Direction::Exit,
    }
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
    fn find_start(&self) -> (usize, usize) {
        for i in 0..self.height {
            for j in 0..self.width {
                if char_to_direction(&self.data[i][j]) != Direction::Exit {
                    return (i, j);
                }
            }
        }
        panic!("no start found");
    }

    fn get_next(&self, pos: (usize, usize), d: Direction) -> ((usize, usize), Direction) {
        let offset = direction_offset(&d);
        let next_i = pos.0 as isize + offset.0;
        let next_j = pos.1 as isize + offset.1;

        if next_i < 0 || next_i >= self.height as isize {
            return (pos, Direction::Exit);
        }
        if next_j < 0 || next_j >= self.width as isize {
            return (pos, Direction::Exit);
        }
        if self.data[next_i as usize][next_j as usize] == '#' {
            return (pos, rotate_right(&d));
        }

        ((next_i as usize, next_j as usize), d)
    }

    fn part1(&self) -> i32 {
        let mut cur = self.find_start();
        let mut dir = char_to_direction(&self.data[cur.0][cur.1]);

        let mut count = 0;
        let mut positions = HashMap::new();
        while dir != Direction::Exit {
            let next = self.get_next(cur, dir);
            println!("{count} cur={:?}, dir={:?}, next={:?}", cur, dir, next);

            cur = next.0;
            dir = next.1;

            count += 1;

            positions.insert(cur, true);
        }
        positions.len() as i32
    }

    fn part2(&self) -> i32 {
        let mut count = 0;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == '.' {
                    // clone the puzzle to add an obstacle in the cloned data
                    let mut clone = self.clone();
                    clone.data[i][j] = '#';

                    // do part1 job on the cloned data, but exit when a loop is met
                    let mut cur = clone.find_start();
                    let mut dir = char_to_direction(&clone.data[cur.0][cur.1]);

                    // a loop is the same position, with the same direction
                    // map pos_dir is the loop detector
                    let mut pos_dir = HashMap::new();

                    // part 1 job
                    while dir != Direction::Exit {
                        let next = clone.get_next(cur, dir);
                        //println!("{count} cur={:?}, dir={:?}, next={:?}", cur, dir, next);

                        cur = next.0;
                        dir = next.1;

                        // check if a loop is detected
                        if pos_dir.contains_key(&cur) {
                            if pos_dir.get(&cur) == Some(&dir) {
                                println!("same position and same direction => in a loop");
                                count += 1;
                                // exit the 'part1 job' while loop
                                dir = Direction::Exit;
                            }
                        } else {
                            pos_dir.insert(cur, dir);
                        }
                    }
                }
            }
        }
        count
    }
}

fn main() {
    //let filename = "2024/day06/input.txt";
    let filename = "2024/day06/test.txt";
    let mut p: Puzzle = Puzzle::new();

    p.load(filename);
    println!("{:?}", p);

    let s = p.find_start();
    println!("start is {:?}", s);

    let res = p.part1();
    println!("p1 result is {:?}", res);

    let res = p.part2();
    println!("p2 result is {:?}", res);
}
