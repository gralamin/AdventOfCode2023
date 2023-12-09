extern crate filelib;

pub use filelib::load_no_blanks;

type InputNum = i32;

pub fn parse_reports(string_list: &Vec<String>) -> Vec<Vec<InputNum>> {
    let mut result = vec![];
    for line in string_list {
        let mut cur_vec = vec![];
        for n in line.split(" ") {
            let n_u: InputNum = n.parse().unwrap();
            cur_vec.push(n_u);
        }
        result.push(cur_vec);
    }
    return result;
}

pub fn predict_next(history: &Vec<InputNum>) -> InputNum {
    if history
        .iter()
        .filter(|&x| *x != 0)
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        return 0;
    }
    let mut derivative_history = vec![];
    let mut last = 0;
    for w in history.windows(2) {
        let a = w[0];
        let b = w[1];
        let change = b - a;
        derivative_history.push(change);
        last = b;
    }
    return last + predict_next(&derivative_history);
}

/// Predict the next values.
/// ```
/// let vec1: Vec<String> = vec![
///     "0 3 6 9 12 15",
///     "1 3 6 10 15 21",
///     "10 13 16 21 30 45"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_a(&vec1), 114);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> InputNum {
    let histories = parse_reports(string_list);
    return histories.iter().map(|x| predict_next(x)).sum();
}

pub fn predict_first(history: &Vec<InputNum>) -> InputNum {
    if history
        .iter()
        .filter(|&x| *x != 0)
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        return 0;
    }
    let mut derivative_history = vec![];
    let mut first: Option<InputNum> = None;
    for w in history.windows(2) {
        let a = w[0];
        let b = w[1];
        let change = b - a;
        derivative_history.push(change);
        if let Some(_) = first {
            continue;
        }
        first = Some(a);
    }
    // The result of the recursive call is the amount to change the first value by
    let result = first.unwrap() - predict_first(&derivative_history);
    return result;
}

/// Predict the first values.
/// ```
/// let vec1: Vec<String> = vec![
///     "0 3 6 9 12 15",
///     "1 3 6 10 15 21",
///     "10 13 16 21 30 45"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_b(&vec1), 2);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> InputNum {
    let histories = parse_reports(string_list);
    return histories.iter().map(|x| predict_first(x)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_predict_first_first_sample() {
        let vec1: Vec<InputNum> = vec![0, 3, 6, 9, 12, 15];
        let result = predict_first(&vec1);
        assert_eq!(result, -3);
    }

    #[test]
    fn tst_predict_first_second_sample() {
        let vec1: Vec<InputNum> = vec![1, 3, 6, 10, 15, 21];
        let result = predict_first(&vec1);
        assert_eq!(result, 0);
    }

    #[test]
    fn tst_predict_first_third_sample() {
        let vec1: Vec<InputNum> = vec![10, 13, 16, 21, 30, 45];
        let result = predict_first(&vec1);
        assert_eq!(result, 5);
    }
}
