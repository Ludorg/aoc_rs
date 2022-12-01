use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // vector to store data (calories carried by elves)
    let mut elves: Vec<i32> = Vec::new();

    // Current elf
    let mut current_elf: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // if read value is empty, store accumulated calories by an elf
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            let val: i32 = line.parse().unwrap();
            current_elf += val;
        }
    }

    // reverse sort of elves vector
    elves.sort_by(|a, b| b.cmp(a));

    // part 1
    println!("maximum calories carried is: {}", elves[0]);

    // part 2
    println!(
        "the top three Elves carrying the most Calories are carrying: {}",
        elves[0] + elves[1] + elves[2]
    );
}
