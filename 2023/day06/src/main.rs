use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
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
                        let val: u32 = n.parse::<u32>().unwrap();
                        times.push(val);
                    }
                }
            } else {
                let idx = item.find("Distance:").unwrap() + "Distance:".len() + 1;
                let v = item[idx..].split(' ');
                for n in v {
                    if !n.trim().is_empty() {
                        println!("distance={n}");
                        let val: u32 = n.parse::<u32>().unwrap();
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
}

fn number_of_ways_to_beat_the_record(filename: &str) -> u32 {
    let mut rd = RacesDocument::new();
    rd.load(filename);

    let mut how_many_to_beat_the_record = 1;

    for r in rd.races {
        let mut how_many_for_1_race = 0;
        for hold_duration in 1..r.time {
            let speed = hold_duration;
            let remaining_time = (r.time - hold_duration);
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
    fn test_number_of_ways_to_beat_the_record() {
        assert_eq!(number_of_ways_to_beat_the_record("example.txt"), 288);
    }
}

fn main() {
    println!(
        "part1 = {}",
        number_of_ways_to_beat_the_record("2023/day06/input.txt")
    );
}
