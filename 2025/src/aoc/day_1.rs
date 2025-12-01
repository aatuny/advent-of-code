use crate::aoc_utils;

pub fn solve_day() {
    let data = aoc_utils::read_file_mem("personal-input/2025-12-01.txt");
    solve_part_1(data.as_str());
    solve_part_2(data.as_str());
}

pub fn solve_part_1(data: &str) {
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

pub fn solve_part_2(data: &str) {
    let mut v = 50;
    let mut result = 0;

    for line in data.lines() {
        let d = line.chars().nth(0).unwrap();
        let a = line[1..].parse::<i32>().unwrap();

        let an_1 = a % 100;
        let an_2: i32 = a / 100;

        v = match d {
            'L' => {
                let local_result = v - an_1;
                if local_result < 0 && v != 0 {
                    result += 1;
                }

                calculate_result(v - an_1)
            }
            'R' => {
                let local_result = v + an_1;
                if local_result > 100 {
                    result += 1;
                }

                calculate_result(v + an_1)
            }
            _ => panic!("Cannot calculate {d}"),
        };

        if v == 0 {
            result += 1;
        }

        if an_2 > 0 {
            result += an_2;
        }
    }

    println!("Day 1 part 2 answer is {result}");
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
