use std::fs;

fn main() {
    let input = fs::read_to_string("adventofcode.com_2022_day_1_input.txt")
        .expect("File does not exist");

    let mut ordered: Vec<u32> = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    ordered.sort();

    println!("{:?}", ordered.get(ordered.len()-3..ordered.len()).unwrap().iter().sum::<u32>());
}
