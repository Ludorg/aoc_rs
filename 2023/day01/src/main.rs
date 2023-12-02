use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // set RUST_LOG=info before running
    env_logger::init();
    let filename = "2023/day01/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ret1 = 0;
    let mut ret2 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let item = &line.unwrap();
        ret1 += calibration_value1(item);
        ret2 += calibration_value2(item);
    }
    info!("calibration values sum for part 1 is {}", ret1);
    info!("calibration values sum for part 2 is {}", ret2);
}

pub fn first_digit1(s: &str) -> u32 {
    let mut ret = 0;
    for c in s.chars() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

pub fn last_digit1(s: &str) -> u32 {
    let mut ret = 0;
    for c in s.chars().rev() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

pub fn calibration_value1(s: &str) -> u32 {
    first_digit1(s) * 10 + last_digit1(s)
}

pub fn first_digit2(s: &str) -> u32 {
    let mut num_pos = 0;
    let mut num_ret = 0;
    for c in s.chars() {
        if c.is_numeric() {
            num_ret = c.to_digit(10).unwrap();
            break;
        }
        num_pos += 1;
    }

    // one is at index 0
    let vec = vec![
        s.find("one"),
        s.find("two"),
        s.find("three"),
        s.find("four"),
        s.find("five"),
        s.find("six"),
        s.find("seven"),
        s.find("eight"),
        s.find("nine"),
    ];

    let mut min_str_pos = s.len();
    let mut str_idx = 1;
    let mut str_ret = 0;
    for v in vec {
        if let Some(val) = v {
            if val < min_str_pos {
                min_str_pos = val;
                str_ret = str_idx;
            }
            debug!("string for number {} is at pos {}", str_idx, val);
            debug!("str_ret = {}", str_ret);
        }
        str_idx += 1;
    }

    if min_str_pos > num_pos {
        num_ret
    } else {
        str_ret
    }
}

pub fn last_digit2(s: &str) -> u32 {
    let mut num_pos = s.len();
    let mut num_ret = 0;
    for c in s.chars().rev() {
        if c.is_numeric() {
            num_ret = c.to_digit(10).unwrap();
            break;
        }
        num_pos -= 1;
    }

    let vec = vec![
        s.rfind("one"),
        s.rfind("two"),
        s.rfind("three"),
        s.rfind("four"),
        s.rfind("five"),
        s.rfind("six"),
        s.rfind("seven"),
        s.rfind("eight"),
        s.rfind("nine"),
    ];

    let mut max_str_pos = 0;
    let mut str_idx = 1;
    let mut str_ret = 0;
    for v in vec {
        if let Some(val) = v {
            if val > max_str_pos {
                max_str_pos = val;
                str_ret = str_idx;
            }
            debug!("string for number {} is at pos {}", str_idx, val);
            debug!("str_ret = {}", str_ret);
        }
        str_idx += 1;
    }

    if max_str_pos < num_pos {
        num_ret
    } else {
        str_ret
    }
}

pub fn string_number_to_int_val(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

pub fn calibration_value2(s: &str) -> u32 {
    first_digit2(s) * 10 + last_digit2(s)
}

pub fn string_find_last_pos(s: &str, pat: &str) -> Option<usize> {
    s.rfind(&pat)
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_string_find_last_pos() {
        assert_eq!(
            string_find_last_pos(&String::from("abcdefsix"), "six"),
            Some(6)
        );

        assert_eq!(
            string_find_last_pos(&String::from("7pqrstsixteen"), "four"),
            None
        );
        assert_eq!(
            string_find_last_pos(&String::from("sixteenteen"), "teen"),
            Some(7)
        );
        assert_eq!(
            string_find_last_pos(&String::from("zzzzccabab"), "ab"),
            Some(8)
        );
    }

    #[test]
    fn test_first_digit1() {
        assert_eq!(first_digit1(&String::from("1abc2")), 1);
        assert_eq!(first_digit1(&String::from("treb7uchet")), 7);
    }
    #[test]
    fn test_last_digit1() {
        assert_eq!(last_digit1(&String::from("1abc2")), 2);
        assert_eq!(last_digit1(&String::from("treb7uchet")), 7);
    }
    #[test]
    fn test_calibration_value1() {
        assert_eq!(calibration_value1(&String::from("1abc2")), 12);
        assert_eq!(calibration_value1(&String::from("pqr3stu8vwx")), 38);
        assert_eq!(calibration_value1(&String::from("a1b2c3d4e5f")), 15);
        assert_eq!(calibration_value1(&String::from("treb7uchet")), 77);
    }
    #[test]
    fn test_string_number_to_int_val() {
        assert_eq!(string_number_to_int_val(&String::from("one")), 1);
        assert_eq!(string_number_to_int_val(&String::from("lefkef")), 0);
        assert_eq!(string_number_to_int_val(&String::from("two")), 2);
    }
    #[test]
    fn test_first_digit2() {
        assert_eq!(first_digit2(&String::from("abcone2threexyz")), 1);
        assert_eq!(first_digit2(&String::from("abcsevenone2threexyz")), 7);
        assert_eq!(first_digit2(&String::from("two1nine")), 2);
        assert_eq!(first_digit2(&String::from("1two1nine")), 1);
        assert_eq!(first_digit2(&String::from("one1two1nine")), 1);
        assert_eq!(first_digit2(&String::from("two1nine")), 2);
        assert_eq!(first_digit2(&String::from("eightwothree")), 8);
        assert_eq!(first_digit2(&String::from("abcone2threexyz")), 1);
        assert_eq!(first_digit2(&String::from("xtwone3four")), 2);
        assert_eq!(first_digit2(&String::from("7xtwone3four")), 7);
        assert_eq!(first_digit2(&String::from("4nineeightseven2")), 4);
        assert_eq!(first_digit2(&String::from("zoneight234")), 1);
        assert_eq!(first_digit2(&String::from("7pqrstsixteen")), 7);
    }

    #[test]
    fn test_last_digit2() {
        assert_eq!(last_digit2(&String::from("abcone2threexyz")), 3);
        assert_eq!(last_digit2(&String::from("abcsevenone2threexyz")), 3);
        assert_eq!(last_digit2(&String::from("two1nine")), 9);
        assert_eq!(last_digit2(&String::from("1two1nine")), 9);
        assert_eq!(last_digit2(&String::from("one1two1nine")), 9);
        assert_eq!(last_digit2(&String::from("two1nine")), 9);
        assert_eq!(last_digit2(&String::from("eightwothree")), 3);
        assert_eq!(last_digit2(&String::from("abcone2threexyz")), 3);
        assert_eq!(last_digit2(&String::from("xtwone3four")), 4);
        assert_eq!(last_digit2(&String::from("7xtwone3four7")), 7);
        assert_eq!(last_digit2(&String::from("4nineeightseven2")), 2);
        assert_eq!(last_digit2(&String::from("zoneight234")), 4);
        assert_eq!(last_digit2(&String::from("7pqrstsixteen")), 6);
    }
    #[test]
    fn test_calibration_value2() {
        assert_eq!(calibration_value2(&String::from("two1nine")), 29);
        assert_eq!(calibration_value2(&String::from("eightwothree")), 83);
        assert_eq!(calibration_value2(&String::from("abcone2threexyz")), 13);
        assert_eq!(calibration_value2(&String::from("xtwone3four")), 24);
        assert_eq!(calibration_value2(&String::from("4nineeightseven2")), 42);
        assert_eq!(calibration_value2(&String::from("zoneight234")), 14);
        assert_eq!(calibration_value2(&String::from("7pqrstsixteen")), 76);
    }
}
