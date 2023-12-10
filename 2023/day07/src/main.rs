use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq)]
enum HandType {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    Two = 3,
    One = 2,
    High = 1,
    None = 0, // should not exist
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
    fn get_type(&self) -> HandType {
        let mut cpy = self.cards.clone();
        cpy.sort();
        cpy.dedup();
        match cpy.len() {
            1 => HandType::Five, // is_five
            2 => {
                // is_four or is_full_house
                // grab the first card and count it in the hand
                let _1_or_4 = self.cards.iter().filter(|&n| *n == self.cards[0]).count();
                if _1_or_4 == 1 || _1_or_4 == 4 {
                    HandType::Four
                } else {
                    HandType::Full
                }
            }
            3 => {
                // is_three or is_two_pairs
                let mut cpy = self.cards.clone();
                cpy.sort();
                if cpy[0] == cpy[1] {
                    if cpy[0] == cpy[2] {
                        HandType::Three
                    } else if cpy[2] == cpy[3] {
                        HandType::Two
                    } else {
                        HandType::Three
                    }
                } else if cpy[1] == cpy[2] {
                    if cpy[1] == cpy[3] {
                        HandType::Three
                    } else {
                        HandType::Two
                    }
                } else {
                    HandType::Three
                }
            }
            4 => HandType::One,  // is_one_pair
            5 => HandType::High, // is_high
            _ => HandType::None,
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
    fn test_get_type() {
        assert!(Hand::new("AAAAA").get_type() == HandType::Five);
        assert!(Hand::new("AA8AA").get_type() == HandType::Four);
        assert!(Hand::new("AAAA8").get_type() == HandType::Four);
        assert!(Hand::new("23332").get_type() == HandType::Full);
        assert!(Hand::new("33232").get_type() == HandType::Full);
        assert!(Hand::new("TTT98").get_type() == HandType::Three);
        assert!(Hand::new("T8T9T").get_type() == HandType::Three);
        assert!(Hand::new("TT8T9").get_type() == HandType::Three);
        assert!(Hand::new("TT89T").get_type() == HandType::Three);
        assert!(Hand::new("23432").get_type() == HandType::Two);
        assert!(Hand::new("22344").get_type() == HandType::Two);
        assert!(Hand::new("A23A4").get_type() == HandType::One);
        assert!(Hand::new("23456").get_type() == HandType::High);
    }

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
