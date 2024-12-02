use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_increasing(v: &Vec<i32>) -> bool {
    v[1] > v[0]
}

fn is_safe(v: &Vec<i32>) -> bool {
    let is_increasing = is_increasing(v);
    for i in 1..(v.len()) {
        if is_increasing {
            let diff = v[i] - v[i - 1];
            println!("+ / {diff}");
            if diff > 3 || diff == 0 || diff < 0 {
                return false;
            }
        } else {
            let diff = v[i - 1] - v[i];
            println!("- / {diff}");
            if diff > 3 || diff == 0 || diff < 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    //let filename = "2024/day02/test.txt";

    let filename = "2024/day02/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut safe_reports = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("invalid number"))
            .collect();
        let is_safe = is_safe(&levels);
        println!(
            "i={}/ line={} / levels={:?} / is_safe={}",
            _index + 1,
            line,
            levels,
            is_safe
        );
        if is_safe {
            safe_reports += 1
        }
    }
    println!("safe_reports={safe_reports}")
}
