use crate::aoc_utils;

// 2024-12-01 used as warmup

pub fn solve_warmup() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let data = aoc_utils::read_file_mem("personal-input/2024-12-01.txt");

    let mut result = 0;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in data.lines() {
        let line_values: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        v1.push(line_values[0]);
        v2.push(line_values[1]);
    }

    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        result += { v1[i] - v2[i] }.abs();
    }

    println!("Warmup part 1 answer is {result}")
}

pub fn solve_part_2() {
    let data = aoc_utils::read_file_mem("personal-input/2024-12-01.txt");

    let mut result = 0;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in data.lines() {
        let line_values: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        v1.push(line_values[0]);
        v2.push(line_values[1]);
    }

    for i in v1.iter() {
        result += v2.iter().filter(|x| **x == *i).count() as i32 * i;
    }

    println!("Warmup part 2 answer is {result}")
}
