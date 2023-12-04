use log::{debug, info, trace};
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // set RUST_LOG=info before running
    env_logger::init();
    let s = load_schematic("2023/day03/input.txt");
    let sum = compute_part_numbers(&s);
    info!("{sum}")
}

pub struct Schematic {
    data: Vec<String>,
}

impl Schematic {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn dimensions(&self) -> (usize, usize) {
        // assumes that the schemantic lines are always of the same size
        (self.data[0].len(), self.data.len())
    }
    fn char_at(&self, x: i32, y: i32) -> char {
        let (dx, dy) = self.dimensions();
        if (x >= 0) & ((x as usize) < dx) & (y >= 0) & ((y as usize) < dy) {
            let c = self.data[y as usize].chars().nth(x as usize).unwrap();
            trace!("char_at [{x},{y}]='{c}'");
            c
        } else {
            trace!("char_at [{x},{y}]='.'");
            '.'
        }
    }
    fn is_digit_adjacent_to_symbol(&self, x: i32, y: i32) -> bool {
        // x+1, y
        // x-1, y
        is_symbol(self.char_at(x+1, y))
            | is_symbol(self.char_at(x-1, y))
        // x-1, y-1
        // x, y-1
        // x+1, y-1
            | is_symbol(self.char_at(x-1, y-1))
            | is_symbol(self.char_at(x, y-1))
            | is_symbol(self.char_at(x+1, y-1))
        // x-1, y+1
        // x, y+1
        // x+1, y+1
            | is_symbol(self.char_at(x-1, y+1))
            | is_symbol(self.char_at(x, y+1))
            | is_symbol(self.char_at(x+1, y+1))
    }
}

pub fn is_symbol(c: char) -> bool {
    if c.is_ascii_digit() | (c == '.') {
        return false;
    }
    true
}

pub fn compute_part_numbers(s: &Schematic) -> u32 {
    let (dx, dy) = s.dimensions();
    let mut sum = 0;
    let mut adjacent = false;
    let mut number_found = false;
    for y in 0..dy {
        let mut number = 0;
        for x in 0..dx {
            // if current char is digit
            if s.char_at(x as i32, y as i32).is_ascii_digit() {
                number_found = true;
                let digit = s.char_at(x as i32, y as i32).to_digit(10).unwrap();
                trace!("digit={digit}");
                number = number * 10 + digit;
                if s.is_digit_adjacent_to_symbol(x as i32, y as i32) {
                    adjacent = true;
                }
            } else {
                // this is not a digit, check if ending a number
                if number_found {
                    if adjacent {
                        sum += number;
                        debug!("number={number} is adjacent to a symbol");
                    } else {
                        debug!("number={number} is NOT adjacent to a symbol");
                    }
                    number_found = false;
                    adjacent = false;
                    number = 0;
                }
            }
            // special case for end of line
            if x == dx - 1 {
                if number_found {
                    if adjacent {
                        sum += number;
                        debug!("at end of line number={number} is adjacent to a symbol");
                    } else {
                        debug!("at end of line number={number} is NOT adjacent to a symbol");
                    }
                }
                number_found = false;
                adjacent = false;
                number = 0;
            }
        }
    }
    sum
}

pub fn load_schematic(filename: &str) -> Schematic {
    let mut s: Schematic = Schematic::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let item = line.unwrap();
        s.data.push(item);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_schematic() {
        let s = load_schematic("test.txt");
        trace!("{}", s.data[0]);
        assert_eq!(s.data[0], "467..114..");
        assert_eq!(s.data[9], ".664.598..");
    }

    #[test]
    fn test_dimensions() {
        let s = load_schematic("test.txt");
        let (x, y) = s.dimensions();
        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }
    #[test]
    fn test_is_symbol() {
        assert_eq!(is_symbol('1'), false);
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('*'), true);
        assert_eq!(is_symbol('+'), true);
        assert_eq!(is_symbol('#'), true);
    }

    #[test]
    fn test_adjacent_digits() {
        let s = load_schematic("test.txt");
        //   0123456789
        // 0 467..114..
        // 1 ...*......
        // 2 ..35..633.
        // 3 ......#...
        // 4 617*......
        // 5 .....+.58.
        // 6 ..592.....
        // 7 ......755.
        // 8 ...$.*....
        // 9 .664.598..

        assert_eq!(s.is_digit_adjacent_to_symbol(5, 0), false); // '1' of 114
        assert_eq!(s.is_digit_adjacent_to_symbol(6, 0), false); // second '1' of 114
        assert_eq!(s.is_digit_adjacent_to_symbol(7, 0), false); // '4' of 114

        assert_eq!(s.is_digit_adjacent_to_symbol(2, 2), true); // '3' of 35
        assert_eq!(s.is_digit_adjacent_to_symbol(3, 2), true); // '5' of 35

        assert_eq!(s.is_digit_adjacent_to_symbol(7, 5), false); // '5' of 58
        assert_eq!(s.is_digit_adjacent_to_symbol(8, 5), false); // '8' of 58
    }
    #[test]
    fn test_compute_part_numbers() {
        env_logger::init();
        let s = load_schematic("test.txt");
        assert_eq!(compute_part_numbers(&s), 4361);
    }

    #[test]
    fn test_compute_part_numbers_find_bug() {
        let s = load_schematic("test_bug.txt");
        compute_part_numbers(&s);
    }
}

//fn is_adjdacent
