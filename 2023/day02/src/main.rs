use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "2023/day02/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {}
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn read_game(s: String) -> Game {
    Game {
        id: 1,
        sets: vec![Set {
            red: 0,
            green: 0,
            blue: 0,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_game() {
        let g1 = Game {
            id: 1,
            sets: vec![Set {
                red: 0,
                green: 0,
                blue: 0,
            }],
        };
        println!("{g1:?}");
        let s1 = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        println!("{s1:?}");

        // assert_eq!(
        //     read_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        //     Game {
        //         id: 1,
        //         sets: vec![Set {
        //             red: 0,
        //             green: 0,
        //             blue: 0,
        //         }],
        //     }
        // );
    }
}
