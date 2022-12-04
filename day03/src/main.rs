use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // vector to store read data
    //let mut rucksacks: Vec<String> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        find_shared_item(split_rucksack(line));
    }
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

#[cfg(test)]
mod tests {
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
}
