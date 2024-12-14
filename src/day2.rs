use crate::utils::read_lines;

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

pub fn solve() {
    if let Ok(lines) = read_lines("./inputs/input_2.txt") {
        let mut safe_reports = 0;

        for line in lines.flatten() {
            let split_whitespace: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            let first = split_whitespace[0];
            let second = split_whitespace[1];
            if first == second {
                continue;
            }
            let is_safe = is_row_safe(first < second, split_whitespace);
            if is_safe {
                safe_reports += 1;
            }
        }
        println!("{}", safe_reports);
    }
}

fn is_row_safe(is_increasing: bool, split_whitespace: Vec<i32>) -> bool {
    let mut prev_num = split_whitespace[0];
    return split_whitespace[1..].iter().all(|num| {
        let is_safe;
        if is_increasing {
            is_safe = prev_num < *num && *num - prev_num <= 3;
        } else {
            is_safe = prev_num > *num && prev_num - *num <= 3;
        }
        prev_num = *num;
        return is_safe;
    });
}

pub fn solve2() {
    if let Ok(lines) = read_lines("./inputs/input_2.txt") {
        let mut safe_reports = 0;

        for line in lines.flatten() {
            let split_whitespace: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            for i in 0..split_whitespace.len() {
                let removed_one: Vec<i32> = split_whitespace[0..i]
                    .iter()
                    .chain(split_whitespace[i + 1..].iter()).map(|x| { *x }).collect();

                let is_safe = is_row_safe(removed_one[0] < removed_one[1], removed_one);
                if is_safe {
                    safe_reports += 1;
                    break;
                }
            }
        }
        println!("{}", safe_reports);
    }
}
