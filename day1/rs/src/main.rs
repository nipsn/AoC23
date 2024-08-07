use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rs::{find_calibration_value, substitute_in_string};

fn main() {
    let mut res1: u64 = 0;
    let mut res2: u64 = 0;
    if let Ok(lines) = read_lines("../input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            res1 += find_calibration_value(line.clone());
            res2 += find_calibration_value(substitute_in_string(line));
        }
    }
    println!("Part 1: {res1}\nPart 2: {res2}");
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
