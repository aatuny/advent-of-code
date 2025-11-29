pub fn main() {
    let solution: String = std::env::args()
        .nth(1)
        .expect("Solution definition is missing");

    match solution.as_str() {
        "warmup" => aoc_lib::aoc::warmup::solve_warmup(),
        _ => println!("Solution has not been implemented"),
    }
}
