extern crate filelib;

pub use filelib::load_no_blanks;

/// Get the sum of all first and last numbers in each line. If a single number appears in a line, count it for both.
/// ```
/// let vec1: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_a(&vec1), 142);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let tens: Vec<u32> = string_list.iter().map(|x| x.chars().nth(x.find(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap()).collect();
    let ones: Vec<u32> = string_list.iter().map(|x| x.chars().nth(x.rfind(char::is_numeric).unwrap()).unwrap().to_digit(10).unwrap()).collect();
    return tens.iter().zip(ones.iter()).map(|(x, y)| x * 10 + y).sum();
}

fn to_sums(calorie_list: &Vec<Vec<i32>>) -> Vec<i32> {
    return calorie_list.iter().map(|l| l.iter().sum()).collect();
}

/// Get the top 3 highest sum of a group of vectors
/// ```
/// let vec1 = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000,8000,9000], vec![10000]];
/// assert_eq!(day01::puzzle_b(&vec1), 45000);
/// ```
pub fn puzzle_b(calorie_lists: &Vec<Vec<i32>>) -> i32 {
    let mut sums = to_sums(calorie_lists);
    sums.sort();
    return sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sums() {
        let vec1 = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(to_sums(&vec1), expected);
    }
}
