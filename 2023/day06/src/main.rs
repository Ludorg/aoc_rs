use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Debug)]
struct RacesDocument {
    races: Vec<Race>,
}

impl RacesDocument {
    fn new() -> Self {
        Self { races: vec![] }
    }
    fn load(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut times = vec![];
        let mut distances = vec![];
        for (index, line) in reader.lines().enumerate() {
            let item = line.unwrap();

            if index == 0 {
                let idx = item.find("Time:").unwrap() + "Time:".len() + 1;
                let v = item[idx..].split(' ');
                for n in v {
                    if !n.trim().is_empty() {
                        println!("time={n}");
                        let val = n.parse::<u64>().unwrap();
                        times.push(val);
                    }
                }
            } else {
                let idx = item.find("Distance:").unwrap() + "Distance:".len() + 1;
                let v = item[idx..].split(' ');
                for n in v {
                    if !n.trim().is_empty() {
                        println!("distance={n}");
                        let val = n.parse::<u64>().unwrap();
                        distances.push(val);
                    }
                }
            }
            for n in 0..distances.len() {
                let r = Race {
                    time: times[n],
                    distance: distances[n],
                };
                self.races.push(r);
            }
        }
    }

    fn load2(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut time = 0;
        let mut distance = 0;
        for (index, line) in reader.lines().enumerate() {
            let item = line.unwrap();

            if index == 0 {
                let idx = item.find("Time:").unwrap() + "Time:".len() + 1;
                time = item[idx..].replace(" ", "").parse::<u64>().unwrap();
            } else {
                let idx = item.find("Distance:").unwrap() + "Distance:".len() + 1;
                distance = item[idx..].replace(" ", "").parse::<u64>().unwrap();
            }
        }
        let r = Race {
            time: time,
            distance: distance,
        };
        println!("{:?}", r);
        self.races.push(r);
    }
}

fn number_of_ways_to_beat_the_record_1(filename: &str) -> u64 {
    let mut rd = RacesDocument::new();
    rd.load(filename);
    number_of_ways_to_beat_the_record(rd)
}

fn number_of_ways_to_beat_the_record_2(filename: &str) -> u64 {
    let mut rd = RacesDocument::new();
    rd.load2(filename);
    number_of_ways_to_beat_the_record(rd)
}

fn number_of_ways_to_beat_the_record(rd: RacesDocument) -> u64 {
    let mut how_many_to_beat_the_record = 1;

    for r in rd.races {
        let mut how_many_for_1_race = 0;
        for hold_duration in 1..r.time {
            let speed = hold_duration;
            let remaining_time = r.time - hold_duration;
            let distance = speed * remaining_time;
            if distance > r.distance {
                how_many_for_1_race += 1;
            }
        }
        println!("1 race {how_many_for_1_race}");
        how_many_to_beat_the_record *= how_many_for_1_race;
    }
    how_many_to_beat_the_record
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_ways_to_beat_the_record_1() {
        assert_eq!(number_of_ways_to_beat_the_record_1("example.txt"), 288);
    }
    #[test]
    fn test_number_of_ways_to_beat_the_record_2() {
        assert_eq!(number_of_ways_to_beat_the_record_2("example.txt"), 71503);
    }
}

fn main() {
    println!(
        "part1 = {}",
        number_of_ways_to_beat_the_record_1("2023/day06/input.txt")
    );

    println!(
        "part2 = {}",
        number_of_ways_to_beat_the_record_2("2023/day06/input.txt")
    );
}
