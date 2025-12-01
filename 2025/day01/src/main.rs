fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Puzzle {}

fn next_l(current: i32, v: i32) -> i32 {
    let mut diff = current - v;
    while diff < 0 {
        diff = 100 + diff;
    }
    diff % 100
}

fn next_r(current: i32, v: i32) -> i32 {
    (current + v) % 100
}

// impl Puzzle {
//     fn load(&mut self, filename: &str) {
//         let file = File::open(filename).unwrap();
//         let reader = BufReader::new(file);
//         self.grid_data = Vec::new();
//         for l in reader.lines() {
//             self.grid_data.push(l.unwrap().chars().collect());
//         }
//         self.grid_width = self.grid_data[0].len();
//         self.grid_height = self.grid_data.len();
//     }
// }

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_next_func() {
        assert_eq!(next_l(50, 68), 82);
        assert_eq!(next_l(82, 30), 52);
        assert_eq!(next_r(52, 48), 0);
        assert_eq!(next_l(0, 5), 95);
        assert_eq!(next_r(95, 60), 55);
        assert_eq!(next_l(55, 55), 0);
        assert_eq!(next_l(0, 1), 99);
        assert_eq!(next_l(99, 99), 0);
        assert_eq!(next_r(0, 14), 14);
        assert_eq!(next_l(14, 82), 32);
    }
    #[test]
    fn test_next_func_more() {
        assert_eq!(next_l(0, 138), 62);
        assert_eq!(next_l(0, 38), 62);
        assert_eq!(next_l(0, 438), 62);
        assert_eq!(next_l(0, 769), 31);
    }
}
