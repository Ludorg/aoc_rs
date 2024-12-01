use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //let filename = "2024/day01/test.txt";
    let filename = "2024/day01/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // vectors to store data
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    // read line into vectors
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("invalid number"))
            .collect();

        vec1.push(nums[0]);
        vec2.push(nums[1]);
        println!(
            "i={}/ line={} / left={} / right={}",
            _index + 1,
            line,
            nums[0],
            nums[1]
        );
    }
    vec1.sort();
    vec2.sort();

    let mut sum = 0;

    for i in 0..vec1.len() {
        let diff = vec1[i] - vec2[i];
        let d = diff.abs();
        sum += d;
        println!("distance={d}")
    }
    println!("total distance={sum}")
}
