use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &'static str =
    "/Users/jamesyeap/Developer/rust/rust-exercises-ferrous-systems/crates/iterators/numbers.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    // Write your iterator chain here
    let sum_of_odd_numbers: i32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .filter(|num| num % 2 != 0)
        .fold(0, |acc, x| acc + x);

    assert_eq!(sum_of_odd_numbers, 31);
    Ok(())
}
