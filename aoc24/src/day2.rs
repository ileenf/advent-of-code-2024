use std::fmt::format;


fn main() {
    let input: &str = include_str!("data/day2.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn is_valid_level_difference(nums: &Vec<i32>) -> bool {
    nums.windows(2).all(|window| window[0]<window[1] && 1<=(window[1]-window[0]) && (window[1]-window[0])<=3)
    || 
    nums.windows(2).all(|window| window[0]>window[1] && 1<=(window[0]-window[1]) && (window[0]-window[1])<=3)
}

pub fn part1(input: &str) -> String {
    let mut safe_reports = 0;
    for line in input.lines() {
        let nums: Result<Vec<i32>, _> = line.to_string().split_whitespace().map(|s| s.parse::<i32>()).collect();
        match nums {
            Ok(nums) => {
                if is_valid_level_difference(&nums) {
                    safe_reports += 1;
                }
            }
            Err(_) => todo!(),
        }
    }

    format!("{}", safe_reports)

}

pub fn part2(input: &str) -> String {
    let mut safe_reports = 0;
    for line in input.lines() {
        let nums: Result<Vec<i32>, _> = line.to_string().split_whitespace().map(|s| s.parse::<i32>()).collect();
        match nums {
            Ok(nums) => {
                if is_valid_level_difference(&nums) {
                    safe_reports += 1;
                } else {
                    for i in 0..nums.len() {
                        let mut removed = nums.clone();
                        removed.remove(i);
                        if is_valid_level_difference(&removed) {
                            safe_reports += 1;
                            break;
                        }
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
    format!("{}", safe_reports)
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    assert_eq!(part1(input), "2");
    assert_eq!(part2(input), "4");
}