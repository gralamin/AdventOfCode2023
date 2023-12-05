use day05::load;
use day05::puzzle_a;
use day05::puzzle_b;
use day05::split_lines_by_blanks;

fn main() {
    let filename = "input";
    let template = load(filename);
    let groups = split_lines_by_blanks(&template);

    let value = puzzle_a(&groups);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&groups);
    println!("Answer to 2nd question: {}", value_b);
}
