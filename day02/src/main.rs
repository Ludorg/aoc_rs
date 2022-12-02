use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut val: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chars: Vec<_> = line.chars().collect();
        println!("Elfo played: {}, I played: {}", chars[0], chars[2]);
        val += compute(chars[0], chars[2]);
    }
    println!("My score is {}", val);
}

// A : rock
// B : paper
// C : scissors
// X : rock => 1
// Y : paper => 2
// Z : scissors => 3
// 0 loss
// 3 draw
// 6 win
pub fn compute(a: char, y: char) -> i32 {
    match a {
        'A' => match y {
            // Elfo plays Rock
            'X' => 1 + 3, // Draw
            'Y' => 2 + 6, // I played Paper, it's a win
            'Z' => 3 + 0, // I played Scissors, it's a loss
            _ => 0,
        },
        'B' => match y {
            // Elfo plays Paper
            'X' => 1 + 0, // Loss
            'Y' => 2 + 3, // Draw
            'Z' => 3 + 6,
            _ => 0,
        },
        'C' => match y {
            // Elfo plays Scissor
            'X' => 1 + 6, // I played Rock, it's a win
            'Y' => 2 + 0, //
            'Z' => 3 + 3, // Draw
            _ => 0,
        },
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn example_part1() {
        assert_eq!(compute('A', 'Y'), 8);
        assert_eq!(compute('B', 'X'), 1);
        assert_eq!(compute('C', 'Z'), 6);
        assert_eq!(compute('D', 'E'), 0);
        assert_eq!(compute('A', 'E'), 0);
        assert_eq!(compute('E', 'X'), 0);
    }
}
