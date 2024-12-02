use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("data/day1.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
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

    (list1, list2)
}

pub fn part1(input: &str) -> String {
    let mut total = 0;
    let (list1, list2) = parse_input(input);

    for i in 0..list1.len() {
        let difference = (list1[i] - list2[i]).abs();
        total += difference;
    }   

    format!("{}", total)
}

pub fn part2(input: &str) -> String {
    let mut similarity_score = 0;
    let (list1, list2) = parse_input(input);

    let num_to_count = list2.iter().fold(HashMap::new(), |mut count, num| {
        *count.entry(num).or_insert(0) += 1;
        count
    });

    for i in 0..list1.len() {
        let num = list1[i];
        if num_to_count.contains_key(&num) {
            let score = num * num_to_count[&num];
            similarity_score += score;
        }
    }
    format!("{}", similarity_score)
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3";

    assert_eq!(part1(input), "11");
    assert_eq!(part2(input), "31");
}