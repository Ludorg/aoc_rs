use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // part1
    let mut val1: i32 = 0;

    // part2
    let mut val2: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chars: Vec<_> = line.chars().collect();
        println!(
            "Elfo plays: {}, I need to play : {}",
            elfo_action(chars[0]),
            my_action(chars[2])
        );
        val1 += compute1(chars[0], chars[2]);
        println!(
            "Elfo plays: {}, I need to: {}",
            elfo_action(chars[0]),
            my_action_2(chars[2])
        );
        val2 += compute2(chars[0], chars[2]);
    }
    println!("My score for part 1 is {}", val1);
    println!("My score for part 2 is {}", val2);
}

fn elfo_action(e: char) -> &'static str {
    match e {
        'A' => "Rock",
        'B' => "Paper",
        'C' => "Scissors",
        _ => "Nothing (this is an error)",
    }
}

fn my_action(e: char) -> &'static str {
    match e {
        'X' => "Rock",
        'Y' => "Paper",
        'Z' => "Scissors",
        _ => "Nothing (this is an error)",
    }
}

fn my_action_2(e: char) -> &'static str {
    match e {
        'X' => "lose",
        'Y' => "draw",
        'Z' => "win",
        _ => "Nothing (this is an error)",
    }
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
pub fn compute1(a: char, y: char) -> i32 {
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

// A : rock
// B : paper
// C : scissors
// X : Lose
// Y : Draw
// Z : Win
pub fn compute2(a: char, y: char) -> i32 {
    match a {
        'A' => match y {
            // Elfo plays Rock
            'X' => 3 + 0, // Loss needed, play Scissors (3)
            'Y' => 1 + 3, // Draw, play Rock (1)
            'Z' => 2 + 6, // Win, play paper (2)
            _ => 0,
        },
        'B' => match y {
            // Elfo plays Paper
            'X' => 1 + 0, // Loss, play Rock (1)
            'Y' => 2 + 3, // Draw, play Paper (2)
            'Z' => 3 + 6, // Win, play Scissors (3)
            _ => 0,
        },
        'C' => match y {
            // Elfo plays Scissor
            'X' => 2 + 0, // Loss, play Paper (2)
            'Y' => 3 + 3, // Draw, play Scissors (3)
            'Z' => 1 + 6, // Win, play Rock (1)
            _ => 0,
        },
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::compute1;
    #[test]
    fn example_part1() {
        assert_eq!(compute1('A', 'Y'), 8);
        assert_eq!(compute1('B', 'X'), 1);
        assert_eq!(compute1('C', 'Z'), 6);
        assert_eq!(compute1('D', 'E'), 0);
        assert_eq!(compute1('A', 'E'), 0);
        assert_eq!(compute1('E', 'X'), 0);
    }

    use super::compute2;
    #[test]
    fn example_part2() {
        assert_eq!(compute2('A', 'Y'), 4);
        assert_eq!(compute2('B', 'X'), 1);
        assert_eq!(compute2('C', 'Z'), 7);
        assert_eq!(compute2('D', 'E'), 0);
        assert_eq!(compute2('A', 'E'), 0);
        assert_eq!(compute2('E', 'X'), 0);
    }
}
