use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let filename = "2023/day01/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ret1 = 0;
    let mut ret2 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let item = &line.unwrap();
        ret1 += calibration_value1(&item);
        ret2 += calibration_value2(&item);
    }
    println!("calibration values sum for part 1 is {}", ret1);
    println!("calibration values sum for part 2 is {}", ret2);
}

fn first_digit1(s: &String) -> u32 {
    let mut ret = 0;
    for c in s.chars() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

fn last_digit1(s: &String) -> u32 {
    let mut ret = 0;
    for c in s.chars().rev() {
        if c.is_numeric() {
            ret = c.to_digit(10).unwrap();
            break;
        }
    }
    ret
}

fn calibration_value1(s: &String) -> u32 {
    first_digit1(&s) * 10 + last_digit1(&s)
}

fn first_digit2(s: &String) -> u32 {
    let mut num_pos = 0;
    let mut num_ret = 0;
    let mut ret = 0;
    for c in s.chars() {
        if c.is_numeric() {
            num_ret = c.to_digit(10).unwrap();
            break;
        }
        num_pos += 1;
    }
    let mut vec = Vec::new();
    //vec.push(usize::MIN);
    vec.push(s.find("one"));
    vec.push(s.find("two"));
    vec.push(s.find("three"));
    vec.push(s.find("four"));
    vec.push(s.find("five"));
    vec.push(s.find("six"));
    vec.push(s.find("seven"));
    vec.push(s.find("eight"));
    vec.push(s.find("nine"));
    let mut min_str_pos = s.len();
    let mut str_idx = 1;
    let mut str_ret = 0;
    for v in vec {
        if v.is_some() {
            if v.unwrap() < min_str_pos {
                min_str_pos = v.unwrap();
                str_ret = str_idx;
            }
            println!("string for number {} is at pos {}", str_idx, v.unwrap());
            println!("str_ret = {}", str_ret);
        }
        str_idx += 1;
    }

    if min_str_pos > num_pos {
        ret = num_ret;
    } else {
        ret = str_ret;
    }
    ret
}

fn string_number_to_int_val(s: &String) -> u32 {
    match s.as_str() {
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

fn calibration_value2(s: &String) -> u32 {
    0
}

mod tests {

    use super::*;

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
}
