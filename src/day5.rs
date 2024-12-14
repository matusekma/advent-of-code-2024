use regex::Regex;
use std::{cmp::Ordering, fs};

use crate::utils::read_lines;

pub fn solve() {
    if let Ok(lines) = read_lines("./inputs/input_5.txt") {
        let mut sum_right_order_middle_item = 0;
        let lines: Vec<String> = lines.flatten().collect();
        let mut rules: Vec<(i32, i32)> = Vec::new();
        let mut pages: Vec<Vec<i32>> = Vec::new();
        let re = Regex::new(r"\d+\|\d+").unwrap();
        lines.iter().for_each(|line| {
            if (re.is_match(line)) {
                let split: Vec<&str> = line.split('|').collect();
                let first: i32 = split[0].parse().unwrap();
                let second: i32 = split[1].parse().unwrap();
                rules.push((first, second));
            } else if !(line == "") {
                let nums: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
                pages.push(nums);
            }
        });

        pages.iter().for_each(|page| {
            let mut in_right_order = true;
            let copy_page = page.to_vec();
            let mut page_mut = page.to_vec();
            page_mut.sort_by(|a, b| {
                let ab_rule = rules.iter().find(|rule| {
                    rule.0 == *a && rule.1 == *b
                });
                let ba_rule = rules.iter().find(|rule| {
                    rule.0 == *b && rule.1 == *a
                });
                if(ab_rule.is_some()) {
                    return Ordering::Less;
                } else if(ba_rule.is_some()) {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            });
            for i in 0..page.len() {
                if page_mut[i] != copy_page[i] {
                    in_right_order = false;
                    break;
                }
            }
            if in_right_order {
                println!("{:?}", copy_page);
                sum_right_order_middle_item += page[page.len() / 2];
            }
        });

        println!("{}", sum_right_order_middle_item);
    }
}
pub fn solve2() {
    if let Ok(lines) = read_lines("./inputs/input_5.txt") {
        let mut sum_not_right_order_middle_item = 0;
        let lines: Vec<String> = lines.flatten().collect();
        let mut rules: Vec<(i32, i32)> = Vec::new();
        let mut pages: Vec<Vec<i32>> = Vec::new();
        let re = Regex::new(r"\d+\|\d+").unwrap();
        lines.iter().for_each(|line| {
            if (re.is_match(line)) {
                let split: Vec<&str> = line.split('|').collect();
                let first: i32 = split[0].parse().unwrap();
                let second: i32 = split[1].parse().unwrap();
                rules.push((first, second));
            } else if !(line == "") {
                let nums: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
                pages.push(nums);
            }
        });

        pages.iter().for_each(|page| {
            let mut in_right_order = true;
            let copy_page = page.to_vec();
            let mut mutable_page = page.to_vec();
            mutable_page.sort_by(|a, b| {
                let ab_rule = rules.iter().find(|rule| {
                    rule.0 == *a && rule.1 == *b
                });
                let ba_rule = rules.iter().find(|rule| {
                    rule.0 == *b && rule.1 == *a
                });
                if ab_rule.is_some() {
                    return Ordering::Less;
                } else if ba_rule.is_some() {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            });
            // println!("{:?}", copy_page);
            // println!("{:?}", page_mut);
            for i in 0..page.len() {
                if mutable_page[i] != copy_page[i] {
                    in_right_order = false;
                    break;
                }
            }
            if !in_right_order {
                println!("{:?}", copy_page);
                sum_not_right_order_middle_item += mutable_page[page.len() / 2];
            }
        });

        println!("{}", sum_not_right_order_middle_item);
    }
}
