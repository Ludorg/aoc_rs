use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_substring(s: &str) -> u32 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn compute_line(s: &str) -> u32 {
    let mut ret = 0;
    for ss in s.split(',') {
        ret += compute_substring(ss);
    }
    ret
}

fn compute_file(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    if let Some((_index, line)) = reader.lines().enumerate().next() {
        return compute_line(line.unwrap().as_str());
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_line() {
        assert_eq!(
            compute_line("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn test_compute_file() {
        assert_eq!(compute_file("example.txt"), 1320);
    }

    #[test]
    fn test_compute_substring() {
        assert_eq!(compute_substring("rn=1"), 30);
        assert_eq!(compute_substring("cm-"), 253);
        assert_eq!(compute_substring("qp=3"), 97);
    }
}

fn main() {
    println!("day15/part1={}", compute_file("2023/day15/example.txt"));
}
