use std::fs::File;
use std::io::Read;

mod day01;

fn main() {
    day01::solve();
}

pub fn read_file(path: &str) -> Vec<i32> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
