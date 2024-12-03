use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the path to the input file
    let path = Path::new("2024/day03/input.txt");

    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Define the regex pattern to match "mul(x, y)"
    let pattern = r"mul\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();
    let mut total_sum = 0;

    // Initialize a variable to keep track of the total sum
    // Read lines from the file and process each line
    let mut mul_enabled = true;
    for line in reader.lines() {
        let line = line?;

        // Find all matches of the regex pattern in the line
        for captures in re.captures_iter(&line) {
            if captures[0].contains("don't") {
                println!("{:?}", captures);
                mul_enabled = false;
                continue;
            }
            if captures[0].contains("do") {
                println!("{:?}", captures);
                mul_enabled = true;
                continue;
            }
            if mul_enabled {
                let num2: i32 = captures[2].parse().unwrap();
                let num1: i32 = captures[1].parse().unwrap();
                // Multiply the captured numbers
                let product = num1 * num2;

                println!("Captured: {} and {} => Product: {}", num1, num2, product);

                total_sum += product;
            }
        }
    }

    // Print the total sum of all products from the captures
    println!("Total Sum of Products: {}", total_sum);

    Ok(())
}
