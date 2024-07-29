
static DIGITS: [[&str; 2]; 10] = [
    ["oneight", "18"],
    ["one", "o1e"],
    ["two", "t2"],
    ["three", "3e"],
    ["four", "4"],
    ["five", "5e"],
    ["six", "6"],
    ["seven", "7"],
    ["eight", "8"],
    ["nine", "9e"]];

pub fn find_calibration_value(line: String) -> u64 {
    // TODO: proper error handling on first and last when no digits in line
    let values = line.chars().filter(|c| c.is_digit(10));
    let first = values.clone().nth(0).unwrap();
    let last = values.clone().nth_back(0).unwrap();

    format!("{first}{last}").parse().unwrap()
}

pub fn substitute_in_string(line: String) -> String {
    DIGITS.into_iter().fold(line.to_string(), |l, [k, v]| str::replace(&l[..], k, v))
}

#[test]
fn test_find_calibration_value() {
    let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let expected = [12, 38, 15, 77];
    let results = inputs.map(|l| l.to_string()).map(find_calibration_value);
    assert_eq!(results, expected)
}

#[test]
fn test_substitute_in_string() {
    let inputs = ["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four",
                           "4nineeightseven2", "zoneight234", "7pqrstsixteen", "oneight"];
    let expected = ["t219e", "823e", "abco1e23exyz", "xt21e34",
                             "49e872", "z18234", "7pqrst6teen", "18"];
    let results = inputs.map(|l| l.to_string()).map(substitute_in_string);
    assert_eq!(results, expected);

    let expected_int = [29, 83, 13, 24, 42, 14, 76, 18];
    let results_int = inputs.map(|l| find_calibration_value(substitute_in_string(l.to_string())));
    assert_eq!(results_int, expected_int);
}