use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<String, Node>,
    instructions: Vec<char>,
}

impl Network {
    fn load(filename: &str) -> Self {
        let mut n = Self {
            nodes: HashMap::new(),
            instructions: vec![],
        };

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let item = &line.unwrap();
            if index == 0 {
                n.instructions = item.chars().collect();
            } else if index == 1 {
                // line is empty
            } else {
                // AAA = (BBB, CCC)
                // ([A-Z][A-Z][A-Z]) = \(([A-Z][A-Z][A-Z]), ([A-Z][A-Z][A-Z])\)
                let re =
                    Regex::new(r"([A-Z][A-Z][A-Z]) = \(([A-Z][A-Z][A-Z]), ([A-Z][A-Z][A-Z])\)")
                        .unwrap();
                let caps = re.captures_iter(item);
                let mut name: String = "".to_string();
                let mut node: Node = Node {
                    left: "".to_string(),
                    right: "".to_string(),
                };
                for cap in caps.into_iter() {
                    if cap.get(1).is_some() {
                        name = cap[1].to_string();
                    }
                    if cap.get(2).is_some() {
                        node.left = cap[2].to_string();
                    }
                    if cap.get(3).is_some() {
                        node.right = cap[3].to_string();
                    }
                }
                n.nodes.insert(name, node);
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load() {
        let n = Network::load("example.txt");
        assert!(n.instructions.len() == 2);
        assert!(n.nodes.len() == 7);
        println!("{:?}", n);
    }
}

fn main() {
    let n = Network::load("2023/day08/input.txt");
    println!("{:?}", n)
}
