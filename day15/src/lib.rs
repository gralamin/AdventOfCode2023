extern crate filelib;

pub use filelib::load_no_blanks;

type HashType = u32;

fn to_hash_list(s_list: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    assert_eq!(s_list.len(), 1);
    let input = s_list[0].clone();
    for x in input.split(",") {
        result.push(x.to_string());
    }
    return result;
}

fn hash(s: &str) -> HashType {
    let mut result = 0;

    for c in s.chars() {
        let ascii = c as u32;
        result += ascii;
        result *= 17;
        result %= 256;
    }

    assert!(result <= 255);
    return result;
}

/// We HASHing BOYS!
/// ```
/// let vec1: Vec<String> = vec![
///     "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day15::puzzle_a(&vec1), 1320);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> HashType {
    let hash_list = to_hash_list(string_list);
    return hash_list.into_iter().map(|x| hash(&x)).sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day15::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_rn1() {
        let s = "rn=1";
        assert_eq!(hash(s), 30);
    }

    #[test]
    fn test_hash_cm() {
        let s = "cm-";
        assert_eq!(hash(s), 253);
    }

    #[test]
    fn test_hash_qp3() {
        let s = "qp=3";
        assert_eq!(hash(s), 97);
    }

    #[test]
    fn test_hash_cm2() {
        let s = "cm=2";
        assert_eq!(hash(s), 47);
    }

    #[test]
    fn test_hash_qp() {
        let s = "qp-";
        assert_eq!(hash(s), 14);
    }

    #[test]
    fn test_hash_pc4() {
        let s = "pc=4";
        assert_eq!(hash(s), 180);
    }

    #[test]
    fn test_hash_ot9() {
        let s = "ot=9";
        assert_eq!(hash(s), 9);
    }

    #[test]
    fn test_hash_ab5() {
        let s = "ab=5";
        assert_eq!(hash(s), 197);
    }

    #[test]
    fn test_hash_pc() {
        let s = "pc-";
        assert_eq!(hash(s), 48);
    }

    #[test]
    fn test_hash_pc6() {
        let s = "pc=6";
        assert_eq!(hash(s), 214);
    }

    #[test]
    fn test_hash_ot7() {
        let s = "ot=7";
        assert_eq!(hash(s), 231);
    }
}
