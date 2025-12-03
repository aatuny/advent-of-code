use crate::aoc_utils;

struct Identifier {
    identifier: String,
    invalid: bool,
}

impl Identifier {
    fn new(identifier: String) -> Identifier {
        let is_valid = Identifier::is_valid(identifier.as_str());

        Identifier {
            identifier,
            invalid: is_valid,
        }
    }

    pub fn is_valid(identifier: &str) -> bool {
        if identifier.len() % 2 != 0 {
            return false;
        }

        let first_half = &identifier[..identifier.len() / 2];
        let second_half = &identifier[identifier.len() / 2..];
        first_half.find(second_half).is_some()
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
    solve_part_1(data);
}

pub fn solve_part_1(data: String) {
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
            .filter(|identifier| identifier.invalid)
            .map(|identifier| identifier.identifier.parse::<u64>().unwrap())
            .sum();

        result += invalid_identifiers_sum;
    }

    println!("Day 2 part 1 answer is {result}");
}
