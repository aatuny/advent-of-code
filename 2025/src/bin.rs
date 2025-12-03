pub fn main() {
    let solution: String = std::env::args()
        .nth(1)
        .expect("Solution definition is missing");

    match solution.as_str() {
        "warmup" => aoc_lib::aoc::warmup::solve_warmup(),
        "day_1" => aoc_lib::aoc::day_1::solve_day(),
        "day_2" => aoc_lib::aoc::day_2::solve_day(),
        _ => println!("Solution has not been implemented"),
    }
}
