use crate::read_file;

pub fn solve() {
    let input: Vec<i32> = read_file("inputs/part1.txt");
    let mut left = split_input(input.clone()).0;
    let mut right = split_input(input.clone()).1;

    left = sort_ascending(left);
    right = sort_ascending(right);

    let differences = get_differences(left, right);

    println!("Day 01 solution: {}", sum(differences));
}

fn get_differences(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new();
    for i in 0..left.len() {
        let diff = (right[i] - left[i]).abs();
        differences.push(diff);
    }
    differences
}

fn sum(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..input.len() {
        sum += input[i];
    }
    sum
}

fn sort_ascending(input: Vec<i32>) -> Vec<i32> {
    let mut sorted = input.clone();
    sorted.sort();
    sorted
}

fn split_input(input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for i in 0..input.len() {
        if i % 2 == 0 {
            first.push(input[i]);
        } else {
            second.push(input[i]);
        }
    }
    (first, second)
}