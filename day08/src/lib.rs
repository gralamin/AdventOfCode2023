extern crate filelib;

pub use filelib::load_no_blanks;

/// Follow instructions from AAA until ZZZ
/// ```
/// let vec1: Vec<String> = vec![
///    "LLR",
///    "",
///    "AAA = (BBB, BBB)",
///    "BBB = (AAA, ZZZ)",
///    "ZZZ = (ZZZ, ZZZ)",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1), 6);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    return 0;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}
