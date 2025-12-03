use crate::aoc_utils;

struct Identifier {
    identifier: String,
    invalid_1: bool,
    invalid_2: bool,
}

impl Identifier {
    fn new(identifier: String) -> Identifier {
        let is_valid_1 = Identifier::is_invalid_1(identifier.as_str());
        let is_valid_2 = Identifier::is_invalid_2(identifier.as_str());

        Identifier {
            identifier,
            invalid_1: is_valid_1,
            invalid_2: is_valid_2,
        }
    }

    fn is_invalid_1(identifier: &str) -> bool {
        if identifier.len() % 2 != 0 {
            return false;
        }

        let first_half = &identifier[..identifier.len() / 2];
        let second_half = &identifier[identifier.len() / 2..];
        first_half.find(second_half).is_some()
    }

    fn is_invalid_2(identifier: &str) -> bool {
        let mut window = String::from("");

        for (idx, c) in identifier.chars().enumerate() {
            window.push(c);

            let char_left = identifier.len() - window.len();
            if window.len() > char_left {
                return false;
            }

            let haystack = &identifier[idx + 1..];
            let split = haystack.split(window.as_str()).collect::<Vec<&str>>();
            let not_repeated = split.iter().filter(|x| x.len() > 0).count();

            if not_repeated == 0 {
                return true;
            }
        }

        false
    }
}

struct Range {
    identifiers: Vec<Identifier>,
}

impl Range {
    fn new(begin: u64, end: u64) -> Range {
        let identifiers: Vec<Identifier> = (begin..end + 1)
            .map(|x| Identifier::new(x.to_string()))
            .collect();

        Range { identifiers }
    }
}

pub fn solve_day() {
    let data = aoc_utils::read_file_mem("personal-input/2025-12-02.txt");
    solve_part_1(data.as_str());
    solve_part_2(data.as_str());
}

fn solve_part_1(data: &str) {
    let range_definitions: Vec<&str> = data.trim().split(",").collect();
    let mut result = 0;

    for range_definition in range_definitions {
        let range_chunks: Vec<&str> = range_definition.split("-").collect();
        let range_begin: u64 = range_chunks[0].parse::<u64>().unwrap();
        let range_end: u64 = range_chunks[1].parse::<u64>().unwrap();

        let range = Range::new(range_begin, range_end);
        let invalid_identifiers_sum: u64 = range
            .identifiers
            .iter()
            .filter(|identifier| identifier.invalid_1)
            .map(|identifier| identifier.identifier.parse::<u64>().unwrap())
            .sum();

        result += invalid_identifiers_sum;
    }

    println!("Day 2 part 1 answer is {result}");
}

fn solve_part_2(data: &str) {
    let range_definitions: Vec<&str> = data.trim().split(",").collect();
    let mut result = 0;

    for range_definition in range_definitions {
        let range_chunks: Vec<&str> = range_definition.split("-").collect();
        let range_begin: u64 = range_chunks[0].parse::<u64>().unwrap();
        let range_end: u64 = range_chunks[1].parse::<u64>().unwrap();

        let range = Range::new(range_begin, range_end);
        let invalid_identifiers_sum: u64 = range
            .identifiers
            .iter()
            .filter(|identifier| identifier.invalid_2)
            .map(|identifier| identifier.identifier.parse::<u64>().unwrap())
            .sum();

        result += invalid_identifiers_sum;
    }

    println!("Day 2 part 2 answer is {result}");
}
