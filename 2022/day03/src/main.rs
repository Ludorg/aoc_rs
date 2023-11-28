use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part2() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut val: u32 = 0;

    let mut current_group: Vec<String> = vec![String::new(); 3];

    for (index, line) in reader.lines().enumerate() {
        // input file contains 3*x lines (300 for mine)
        current_group[index % 3] = line.unwrap();
        if index % 3 == 2 {
            val += compute_priority(find_common_item(&current_group)) as u32;
        }
    }

    println!("part2 result={}", val);
}

fn find_common_item(group: &Vec<String>) -> char {
    for c in group[0].chars() {
        if group[1].find(c) != None {
            if group[2].find(c) != None {
                return c;
            }
        }
    }
    ' '
}

fn part1() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut val: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        val += compute_priority(find_shared_item(split_rucksack(line))) as u32;
    }
    println!("part1 result={}", val);
}

struct Rucksack {
    pub(crate) c1: String,
    pub(crate) c2: String,
}

fn split_rucksack(data: String) -> Rucksack {
    let (comp1, comp2) = data.split_at(data.len() / 2);
    // println!("{} / {}", comp1, comp2);
    let sack: Rucksack = Rucksack {
        c1: comp1.to_string(),
        c2: comp2.to_string(),
    };
    sack
}

fn find_shared_item(sack: Rucksack) -> char {
    for c in sack.c1.chars() {
        if sack.c2.find(c) != None {
            return c;
        }
    }
    ' '
}

fn compute_priority(c: char) -> u8 {
    // let mut ret : i32 = 'a'
    let offset: u8 = if c.is_uppercase() { 27 } else { 1 };
    let diff: u8 = if c.is_uppercase() {
        u8::try_from('A').unwrap()
    } else {
        u8::try_from('a').unwrap()
    };

    u8::try_from(c).unwrap() - diff + offset
}

#[cfg(test)]
mod tests {
    use super::compute_priority;
    use super::find_shared_item;
    use super::split_rucksack;
    use super::Rucksack;
    #[test]
    fn part1_split1() {
        let s: Rucksack = split_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(s.c1, "vJrwpWtwJgWr");
        assert_eq!(s.c2, "hcsFMMfFFhFp");
    }
    #[test]
    fn part1_split2() {
        let s: Rucksack = split_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string());
        assert_eq!(s.c1, "jqHRNqRjqzjGDLGL");
        assert_eq!(s.c2, "rsFMfFZSrLrFZsSL");
    }
    #[test]
    fn part1_split3() {
        let s: Rucksack = split_rucksack("PmmdzqPrVvPwwTWBwg".to_string());
        assert_eq!(s.c1, "PmmdzqPrV");
        assert_eq!(s.c2, "vPwwTWBwg");
    }
    #[test]
    fn part1_shared1() {
        assert_eq!(
            find_shared_item(split_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp".to_string())),
            'p'
        );
    }
    #[test]
    fn part1_shared2() {
        assert_eq!(
            find_shared_item(split_rucksack(
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()
            )),
            'L'
        );
    }
    #[test]
    fn part1_shared3() {
        assert_eq!(
            find_shared_item(split_rucksack("PmmdzqPrVvPwwTWBwg".to_string())),
            'P'
        );
    }
    #[test]
    fn part1_shared4() {
        assert_eq!(
            find_shared_item(split_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string())),
            'v'
        );
    }
    #[test]
    fn part1_shared5() {
        assert_eq!(
            find_shared_item(split_rucksack("ttgJtRGJQctTZtZT".to_string())),
            't'
        );
    }
    #[test]
    fn part1_shared6() {
        assert_eq!(
            find_shared_item(split_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw".to_string())),
            's'
        );
    }

    #[test]
    fn part1_priority1() {
        assert_eq!(compute_priority('a'), 1);
    }

    #[test]
    fn part1_priority2() {
        assert_eq!(compute_priority('A'), 27);
    }
    #[test]
    fn part1_priority3() {
        assert_eq!(compute_priority('p'), 16);
    }
    #[test]
    fn part1_priority4() {
        assert_eq!(compute_priority('L'), 38);
    }
    #[test]
    fn part1_priority5() {
        assert_eq!(compute_priority('P'), 42);
    }
    #[test]
    fn part1_priority6() {
        assert_eq!(compute_priority('v'), 22);
    }
    #[test]
    fn part1_priority7() {
        assert_eq!(compute_priority('t'), 20);
    }
    #[test]
    fn part1_priority8() {
        assert_eq!(compute_priority('s'), 19);
    }
}
