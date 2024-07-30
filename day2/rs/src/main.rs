use rs::{read_lines, valid_game, get_power, Hand};

fn main() {
    if let Ok(lines) = read_lines("../input") {
        let max_vals = Hand {
            red: 12,
            green: 13,
            blue: 14,
        };

        let res1: usize = lines.flatten()
            .map(|l| valid_game(l, max_vals))
            .filter(|(_, v)| *v)
            .map(|(id, v)| id)
            .sum();

        println!("Part 1: {res1}")
    }

    // TODO: should be reading just once but oh well...
    if let Ok(lines) = read_lines("../input") {
        let res2: usize = lines.flatten()
            .map(get_power)
            .sum();
        println!("Part 2: {res2}")
    }
}
