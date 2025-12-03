use crate::aoc_utils;

pub fn solve_day() {
    let data = aoc_utils::read_file_mem("personal-input/2025-12-01.txt");
    solve_part_1(data.as_str());
    solve_part_2(data.as_str());
}

fn solve_part_1(data: &str) {
    let mut dial = 50;
    let mut result = 0;

    for line in data.lines() {
        let directive = line.chars().nth(0).unwrap();
        let move_dial = line[1..].parse::<i32>().unwrap() % 100;

        dial = match directive {
            'L' => calculate_dial(dial - move_dial),
            'R' => calculate_dial(dial + move_dial),
            _ => panic!("Cannot calculate {directive}"),
        };

        if dial == 0 {
            result += 1;
        }
    }

    println!("Day 1 part 1 answer is {result}");
}

fn solve_part_2(data: &str) {
    let mut dial = 50;
    let mut result = 0;

    for line in data.lines() {
        let directive = line.chars().nth(0).unwrap();
        let amount = line[1..].parse::<i32>().unwrap();

        let move_dial = amount % 100;
        let full_rounds: i32 = amount / 100;

        dial = match directive {
            'L' => {
                let local_result = dial - move_dial;
                if local_result < 0 && dial != 0 {
                    result += 1;
                }

                calculate_dial(local_result)
            }
            'R' => {
                let local_result = dial + move_dial;
                if local_result > 100 {
                    result += 1;
                }

                calculate_dial(local_result)
            }
            _ => panic!("Cannot calculate {directive}"),
        };

        if dial == 0 {
            result += 1;
        }

        result += full_rounds;
    }

    println!("Day 1 part 2 answer is {result}");
}

fn calculate_dial(dial: i32) -> i32 {
    if dial == 100 {
        return 0;
    }

    if dial > 100 {
        return dial - 100;
    }

    if dial < 0 {
        return dial + 100;
    }

    dial
}
