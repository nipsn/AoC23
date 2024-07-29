pub fn find_calibration_value(line: &str) -> u64 {
    // TODO: proper error handling on first and last when no digits in line
    let binding = line.to_string();
    let values = binding.chars().filter(|c| c.is_digit(10));
    let first = values.clone().nth(0).unwrap();
    let last = values.clone().nth_back(0).unwrap();

    format!("{first}{last}").parse().unwrap()
}

#[test]
fn test_find_calibration_value() {
    let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let expected = [12, 38, 15, 77];
    let results = inputs.map(find_calibration_value);
    assert_eq!(results, expected)
}
