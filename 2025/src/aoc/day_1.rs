use crate::aoc_utils;

pub fn solve_day() {
    solve_part_1();
    solve_part_2();
}

pub fn solve_part_1() {
    let data = aoc_utils::read_file_mem("personal-input/2025-12-01.txt");

    let mut v = 50;
    let mut result = 0;

    for line in data.lines() {
        let d = line.chars().nth(0).unwrap();
        let a = line[1..].parse::<i32>().unwrap() % 100;

        v = match d {
            'L' => calculate_result(v - a),
            'R' => calculate_result(v + a),
            _ => panic!("Cannot calculate {d}"),
        };

        if v == 0 {
            result += 1;
        }
    }

    println!("Day 1 part 1 answer is {result}");
}

pub fn solve_part_2() {
    println!("Day 1 part 2 has not yet been solved.")
}

fn calculate_result(result: i32) -> i32 {
    if result == 0 || result == 100 {
        return 0;
    }

    if result > 100 {
        return result - 100;
    }

    if result < 0 {
        return 100 - result.abs();
    }

    result
}
