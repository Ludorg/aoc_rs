use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "2023/day02/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut id_sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let item = &line.unwrap();
        let g = read_game(item);
        let b = is_game_possible(&g);
        if b == true {
            id_sum += g.id;
        }
        println!("game {g:?} is possible = {b}");
    }
    println!("part 1 = {id_sum}");
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn read_game(s: &str) -> Game {
    let id = read_id(s);
    let sets = read_sets(s);
    Game { id: id, sets: sets }
}

fn read_id(s: &str) -> u32 {
    let re = Regex::new(r"Game (\d*):").unwrap();
    let caps = re.captures(s).unwrap();
    caps[1].parse::<u32>().unwrap()
}

fn read_sets(s: &str) -> Vec<Set> {
    let v = split_sets(s);
    let mut sets = Vec::new();
    for s in v {
        let set = string_to_set(s);
        //println!("in string {set:?}");
        sets.push(set);
    }
    sets
}

fn split_sets(s: &str) -> Vec<&str> {
    let mut v = Vec::new();
    let start_pos = s.find(": ").unwrap() + 2;
    let sets = s[start_pos..].split(";");
    for set in sets {
        //println!("set = {}", set);
        v.push(set);
    }
    v
}

fn string_to_set(s: &str) -> Set {
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    // group1 is red, group2 is green, group3 is blue
    let re = Regex::new(r"(\d*) red|(\d*) green|(\d*) blue").unwrap();
    let caps = re.captures_iter(s);

    for cap in caps.into_iter() {
        if cap.get(1) != None {
            set.red = cap[1].parse::<u32>().unwrap();
        }
        if cap.get(2) != None {
            set.green = cap[2].parse::<u32>().unwrap();
        }
        if cap.get(3) != None {
            set.blue = cap[3].parse::<u32>().unwrap();
        }
    }

    set
}

// true if less than 12 red cubes, 13 green cubes, and 14 blue cubes
fn is_game_possible(g: &Game) -> bool {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for s in &g.sets {
        red += s.red;
        green += s.green;
        blue += s.blue;
    }
    (red <= 12) & (green <= 13) & (blue <= 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_possible() {
        assert_eq!(
            is_game_possible(&read_game(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 7 green"
            )),
            true
        );

        assert_eq!(
            is_game_possible(&read_game(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            )),
            true
        );
        assert_eq!(
            is_game_possible(&read_game(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            )),
            true
        );
        assert_eq!(
            is_game_possible(&read_game(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )),
            false
        );
        assert_eq!(
            is_game_possible(&read_game(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            )),
            false
        );
        assert_eq!(
            is_game_possible(&read_game(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            true
        );
    }

    #[test]
    fn test_read_id() {
        assert_eq!(
            read_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
    }
    #[test]
    fn test_read_sets() {
        let sets = read_sets("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        println!("{sets:?}");
    }

    #[test]
    fn test_read_sets_0() {
        let v = split_sets("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        println!("v={v:?}");
        let s1 = string_to_set("1 red, 2 green, 6 blue");
        println!("{s1:?}");
        let s1 = string_to_set("7 green, 8 blue");
        println!("{s1:?}");
        let s1 = string_to_set("18 blue, 17 red");
        println!("{s1:?}");

        for s in v {
            let s2 = string_to_set(s);
            println!("in string {s2:?}");
        }
    }

    #[test]
    fn test_read_game() {
        let g1 = Game {
            id: 7,
            sets: vec![Set {
                red: 4,
                green: 5,
                blue: 6,
            }],
        };
        println!("{g1:?}");
        let s1 = Set {
            red: 1,
            green: 2,
            blue: 3,
        };
        println!("{s1:?}");

        let g2 = read_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 27 green");

        assert_eq!(g2.id, 1);
        assert_eq!(g2.sets[0].blue, 3);
        assert_eq!(g2.sets[0].red, 4);
        assert_eq!(g2.sets[0].green, 0);

        assert_eq!(g2.sets[1].blue, 6);
        assert_eq!(g2.sets[1].red, 1);
        assert_eq!(g2.sets[1].green, 2);

        assert_eq!(g2.sets[2].blue, 0);
        assert_eq!(g2.sets[2].red, 0);
        assert_eq!(g2.sets[2].green, 27);
    }
}
