use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

#[derive(PartialEq, PartialOrd)]
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
        Self { cards, bid: 0 }
    }

    fn new2(s: &str, bid_str: &str) -> Self {
        let mut h = Self::new(s);
        h.bid = bid_str.parse().unwrap();
        h
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
                // the should be six combinations
                // AAABC
                // AABBC
                // ABBCC
                // ABBBC
                // ABCCC
                // AABCC
                if cpy[0] == cpy[1] {
                    if cpy[0] == cpy[2] {
                        HandType::Three // AAABC
                    } else {
                        HandType::Two // AABBC of AABCC
                    }
                } else if cpy[1] == cpy[2] {
                    if cpy[1] == cpy[3] {
                        HandType::Three // ABBBC
                    } else {
                        HandType::Two // ABBCC
                    }
                } else {
                    HandType::Three // ACBBB
                }
            }
            4 => HandType::One,  // is_one_pair
            5 => HandType::High, // is_high
            _ => HandType::None,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = self.get_type();
        let o = other.get_type();
        let mut ret = Ordering::Equal;

        if s > o {
            Ordering::Greater
        } else if o > s {
            Ordering::Less
        } else {
            // hands are of equal types, compare firsts cards
            for idx in 0..self.cards.len() {
                if char_to_card_value(self.cards[idx]) > char_to_card_value(other.cards[idx]) {
                    ret = Ordering::Greater;
                    break;
                }
                if char_to_card_value(self.cards[idx]) < char_to_card_value(other.cards[idx]) {
                    ret = Ordering::Less;
                    break;
                }
            }
            ret
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

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn load(filename: &str) -> Self {
        let mut g = Self { hands: vec![] };

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // let mut id_sum = 0;
        // let mut power_sum = 0;
        for (_index, line) in reader.lines().enumerate() {
            let item = &line.unwrap();
            let v: Vec<_> = item.split(' ').collect();
            g.hands.push(Hand::new2(v[0], v[1]));
        }
        g
    }
    fn get_total_winnings(&self) -> u32 {
        println!("before sort={:?}", self.hands);
        let mut hands = self.hands.clone();
        hands.sort();
        println!("after sort={:?}", hands);

        let mut ret = 0;
        let mut idx = 1;

        for h in hands {
            println!("{:?}", h);
            ret += h.bid * idx;
            idx += 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_total_winnings() {
        let g = Game::load("example_not_sorted.txt");
        assert_eq!(g.get_total_winnings(), 6440);
    }

    #[test]
    fn test_game_load() {
        let g = Game::load("example.txt");
        assert!(g.hands.len() == 5);
        println!("{:?}", g);
    }

    #[test]
    fn test_new2() {
        assert!(Hand::new2("32T3K", "765").bid == 765);
        assert!(Hand::new2("32T3K", "765").get_type() == HandType::One);

        assert!(Hand::new2("32T3K", "465").get_type() == HandType::One);
        assert!(Hand::new2("32T3K", "465") >= Hand::new2("32T3K", "765"));
        assert!(Hand::new2("32T3K", "465") == Hand::new2("32T3K", "765"));
    }

    #[test]
    fn test_compare() {
        assert!(Hand::new("33332") > Hand::new("2AAAA")); // amazing !!
        assert!(Hand::new("77888") > Hand::new("77788")); // amazing !!
    }

    #[test]
    fn test_eq() {
        assert!(Hand::new("AAAAA") == Hand::new("AAAAA"));
        assert!(Hand::new("7AAAA") != Hand::new("AAAA7"));
    }

    #[test]
    fn test_get_type() {
        assert!(Hand::new("AAAAA").get_type() == HandType::Five);

        assert!(Hand::new("KKKAK").get_type() == HandType::Four);
        assert!(Hand::new("AA8AA").get_type() == HandType::Four);
        assert!(Hand::new("AAAA8").get_type() == HandType::Four);

        assert!(Hand::new("23332").get_type() == HandType::Full);
        assert!(Hand::new("33232").get_type() == HandType::Full);
        assert!(Hand::new("23332").get_type() == HandType::Full);
        assert!(Hand::new("43344").get_type() == HandType::Full);

        assert!(Hand::new("TTT98").get_type() == HandType::Three);
        assert!(Hand::new("T8T9T").get_type() == HandType::Three);
        assert!(Hand::new("TT8T9").get_type() == HandType::Three);
        assert!(Hand::new("TT89T").get_type() == HandType::Three);

        assert!(Hand::new("23432").get_type() == HandType::Two);
        assert!(Hand::new("22344").get_type() == HandType::Two);
        assert!(Hand::new("22443").get_type() == HandType::Two);
        assert!(Hand::new("44223").get_type() == HandType::Two);
        assert!(Hand::new("33224").get_type() == HandType::Two);

        assert!(Hand::new("32T3K").get_type() == HandType::One);

        assert!(Hand::new("A23A4").get_type() == HandType::One);
        assert!(Hand::new("323A4").get_type() == HandType::One);
        assert!(Hand::new("423A4").get_type() == HandType::One);

        assert!(Hand::new("23456").get_type() == HandType::High);
        assert!(Hand::new("AJKT2").get_type() == HandType::High);
        assert!(Hand::new("73456").get_type() == HandType::High);
    }

    #[test]
    fn test_compare_cards() {
        assert!(char_to_card_value('A') > char_to_card_value('K'));
        assert!(char_to_card_value('7') != char_to_card_value('K'));
        assert!(char_to_card_value('7') == char_to_card_value('7'));
    }
}

fn main() {
    let g = Game::load("2023/day07/input.txt");
    println!("day07/part1={:?}", g.get_total_winnings());
}
