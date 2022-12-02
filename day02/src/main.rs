fn main() {
    println!("{}", compute('A', 'Y'));
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
            'X' => 1 + 3,
            'Y' => 2 + 6,
            'Z' => 3 + 0,
            _ => 0,
        },
        'B' => match y {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => 0,
        },
        'C' => match y {
            'X' => 1 + 0,
            'Y' => 2 + 6,
            'Z' => 3 + 3,
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
