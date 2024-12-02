use crate::{read_file_to_big_array};

pub fn solve() {
    let input: Vec<Vec<i32>> = read_file_to_big_array("inputs/day02.txt");

    for i in 0..input.len() {
        let level = input.get(i).unwrap();
        println!("{:?}", level);
    }

    //println!("Day 02 part 1 solution: {}", input.len());
}

fn is_safe(level: Vec<i32>) -> bool {



    true
}
