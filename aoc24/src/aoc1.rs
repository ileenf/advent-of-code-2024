use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut total = 0;

    for line in read_to_string("aoc1.txt").unwrap().lines() {
        let nums: Result<Vec<i32>, _> = line.to_string().split_whitespace().map(|s| s.parse::<i32>()).collect();
        match nums {
            Ok(nums) => {
                list1.push(nums[0]);
                list2.push(nums[1]);
            }
            Err(_) => todo!(),
        }
    }
    
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        let difference = (list1[i] - list2[i]).abs();
        total += difference;
    }   

    print!("{}\n", total);

    let num_to_count = list2.iter().fold(HashMap::new(), |mut count, num| {
        *count.entry(num).or_insert(0) += 1;
        count
    });

    let mut similarity_score = 0;
    for i in 0..list1.len() {
        let num = list1[i];
        if num_to_count.contains_key(&num) {
            let score = num * num_to_count[&num];
            similarity_score += score;
        }
    }

    print!("{}\n", similarity_score);
}