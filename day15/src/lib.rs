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

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct LabeledLens {
    focal_length: HashType,
    label: String,
}

fn hash_map(maps: &Vec<Vec<LabeledLens>>, s: &str) -> Vec<Vec<LabeledLens>> {
    let mut new_maps = maps.clone();
    if s.chars().last().unwrap().is_numeric() {
        // = case
        let (label, value) = s.split_once("=").unwrap();
        let value_parsed: HashType = value.parse().unwrap();
        let lens = LabeledLens {
            label: label.to_string(),
            focal_length: value_parsed,
        };
        let box_num: usize = hash(label).try_into().unwrap();
        let mut match_found = false;
        for (index, x) in maps[box_num].clone().into_iter().enumerate() {
            if x.label == lens.label {
                new_maps[box_num][index] = lens.clone();
                match_found = true;
                break;
            }
        }
        if !match_found {
            new_maps[box_num].push(lens);
        }
    } else {
        let (label, _) = s.split_once("-").unwrap();
        let box_num: usize = hash(label).try_into().unwrap();
        let mut offset = 0;
        let base = maps[box_num].clone();
        let base_len = base.len();
        for (index, x) in base.into_iter().enumerate() {
            if index + offset > base_len {
                break;
            }
            if offset == 0 {
                if x.label == label {
                    offset += 1;
                }
            } else {
                // we found one, move things over
                new_maps[box_num][index - offset] = x;
            }
        }
        while offset > 0 {
            new_maps[box_num].pop();
            offset -= 1;
        }
    }
    return new_maps;
}

fn focus_power(l: &LabeledLens, box_index: usize, slot_index: usize) -> HashType {
    let box_num: HashType = (box_index + 1).try_into().unwrap();
    let slot_num: HashType = (slot_index + 1).try_into().unwrap();
    return l.focal_length * box_num * slot_num;
}

/// We HASH MAPPING BOYS
/// ```
/// let vec1: Vec<String> = vec![
///     "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day15::puzzle_b(&vec1), 145);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> HashType {
    let hash_list = to_hash_list(string_list);
    let mut maps: Vec<Vec<LabeledLens>> = vec![];
    for _ in 0..256 {
        maps.push(vec![]);
    }
    for x in hash_list {
        maps = hash_map(&maps, &x);
    }
    let mut result = 0;
    for (index, cur_box) in maps.iter().enumerate() {
        for (len_index, cur_lens) in cur_box.iter().enumerate() {
            result += focus_power(&cur_lens, index, len_index);
        }
    }
    return result;
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

    #[test]
    fn test_hashmap_rn1() {
        let hash_list: Vec<String> = vec!["rn=1"].iter().map(|s| s.to_string()).collect();
        let mut maps: Vec<Vec<LabeledLens>> = vec![];
        for _ in 0..256 {
            maps.push(vec![]);
        }
        for x in hash_list {
            maps = hash_map(&maps, &x);
        }
        assert_eq!(maps[0].len(), 1);
        assert_eq!(
            maps[0][0],
            LabeledLens {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
    }

    #[test]
    fn test_hashmap_cm() {
        let hash_list: Vec<String> = vec!["rn=1", "cm-"].iter().map(|s| s.to_string()).collect();
        let mut maps: Vec<Vec<LabeledLens>> = vec![];
        for _ in 0..256 {
            maps.push(vec![]);
        }
        for x in hash_list {
            maps = hash_map(&maps, &x);
        }
        assert_eq!(maps[0].len(), 1);
        assert_eq!(
            maps[0][0],
            LabeledLens {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
    }

    #[test]
    fn test_hashmap_qp3() {
        let hash_list: Vec<String> = vec!["rn=1", "cm-", "qp=3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut maps: Vec<Vec<LabeledLens>> = vec![];
        for _ in 0..256 {
            maps.push(vec![]);
        }
        for x in hash_list {
            maps = hash_map(&maps, &x);
        }
        assert_eq!(maps[0].len(), 1);
        assert_eq!(
            maps[0][0],
            LabeledLens {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
        assert_eq!(maps[1].len(), 1);
        assert_eq!(
            maps[1][0],
            LabeledLens {
                label: "qp".to_string(),
                focal_length: 3
            }
        );
    }

    #[test]
    fn test_hashmap_cm2() {
        let hash_list: Vec<String> = vec!["rn=1", "cm-", "qp=3", "cm=2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut maps: Vec<Vec<LabeledLens>> = vec![];
        for _ in 0..256 {
            maps.push(vec![]);
        }
        for x in hash_list {
            maps = hash_map(&maps, &x);
        }
        assert_eq!(maps[0].len(), 2);
        assert_eq!(
            maps[0][0],
            LabeledLens {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
        assert_eq!(
            maps[0][1],
            LabeledLens {
                label: "cm".to_string(),
                focal_length: 2
            }
        );
        assert_eq!(maps[1].len(), 1);
        assert_eq!(
            maps[1][0],
            LabeledLens {
                label: "qp".to_string(),
                focal_length: 3
            }
        );
    }

    #[test]
    fn test_hashmap_qp() {
        let hash_list: Vec<String> = vec!["rn=1", "cm-", "qp=3", "cm=2", "qp-"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut maps: Vec<Vec<LabeledLens>> = vec![];
        for _ in 0..256 {
            maps.push(vec![]);
        }
        for x in hash_list {
            maps = hash_map(&maps, &x);
        }
        assert_eq!(maps[0].len(), 2);
        assert_eq!(
            maps[0][0],
            LabeledLens {
                label: "rn".to_string(),
                focal_length: 1
            }
        );
        assert_eq!(
            maps[0][1],
            LabeledLens {
                label: "cm".to_string(),
                focal_length: 2
            }
        );
        assert_eq!(maps[1].len(), 0);
    }
}
