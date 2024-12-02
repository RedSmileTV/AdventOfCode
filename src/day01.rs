use crate::read_file;

pub fn solve() {
    // Getting the input
    let input: Vec<i32> = read_file("inputs/day01.txt");
    let left: Vec<i32> = split_input(input.clone()).0;
    let right: Vec<i32> = split_input(input.clone()).1;

    // Solving Part 1
    let differences: Vec<i32> = get_differences(sort_ascending(left.clone()), sort_ascending(right.clone()));
    println!("Day 01 part 1 solution: {}", sum(differences));

    // Solving Part 2
    let similarities: Vec<i32> = find_similarities(left.clone(), right.clone());
    println!("Day 01 part 2 solution: {}", sum(similarities))
}

fn find_similarities(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut similarities: Vec<i32> = Vec::new();
    let mut count: i32 = 0;
    for i in 0..left.len() {
        for j in 0..right.len() {
            if left[i] == right[j] {
                count += 1;
            }
        }
        similarities.push(left[i] * count.clone());
        count = 0
    }
    similarities
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