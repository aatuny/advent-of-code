use crate::aoc_utils;

pub fn solve_day() {
    let data = aoc_utils::read_file_mem("personal-input/2025-12-03.txt");
    let lines: Vec<&str> = data.lines().collect();

    solve_part_1(lines);
    solve_part_2();
}

#[derive(Debug)]
struct Bank {
    voltage: u32,
}

impl Bank {
    fn from(batteries: String) -> Bank {
        Bank {
            voltage: Bank::find_voltage_1(batteries.as_str()),
        }
    }

    fn find_voltage_1(batteries: &str) -> u32 {
        let mut first = 0;
        let mut second = 0;

        let mut local_max = 0;

        for (idx, c) in batteries.chars().enumerate() {
            let num = c.to_digit(10).unwrap();
            let is_final_char = idx == batteries.len() - 1;

            if idx == 0 {
                first = num;
                continue;
            }

            if num > first && !is_final_char {
                let new_local_max = Bank::parse_voltage(first, second);
                if new_local_max > local_max {
                    local_max = new_local_max;
                    first = num;
                    second = 0;
                    continue;
                }
            }

            if second == 0 && is_final_char {
                second = num;
                continue;
            }

            if num > second {
                second = num;
            }
        }

        if second == 0 {
            return local_max;
        }

        Bank::parse_voltage(first, second)
    }

    fn parse_voltage(first: u32, second: u32) -> u32 {
        format!("{first}{second}").parse::<u32>().unwrap()
    }
}

fn solve_part_1(lines: Vec<&str>) {
    let mut result = 0;

    for line in lines {
        let bank = Bank::from(line.to_string());
        result += bank.voltage;
    }

    println!("Day 2 part 1 answer is {result}");
}

fn solve_part_2() {
    println!("Day 2 part 2 answer not yet solved");
}
