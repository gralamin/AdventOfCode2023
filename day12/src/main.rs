use day12::load_no_blanks;
use day12::puzzle_a;
use day12::puzzle_b;

fn main() {
    let filename = "input";
    let lines = load_no_blanks(filename);

    let value = puzzle_a(&lines);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines);
    println!("Answer to 2nd question: {}", value_b);
}
