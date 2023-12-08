use std::cmp::Ordering;

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

    fn is_five_of_a_kind(&self) -> (bool, char) {
        let first = &self.cards[0];
        for c in &self.cards {
            if c != first {
                return (false, 'X');
            }
        }
        (true, *first)
    }

    fn is_four_of_a_kind(&self) -> (bool, char) {
        if self.is_five_of_a_kind().0 {
            return (false, 'X');
        }
        for c in &self.cards {
            if self.get_nb_equal_cards(c) == 4 {
                return (true, *c);
            }
        }
        (false, 'X')
    }
    fn is_full_house(&self) -> (bool, char, char) {
        if self.is_four_of_a_kind().0 {
            return (false, 'X', 'X');
        }

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
            return (false, 'X', 'X');
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
            return (false, 'X', 'X');
        }

        (true, found_3, found_2)
    }

    fn is_three_of_a_kind(&self) -> (bool, char) {
        if self.is_full_house().0 {
            return (false, 'X');
        }

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
            return (false, 'X');
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
            return (true, found_3);
        }
        (false, 'X')
    }

    fn pair_check(&self) -> (char, char) {
        let mut found_2_1 = 'X';
        let mut found_2_2 = 'X';

        for c in &self.cards {
            if self.get_nb_equal_cards(c) == 2 {
                println!("found 2_1 '{c}'");
                found_2_1 = *c;
                break;
            }
        }
        // if not found 2 cards
        if found_2_1 == 'X' {
            return (found_2_1, found_2_2);
        }

        for c in &self.cards {
            if found_2_1 != *c {
                if self.get_nb_equal_cards(c) == 2 {
                    println!("found 2_2 '{c}'");
                    found_2_2 = *c;
                    break;
                }
            }
        }
        return (found_2_1, found_2_2);
    }

    fn is_two_pairs(&self) -> (bool, char, char) {
        if self.is_three_of_a_kind().0 {
            return (false, 'X', 'X');
        }

        let (found_2_1, found_2_2) = self.pair_check();
        if found_2_2 == 'X' {
            return (false, 'X', 'X');
        }
        if found_2_2 == found_2_1 {
            return (false, 'X', 'X');
        }
        (true, found_2_1, found_2_2)
    }

    fn is_one_pair(&self) -> (bool, char) {
        if self.is_two_pairs().0 {
            println!("B");
            return (false, 'X');
        }

        let (found_2_1, found_2_2) = self.pair_check();
        println!("C");

        if found_2_2 == found_2_1 {
            return (false, 'X');
        }
        if found_2_2 == 'X' {
            return (true, found_2_1);
        }
        (false, 'X')
    }

    // TODO : fixme
    fn is_high_card(&self) -> (bool, char) {
        if self.is_five_of_a_kind().0 {
            return (false, 'X');
        }
        if self.is_three_of_a_kind().0 {
            return (false, 'X');
        }
        if self.is_two_pairs().0 {
            return (false, 'X');
        }
        if self.is_one_pair().0 {
            return (false, 'X');
        }

        let mut max = 0;
        let mut idx_curr = 0;
        let mut idx_max = 0;
        for c in &self.cards {
            let val = char_to_card_value(*c);
            if val > max {
                idx_max = idx_curr;
                max = val;
            }
            idx_curr += 1;
        }

        (true, self.cards[idx_max])
    }

    fn get_score(&self) -> u32 {
        let t = self.is_high_card();
        if t.0 {
            return 1 * char_to_card_value(t.1); // 1..13
        }

        let t1 = self.is_one_pair();
        if t1.0 {
            return 13 * char_to_card_value(t1.1) + 1; // 14..170
        }

        let t2 = self.is_two_pairs();
        println!("{:?}", t2);
        if t2.0 {
            let v1 = 170 * char_to_card_value(t2.1) + 1; // 171..2211
            let v2 = 170 * char_to_card_value(t2.2) + 1; // 171..2211
            return v2 + v1;
        }

        9999999
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

// impl Ord for Hand {
//     fn cmp(&self, other: &Self) -> Ordering {}
// }

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
    fn test_get_score() {
        let h1 = Hand::new("3JKT2");
        let h2 = Hand::new("4JAT2");
        assert_eq!(h1.get_score(), 12);
        assert_eq!(h2.get_score(), 13);

        let h3 = Hand::new("33KT2");
        let h4 = Hand::new("KK4T2");
        let h5 = Hand::new("K4T2K");
        let h6 = Hand::new("K4KT2");
        assert_eq!(h3.get_score(), 27);
        assert_eq!(h4.get_score(), 157);
        assert_eq!(h5.get_score(), h4.get_score());
        assert_eq!(h6.get_score(), h4.get_score());

        let h7 = Hand::new("33KK7");
        assert!(h7.is_two_pairs().0);
        assert!(!h7.is_one_pair().0);

        assert!(!h7.is_one_pair().0);
        assert!(!h7.is_high_card().0);

        assert_eq!(h7.is_two_pairs().1, '3');
        assert_eq!(h7.is_two_pairs().2, 'K');
        let h8 = Hand::new("KKTT2");
        let h9 = Hand::new("AATT2");
        let h10 = Hand::new("TTAA4");

        println!("h7 {}", h7.get_score());
        println!("h8 {}", h8.get_score());

        assert!(h8.get_score() > h7.get_score());
        assert!(h9.get_score() > h7.get_score());
        assert!(h9.get_score() == h10.get_score());
    }

    #[test]
    fn test_get_type_of_hand() {
        let h1 = Hand::new("KKKKK");
        let h2 = Hand::new("AKKKK");
        let h3 = Hand::new("AKJKK");
        let h4 = Hand::new("AAKKK");
        let h5 = Hand::new("AAKKJ");

        let h10 = Hand::new("KAKJK");
        let h11 = Hand::new("22KK2");
        let h15 = Hand::new("TT332");

        let h20 = Hand::new("AAKJ2");
        let h21 = Hand::new("AAK42");
        let h22 = Hand::new("AAKK2");

        let h30 = Hand::new("AJKT2");
        let h31 = Hand::new("3JKT2");

        assert!(h1.is_five_of_a_kind().0);
        assert!(!h1.is_high_card().0);

        assert_eq!(h1.is_five_of_a_kind().1, 'K');
        assert!(!h2.is_five_of_a_kind().0);
        assert_eq!(h2.is_five_of_a_kind().1, 'X');

        assert!(h1.get_nb_equal_cards(&'K') == 5);
        assert!(h2.get_nb_equal_cards(&'K') == 4);
        assert!(h3.get_nb_equal_cards(&'K') == 3);

        assert!(h2.is_four_of_a_kind().0);
        assert_eq!(h2.is_four_of_a_kind().1, 'K');
        assert!(!h1.is_four_of_a_kind().0);
        assert!(!h3.is_four_of_a_kind().0);

        assert!(h4.is_full_house().0);
        assert_eq!(h4.is_full_house().1, 'K');
        assert_eq!(h4.is_full_house().2, 'A');
        assert!(!h3.is_full_house().0);
        assert!(!h1.is_full_house().0);
        assert!(!h2.is_full_house().0);

        assert!(h3.is_three_of_a_kind().0);
        assert_eq!(h3.is_three_of_a_kind().1, 'K');
        assert!(!h4.is_three_of_a_kind().0);
        assert!(!h1.is_three_of_a_kind().0);
        assert!(!h2.is_three_of_a_kind().0);
        assert!(!h5.is_three_of_a_kind().0);

        assert!(h10.is_three_of_a_kind().0);
        assert_eq!(h10.is_three_of_a_kind().1, 'K');
        assert!(!h10.is_full_house().0);
        assert!(h11.is_full_house().0);
        assert_eq!(h11.is_full_house().1, '2');
        assert_eq!(h11.is_full_house().2, 'K');
        assert!(!h11.is_three_of_a_kind().0);

        assert!(h5.is_two_pairs().0);
        assert_eq!(h5.is_two_pairs().1, 'A');
        assert_eq!(h5.is_two_pairs().2, 'K');
        assert!(h15.is_two_pairs().0);
        assert_eq!(h15.is_two_pairs().1, 'T');
        assert_eq!(h15.is_two_pairs().2, '3');
        assert!(!h10.is_two_pairs().0);
        assert!(!h11.is_two_pairs().0);
        assert!(!h20.is_two_pairs().0);

        assert!(h20.is_one_pair().0);
        assert!(h21.is_one_pair().0);
        assert!(!h22.is_one_pair().0);
        assert!(!h2.is_one_pair().0);

        assert!(h22.is_two_pairs().0);
        assert!(!h21.is_high_card().0);
        assert!(h30.is_high_card().0);
        assert_eq!(h30.is_high_card().1, 'A');
        assert_eq!(h31.is_high_card().1, 'K');
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
