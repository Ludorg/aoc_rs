//! [Advent of Code 2024 Day 11: Plutonian Pebbles](https://adventofcode.com/2024/day/11)

use std::collections::VecDeque;

fn is_number_digits_even(n: i64) -> bool {
    if n == 0 {
        return false;
    }
    count_digits(n) % 2 == 0
}

fn count_digits(n: i64) -> i64 {
    let mut digit = 0;
    let mut num = n;
    while num != 0 {
        num /= 10;
        digit += 1;
    }
    digit
}

fn split_number(n: i64) -> (i64, i64) {
    let nb = count_digits(n);
    let mut num = n;
    let mut digit = 0;
    while num != 0 {
        num /= 10;
        digit += 1;
        if digit == nb / 2 {
            break;
        }
    }
    let r: i64 = n - num * 10_i32.pow(digit as u32) as i64;
    (num, r)
}

fn string_to_nums(s: &str) -> VecDeque<i64> {
    s.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn apply_rules(v: VecDeque<i64>) -> VecDeque<i64> {
    let mut ret = VecDeque::new();

    for num in v {
        if num == 0 {
            ret.push_back(1);
            continue;
        }
        if is_number_digits_even(num) {
            let split = split_number(num);
            ret.push_back(split.0);
            ret.push_back(split.1);
            continue;
        }
        ret.push_back(num * 2024);
    }

    ret
}

fn main() {
    // part 1
    let mut r1 = apply_rules(string_to_nums(&"125 17".to_string()));
    for _ in 0..24 {
        r1 = apply_rules(r1);
        //println!("{:?}", r1);
    }
    println!("{:?}", r1.len());
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_string_to_nums() {
        let s = "0 1 10 99 999".to_string();
        let v = string_to_nums(&s);
        assert_eq!(v.len(), 5);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 10);
        assert_eq!(v[3], 99);
        assert_eq!(v[4], 999);
    }

    #[test]
    fn test_is_number_digits_even() {
        assert_eq!(is_number_digits_even(123), false);
        assert_eq!(is_number_digits_even(0), false);
        assert_eq!(is_number_digits_even(1), false);
        assert_eq!(is_number_digits_even(12), true);
        assert_eq!(is_number_digits_even(1234), true);
    }

    #[test]
    fn test_split_number() {
        assert_eq!((12, 34), split_number(1234));
        assert_eq!((123, 45), split_number(12345));
        assert_eq!((9988, 7766), split_number(99887766));
        assert_eq!((12345, 67890), split_number(1234567890));
        assert_eq!((99887766, 55443322), split_number(9988776655443322));
    }

    #[test]
    fn test_apply_rules_1() {
        let r1 = apply_rules(string_to_nums(&"0 1 10 99 999".to_string()));
        println!("{:?}", r1);
        assert_eq!(r1[0], 1);
        assert_eq!(r1[1], 2024);
        assert_eq!(r1[2], 1);
        assert_eq!(r1[3], 0);
        assert_eq!(r1[4], 9);
        assert_eq!(r1[5], 9);
        assert_eq!(r1[6], 2021976);
    }

    #[test]
    fn test_apply_rules_2_25() {
        let mut r1 = apply_rules(string_to_nums(&"125 17".to_string()));
        println!("{:?}", r1);

        r1 = apply_rules(string_to_nums(&"125 17".to_string()));
        for _ in 0..24 {
            r1 = apply_rules(r1);
            //println!("{:?}", r1);
        }
        println!("{:?}", r1.len());
        assert_eq!(r1.len(), 55312);
    }
}
