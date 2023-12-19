use day19::load;
use day19::puzzle_a;
use day19::puzzle_b;
use day19::split_lines_by_blanks;

fn main() {
    let filename = "input";
    let template = load(filename);
    let groups = split_lines_by_blanks(&template);

    let value = puzzle_a(&groups);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&groups);
    println!("Answer to 2nd question: {}", value_b);
}
