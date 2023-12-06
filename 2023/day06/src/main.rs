#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn number_of_ways_to_beat_the_record(filemane: &str) -> u32 {
    288
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
