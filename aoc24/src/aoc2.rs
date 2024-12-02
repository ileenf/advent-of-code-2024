use std::fs::read_to_string;

fn main() {
    let mut safe_reports = 0;
    for line in read_to_string("aoc2.txt").unwrap().lines() {
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

    println!("{}", safe_reports);
}

fn is_valid_level_difference(nums: &Vec<i32>) -> bool {
    nums.windows(2).all(|window| window[0]<window[1] && 1<=(window[1]-window[0]) && (window[1]-window[0])<=3)
    || 
    nums.windows(2).all(|window| window[0]>window[1] && 1<=(window[0]-window[1]) && (window[0]-window[1])<=3)
}