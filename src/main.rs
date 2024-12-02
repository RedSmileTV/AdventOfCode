use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Read};

mod day01;
mod day02;

fn main() {
    //day01::solve();
    day02::solve();
}

pub fn read_file_to_array(path: &str) -> Vec<i32> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub fn read_file_to_hashmap(path: &str) -> HashMap<u32, Vec<i32>> {
    let mut map: HashMap<u32, Vec<i32>> = HashMap::new();
    let file = File::open(path).expect("File not found");
    let reader = std::io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        map.insert(index as u32, numbers);
    }
    map
}