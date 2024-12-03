use std::i32;
use regex::Regex;

fn main() {
    let input: &str = include_str!("data/day3.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

pub fn part1(input: &str) -> String {
    let mut total = 0;
        total += input.split("mul(")
        .filter_map(|s| s.split(")").next())
        .filter_map(|elements| { 
            let parts: Vec<_> = elements.split(",").collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    Some(x*y)
                } else {
                    None
                }
            } else {
                None
            }
        }).sum::<i32>();

    format!("{}", total)
}

pub fn part2(input: &str) -> String {
    let mut total = 0;
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    for matched in pattern.find_iter(input) {
        let str_match = matched.as_str();
        if str_match.starts_with("mul") && enabled {
            let parts: Vec<_> = str_match.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",").collect();
            if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                total += x*y;
            }
            continue;
        }

        if str_match.starts_with("don't") {
            enabled = false;
            continue;
        } 

        if str_match.starts_with("do") {
            enabled = true;
        }
    }
    format!("{}", total)
}

#[cfg(test)]
#[test]
fn test_solution() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part1(input), "161");
    assert_eq!(part2(input2), "48");
}