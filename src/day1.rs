use crate::utils::read_lines;

pub fn solve() {
    if let Ok(lines) = read_lines("./inputs/input_1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut firsts: Vec<i32> = Vec::new();
        let mut seconds: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let mut i = 0;
            line.split_whitespace().for_each(|x| {
                let num: i32 = x.parse().unwrap();
                if i == 0 {
                    firsts.push(num);
                } else {
                    seconds.push(num);
                }
                i += 1;
            });
        }
        firsts.sort();
        seconds.sort();
        let mut sum = 0;
        for i in 0..firsts.len() {
            sum += (firsts[i] - seconds[i]).abs();
        }
        println!("{}", sum);
    }
}

pub fn solve2() {
    if let Ok(lines) = read_lines("./inputs/input_1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut firsts: Vec<i32> = Vec::new();
        let mut seconds: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let mut i = 0;
            line.split_whitespace().for_each(|x| {
                let num: i32 = x.parse().unwrap();
                if i == 0 {
                    firsts.push(num);
                } else {
                    seconds.push(num);
                }
                i += 1;
            });
        }
        let mut sum = 0;
        firsts.iter().for_each(|x| {
            let count = seconds.iter().filter(|y| x == *y).count() as i32;
            sum += count * x;
        });
        println!("{}", sum);
    }
}