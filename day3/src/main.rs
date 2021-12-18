use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // read file
    let mut filename = "input.txt";
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        2 => {
            filename = &args[1];
        }
        _ => {
            println!("using default filename");
        }
    }
    println!("reading from {}", filename);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec: Vec<Vec<i32>> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        vec.push(binstr_to_vec(&line));
    }

    let mut gamma_vec: Vec<usize> = Vec::new();
    let mut epsilon_vec_str: Vec<char> = Vec::new();
    let mut gamma_vec_str: Vec<char> = Vec::new();

    for (i, v) in vec.iter().enumerate() {
        println!("{}: {}/{}/{}/{}/{}", i, v[0], v[1], v[2], v[3], v[4]);
        // find most common bit in column and store result for each column
        // init vector once
        if gamma_vec.len() != v.len() {
            for (_j, _v2) in v.iter().enumerate() {
                gamma_vec.push(0);
            }
        }
        // count '1' in each column
        for (j, _v2) in v.iter().enumerate() {
            if v[j] == 1 {
                gamma_vec[j] += 1;
            }
        }
    }

    // build gamma and epsilon strings
    for (k, _gamma_v) in gamma_vec.iter().enumerate() {
        if gamma_vec[k] > (vec.len() / 2) {
            println!("column {}: 1 is most common", k);
            epsilon_vec_str.push('0');
            gamma_vec_str.push('1');
        } else {
            println!("column {}: 0 is most common", k);
            epsilon_vec_str.push('1');
            gamma_vec_str.push('0');
        }
    }
    let gamma = gamma_vec_str.iter().collect::<String>();
    let espilon = epsilon_vec_str.iter().collect::<String>();
    println!("gamma = {}", gamma);
    println!("espilon = {}", espilon);

    println!(
        "power consumption = {}",
        binstr_to_int(&gamma) * binstr_to_int(&espilon)
    );
}

fn binstr_to_vec(bin_val: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for (_index, c) in bin_val.chars().enumerate() {
        match c {
            '0' => vec.push(0),
            '1' => vec.push(1),
            _ => (),
        }
    }
    vec
}

fn binstr_to_int(bin_val: &str) -> i32 {
    let intval = i32::from_str_radix(bin_val, 2).unwrap();
    println!("{}", intval);
    intval
}

#[cfg(test)]
#[test]
fn bin_int_convert_test() {
    assert_eq!(binstr_to_int("011"), 3);
    assert_eq!(binstr_to_int("00000000000"), 0);
    assert_eq!(binstr_to_int("00000000001"), 1);
    assert_eq!(binstr_to_int("00000001101"), 13);
}
#[test]
fn bin_vec_convert_test_1() {
    let v1 = binstr_to_vec("001");
    assert_eq!(v1.len(), 3);
    assert_eq!(v1[0], 0);
    assert_eq!(v1[1], 0);
    assert_eq!(v1[2], 1);
}
#[test]
fn bin_vec_convert_test_2() {
    let v1 = binstr_to_vec("11001");
    assert_eq!(v1.len(), 5);
    assert_eq!(v1[0], 1);
    assert_eq!(v1[1], 1);
    assert_eq!(v1[2], 0);
    assert_eq!(v1[3], 0);
    assert_eq!(v1[4], 1);
}
