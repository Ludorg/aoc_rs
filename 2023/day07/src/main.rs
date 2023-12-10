use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

impl Hand {
    fn new(s: &str) -> Self {
        let mut cards = vec![];
        if s.len() == 5 {
            for c in s.chars() {
                cards.push(c);
            }
        }

        Self { cards }
    }
    fn get_type(&self) -> u32 {
        let mut cpy = self.cards.clone();
        cpy.dedup();
        match cpy.len() {
            1 => 7, // is_five
            2 => 6, // is_four or is_full_house
            3 => // is_two_pairs or is_three
            4 => // is_one_pair
            5 => 1
        }

    }
}

fn char_to_card_value(c: char) -> u32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_cards() {
        assert!(char_to_card_value('A') > char_to_card_value('K'));
        assert!(char_to_card_value('7') != char_to_card_value('K'));
        assert!(char_to_card_value('7') == char_to_card_value('7'));
    }
}

fn main() {
    println!("Hello, world!");
}
