fn main() {}

fn binstr_to_int(bin_idx: &str) -> i32 {
    let intval = i32::from_str_radix(bin_idx, 2).unwrap();
    println!("{}", intval);
    intval
}

#[cfg(test)]
#[test]
fn bin_convert_test() {
    assert_eq!(binstr_to_int("011"), 3);
    assert_eq!(binstr_to_int("00000000000"), 0);
    assert_eq!(binstr_to_int("00000000001"), 1);
    assert_eq!(binstr_to_int("00000001101"), 13);
}
