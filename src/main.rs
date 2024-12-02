use std::fs::File;
use std::io::{BufRead, BufReader, Read};

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

pub fn read_file_to_big_array(path: &str) -> Vec<Vec<i32>> {
    let mut big_array: Vec<Vec<i32>> = Vec::new();
    let file: File = File::open(path).expect("File not found");
    let reader: BufReader<File> = BufReader::new(file);

    for i in 0..reader.lines() {
        let line = reader.lines().nth(i).unwrap().unwrap();
        let array: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        big_array.push(array);
    }
    big_array



}