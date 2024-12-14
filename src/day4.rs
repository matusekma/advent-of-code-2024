use regex::Regex;
use std::fs;

use crate::utils::read_lines;

pub fn solve() {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut diagonal = 0;
    let text: String = fs::read_to_string("./inputs/input_4.txt").unwrap();

    horizontal = text.match_indices("XMAS").count() + text.match_indices("SAMX").count();
    println!("horizontal: {}", horizontal);

    if let Ok(lines) = read_lines("./inputs/input_4.txt") {
        let lines: Vec<String> = lines.flatten().collect();
        let line_length = lines[0].len();

        // vertical
        for i in 0..(lines.len() - 3) {
            for j in 0..line_length {
                if lines[i].chars().nth(j).unwrap() == 'X'
                    && lines[i + 1].chars().nth(j).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j).unwrap() == 'S'
                {
                    vertical += 1;
                }
                if lines[i].chars().nth(j).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j).unwrap() == 'A'
                    && lines[i + 2].chars().nth(j).unwrap() == 'M'
                    && lines[i + 3].chars().nth(j).unwrap() == 'X'
                {
                    vertical += 1;
                }
            }
            println!();
        }
        println!("vertical: {}", vertical);

        // diagonal right
        for i in 0..(lines.len() - 3) {
            for j in 0..(line_length - 3) {
                if lines[i].chars().nth(j).unwrap() == 'X'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    diagonal += 1;
                }
                if lines[i].chars().nth(j).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'A'
                    && lines[i + 2].chars().nth(j + 2).unwrap() == 'M'
                    && lines[i + 3].chars().nth(j + 3).unwrap() == 'X'
                {
                    diagonal += 1;
                }
            }
            println!();
        }
        println!("diagonal right: {}", diagonal);

        // diagonal left
        for i in 0..(lines.len() - 3) {
            for j in 3..line_length {
                if lines[i].chars().nth(j).unwrap() == 'X'
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    diagonal += 1;
                }
                if lines[i].chars().nth(j).unwrap() == 'S'
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'A'
                    && lines[i + 2].chars().nth(j - 2).unwrap() == 'M'
                    && lines[i + 3].chars().nth(j - 3).unwrap() == 'X'
                {
                    diagonal += 1;
                }
            }
            println!();
        }
        println!("diagonal left: {}", diagonal);
    }
    println!("{}", vertical);
    println!("{}", diagonal);

    println!("{}", horizontal + vertical + diagonal);
}

pub fn solve2() {
    let mut num_x_mas = 0;

    if let Ok(lines) = read_lines("./inputs/input_4.txt") {
        let lines: Vec<String> = lines
            .flatten()
            .map(|line| "...".to_string() + &line + "...")
            .collect(); // padding
        let line_length = lines[0].len();
        for i in 0..(lines.len() - 2) {  // only 2 because MAS is 3 characters
            for j in 0..(line_length-1) {
                if (lines[i].chars().nth(j).unwrap() == 'M'
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'A'
                    && lines[i + 2].chars().nth(j + 2).unwrap() == 'S')
                    || (lines[i].chars().nth(j).unwrap() == 'S'
                        && lines[i + 1].chars().nth(j + 1).unwrap() == 'A'
                        && lines[i + 2].chars().nth(j + 2).unwrap() == 'M')
                {
                    if (lines[i].chars().nth(j + 2).unwrap() == 'M'
                        && lines[i + 1].chars().nth(j + 1).unwrap() == 'A'
                        && lines[i + 2].chars().nth(j).unwrap() == 'S')
                        || (lines[i].chars().nth(j + 2).unwrap() == 'S'
                            && lines[i + 1].chars().nth(j + 1).unwrap() == 'A'
                            && lines[i + 2].chars().nth(j).unwrap() == 'M')
                    {
                        num_x_mas += 1;
                    }
                }
            }
        }
    }

    println!("{}", num_x_mas);
}
