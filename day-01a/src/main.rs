use std::fs;

fn main() {
    let input = fs::read_to_string("adventofcode.com_2022_day_1_input.txt")
        .expect("File does not exist");

    let max = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
    println!("{max}");
}
