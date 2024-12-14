use regex::Regex;
use std::fs;

use crate::utils::read_lines;

pub fn solve() {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    let text: String = fs::read_to_string("./inputs/input_3.txt").unwrap();
    let result = re.find_iter(&text).map(|x| x.as_str()).fold(0, |acc, x| {
        let split: Vec<&str> = x.split(|c| c == '(' || c == ',' || c == ')').collect();
        let first: i32 = split[1].parse().unwrap();
        let second: i32 = split[2].parse().unwrap();
        acc + first * second
    });
    println!("{}", result);
}

pub fn solve2() {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let do_string = "do()";
    let dont_string = "don't()";
    let text: String = fs::read_to_string("./inputs/input_3.txt").unwrap();
    let mut enabled = true;
    let result = re.find_iter(&text).map(|x| x.as_str()).fold(0, |acc, x| {
        if x == do_string {
            enabled = true;
            return acc;
        } else if x == dont_string {
            enabled = false;
            return acc;
        }
        if !enabled {
            return acc;
        }
        let split: Vec<&str> = x.split(|c| c == '(' || c == ',' || c == ')').collect();
        let first: i32 = split[1].parse().unwrap();
        let second: i32 = split[2].parse().unwrap();
        acc + first * second
    });
    println!("{}", result);
}
