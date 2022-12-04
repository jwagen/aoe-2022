use std::path::Path;
use std::io::BufRead;
mod day1;

pub fn get_input(p: impl AsRef<Path>) -> Vec<Option<u32>> {
    let f = std::fs::File::open(p).unwrap();
    let lines = std::io::BufReader::new(f).lines();
    lines.map(|l| l.unwrap().parse::<u32>().ok()).collect()



}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day: u32 = args[1].parse().unwrap();

    match day {
        1 => day1::part1(),
        _ => panic!("Day not implemented")
    }

    println!("day: {}", day);

    println!("Hello, world!");
}
