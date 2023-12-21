use day21::load_no_blanks;
use day21::puzzle_a;
use day21::puzzle_b;

fn main() {
    let filename = "input";
    let lines = load_no_blanks(filename);

    let value = puzzle_a(&lines, 64);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines, 26501365);
    println!("Answer to 2nd question: {}", value_b);
}
