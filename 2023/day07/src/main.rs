//#[derive(Debug, PartialOrd, PartialEq)]
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

    fn get_nb_equal_cards(&self, c: &char) -> u32 {
        let mut ret = 0;
        for ci in &self.cards {
            if ci == c {
                ret += 1;
            }
        }
        ret
    }

    fn is_five_of_a_kind(&self) -> bool {
        let mut cpy = self.cards.clone();
        cpy.sort();
        let first = &self.cards[0];
        for c in &self.cards {
            if c != first {
                return false;
            }
        }
        true
    }

    fn is_four_of_a_kind(&self) -> bool {
        for c in &self.cards {
            if self.get_nb_equal_cards(c) == 4 {
                return true;
            }
        }
        false
    }
    fn is_full_house(&self) -> bool {
        if self.is_five_of_a_kind() {
            return false;
        }
        if self.is_four_of_a_kind() {
            return false;
        }
        let ret = false;
        let mut found_3 = 'X';
        let mut found_2 = 'X';

        for c in &self.cards {
            if self.get_nb_equal_cards(c) == 3 {
                println!("found 3 '{c}'");
                found_3 = *c;
                break;
            }
        }
        // if not found 3 cards
        if found_3 == 'X' {
            return false;
        }

        for c in &self.cards {
            if found_3 != *c {
                if self.get_nb_equal_cards(c) == 2 {
                    println!("found 2 '{c}'");
                    found_2 = *c;
                    break;
                }
            }
        }

        if found_2 == 'X' {
            return false;
        }

        true
    }
    fn is_three_of_a_kind(&self) -> bool {
        if self.is_five_of_a_kind() {
            return false;
        }
        if self.is_four_of_a_kind() {
            return false;
        }
        if self.is_full_house() {
            return false;
        }
        let ret = false;
        let mut found_3 = 'X';
        let mut found_2 = 'X';

        for c in &self.cards {
            if self.get_nb_equal_cards(c) == 3 {
                println!("found 3 '{c}'");
                found_3 = *c;
                break;
            }
        }
        // if not found 3 cards
        if found_3 == 'X' {
            return false;
        }

        for c in &self.cards {
            if found_3 != *c {
                if self.get_nb_equal_cards(c) == 2 {
                    println!("found 2 '{c}'");
                    found_2 = *c;
                    break;
                }
            }
        }

        if found_2 == 'X' {
            return true;
        }
        false
    }
    fn is_two_pairs(&self) -> bool {
        false
    }
    fn is_one_pair(&self) -> bool {
        false
    }
    fn is_high_card(&self) -> bool {
        false
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut c = self.cards.clone();
        let mut o = other.cards.clone();
        c.sort();
        o.sort();
        c == o
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
    fn test_get_type_of_hand() {
        let h1 = Hand::new("KKKKK");
        let h2 = Hand::new("AKKKK");
        let h3 = Hand::new("AKJKK");
        let h4 = Hand::new("AAKKK");
        let h5 = Hand::new("AAKKJ");
        assert!(h1.is_five_of_a_kind());
        assert!(!h2.is_five_of_a_kind());

        assert!(h1.get_nb_equal_cards(&'K') == 5);
        assert!(h2.get_nb_equal_cards(&'K') == 4);
        assert!(h3.get_nb_equal_cards(&'K') == 3);

        assert!(h2.is_four_of_a_kind());
        assert!(!h1.is_four_of_a_kind());
        assert!(!h3.is_four_of_a_kind());

        assert!(h4.is_full_house());
        assert!(!h3.is_full_house());
        assert!(!h1.is_full_house());
        assert!(!h2.is_full_house());

        assert!(h3.is_three_of_a_kind());
        assert!(!h4.is_three_of_a_kind());
        assert!(!h1.is_three_of_a_kind());
        assert!(!h2.is_three_of_a_kind());
        assert!(!h5.is_three_of_a_kind());

        let h10 = Hand::new("KAKJK");
        let h11 = Hand::new("22KK2");
        assert!(h10.is_three_of_a_kind());
        assert!(!h10.is_full_house());
        assert!(h11.is_full_house());
        assert!(!h11.is_three_of_a_kind());
    }

    #[test]
    fn test_eq_hands() {
        let h1 = Hand::new("KKKKA");
        let h2 = Hand::new("AKKKK");
        assert!(h1 == h2);

        let h3 = Hand::new("JKKKA");
        let h4 = Hand::new("AJKKK");
        println!("h1={:?}", h1);
        println!("h3={:?}", h3);
        assert_eq!(h1 == h3, false);
        assert!(h4 == h3);
        assert!(h1 != h4);
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
