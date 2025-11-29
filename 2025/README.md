# Advent of Code 2025

This year I'm attempting to solve at least some of the Advent of Code puzzles with Rust. Aim is to learn by doing.

Few points regarding the puzzle solutions posted to this repository:

- Each day is written as lib module which is usable through cli executable (bin). Cli takes exactly one argument: number for the day to acquire solution (with addition of "warmup").
- Each day reads input from within personal-input -folder using file naming convention of `yyyy-mm-dd.txt`. All inputs are omitted from repo per AoC FAQ.
- The code readability will probably be awful(ish) and some of it will be on purpose to avoid including any part of the puzzle text to code/comments in order to respect the AoC FAQ guidelines.
- Tests are not implemented (again with purpose of avoiding to include any inputs).
- Input format is trusted implicitly. If input file is broken, the solution will also produce panic.
