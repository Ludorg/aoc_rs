#[derive(Debug)]
struct Line {
    values: Vec<i32>,
}

#[derive(Debug)]
struct Report {
    lines: Vec<Line>,
}

impl Line {
    fn new(s: &str) -> Self {
        let mut values = vec![];
        let val_str: Vec<_> = s.split(' ').collect();
        for v in val_str {
            values.push(v.parse().unwrap());
        }
        Self { values }
    }
}

fn extrapolate_step(vec_in: &Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];

    for i in 1..vec_in.len() {
        let d = vec_in[i] - vec_in[i - 1];
        ret.push(d);
    }
    ret
}

fn is_sequence_zero(vec_in: Vec<i32>) -> bool {
    for i in vec_in {
        if i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate_step_seq_0() {
        let l = Line::new("0 3 6 9 12 15");

        let mut s0 = extrapolate_step(&l.values);
        assert!(s0.len() == 5);
        println!("{:?}", s0);
        assert!(is_sequence_zero(&s0) == false);

        let mut s1 = extrapolate_step(&s0);
        println!("{:?}", s1);
        assert!(s1.len() == 0);
        assert!(is_sequence_zero(s1) == true);
    }

    #[test]
    fn test_load_line() {
        let l = Line::new("0 3 6 9 12 15");
        assert!(l.values.len() == 6);
        println!("{:?}", l);
    }
}

fn main() {
    println!("Hello, world!");
}
