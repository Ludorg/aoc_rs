use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let filename = "2023/day01/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ret = 0;
    for (_index, line) in reader.lines().enumerate() {
        ret += calibration_value(&line.unwrap());
    }
    println!("calibration values sum is {}", ret);
}

fn first_digit(s: &String) -> u32 {
    let mut ret = 0;
    for c in s.chars() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

fn last_digit(s: &String) -> u32 {
    let mut ret = 0;
    for c in s.chars().rev() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

fn calibration_value(s: &String) -> u32 {
    first_digit(&s) * 10 + last_digit(&s)
}

mod tests {

    use super::*;

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit(&String::from("1abc2")), 1);
        assert_eq!(first_digit(&String::from("treb7uchet")), 7);
    }
    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit(&String::from("1abc2")), 2);
        assert_eq!(last_digit(&String::from("treb7uchet")), 7);
    }
    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value(&String::from("1abc2")), 12);
        assert_eq!(calibration_value(&String::from("pqr3stu8vwx")), 38);
        assert_eq!(calibration_value(&String::from("a1b2c3d4e5f")), 15);
        assert_eq!(calibration_value(&String::from("treb7uchet")), 77);
    }
}
