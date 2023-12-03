use log::{debug, info, trace};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // set RUST_LOG=info before running
    env_logger::init();
    let filename = "2023/day02/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
}
