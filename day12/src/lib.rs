extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

// One vec of states (first output)
// One vec of groupings of damaged strings per line (second output)
// These can be zipped together to get a single line
fn parse_state(string_list: &Vec<String>) -> (Vec<Vec<SpringState>>, Vec<Vec<u32>>) {
    let mut states = vec![];
    let mut groups = vec![];

    for line in string_list {
        let (cur_state_s, group_s) = line.split_once(" ").unwrap();
        let mut cur_states = vec![];
        for c in cur_state_s.chars() {
            let s = match c {
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                '?' => SpringState::Unknown,
                _ => panic!("Parse error '{}'", c),
            };
            cur_states.push(s);
        }

        let mut cur_groups = vec![];
        for num in group_s.split(",") {
            let as_u32: u32 = num.parse().unwrap();
            cur_groups.push(as_u32);
        }

        groups.push(cur_groups);
        states.push(cur_states);
    }

    return (states, groups);
}

fn is_line_solved(state: &Vec<SpringState>, group: &Vec<u32>) -> bool {
    // No question marks
    let count = state
        .clone()
        .into_iter()
        .filter(|x| *x == SpringState::Unknown)
        .collect::<Vec<_>>()
        .len();
    //println!("Evaluating: {:?}; {:?}", state, group);
    if count != 0 {
        return false;
    }
    let mut run = 0;
    let mut run_len = 0;
    // Find first run of #
    for i in state {
        if *i == SpringState::Damaged {
            if run >= group.len() {
                //println!("Too many runs");
                return false;
            }
            run_len += 1;
            if run_len > group[run] {
                //println!(
                //    "Detected run too long [{}] found: {}, expected: {}",
                //    run, run_len, group[run]
                //);
                return false;
            }
            continue;
        }
        // . if run_len is already 0, just keep moving
        if run_len == 0 {
            continue;
        }
        // We just ended a run
        if run >= group.len() || run_len != group[run] {
            //println!("Detected not matching run [{}]", run);
            return false;
        }
        // Run is valid!
        run += 1;
        run_len = 0;
    }
    // Possible there is a run left to evaluate, if its at the end.
    if run_len > 0 && run < group.len() {
        return run_len == group[run] && run + 1 == group.len();
    } else if run_len == 0 && run == group.len() {
        return true;
    }
    //println!("Insufficient runs");
    return false;
}

fn solve_possibliites(state: &Vec<SpringState>, group: &Vec<u32>) -> u32 {
    // Brute force solution would be to flip every Unknown to operational or damaged, until it passes group, then
    // count those.
    // We can figure out how many loops that would be, each has two possible states, so 2^n
    let count = state
        .clone()
        .into_iter()
        .filter(|x| *x == SpringState::Unknown)
        .collect::<Vec<_>>()
        .len();
    let mut solves = 0;
    for i in 0..2_usize.pow(count.try_into().unwrap()) {
        // Flip based on bit mask
        // 0 = Operational
        // 1 = Damaged
        // i represents our bitmask, but its not at the correct indexes to just and
        let mut cur_i = i;
        let possible_solution: Vec<SpringState> = state
            .clone()
            .into_iter()
            .map(|value| {
                if value != SpringState::Unknown {
                    return value;
                }
                let result;
                if cur_i % 2 == 1 {
                    result = SpringState::Damaged;
                } else {
                    result = SpringState::Operational;
                }
                cur_i = cur_i / 2;
                return result;
            })
            .collect();
        if is_line_solved(&possible_solution, group) {
            //println!("Solved: {:?}", possible_solution);
            solves += 1;
        }
    }
    return solves;
}

/// Get sum of different possible arrangements
/// ```
/// let vec1: Vec<String> = vec![
///     "???.### 1,1,3",
///     ".??..??...?##. 1,1,3",
///     "?#?#?#?#?#?#?#? 1,3,1,6",
///     "????.#...#... 4,1,1",
///     "????.######..#####. 1,6,5",
///     "?###???????? 3,2,1",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day12::puzzle_a(&vec1), 21);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let (state, groups) = parse_state(string_list);
    let possibilities: Vec<u32> = state
        .iter()
        .zip(groups.iter())
        .map(|(cur_state, cur_group)| solve_possibliites(cur_state, cur_group))
        .collect();
    return possibilities.into_iter().sum();
}

fn parse_state_unfold(
    string_list: &Vec<String>,
    multiply_by: u32,
) -> (Vec<Vec<SpringState>>, Vec<Vec<u32>>) {
    let mut states = vec![];
    let mut groups = vec![];

    for line in string_list {
        let (cur_state_s, group_s) = line.split_once(" ").unwrap();
        let mut cur_states = vec![];
        for c in cur_state_s.chars() {
            let s = match c {
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                '?' => SpringState::Unknown,
                _ => panic!("Parse error '{}'", c),
            };
            cur_states.push(s);
        }

        let mut cur_groups = vec![];
        for num in group_s.split(",") {
            let as_u32: u32 = num.parse().unwrap();
            cur_groups.push(as_u32);
        }

        let mut unfolded_groups = vec![];
        let mut unfolded_states = vec![];
        for i in 0..multiply_by {
            if i != 0 {
                unfolded_states.push(SpringState::Unknown);
            }
            unfolded_groups.extend(cur_groups.clone());
            unfolded_states.extend(cur_states.clone());
        }

        // To avoid complications, lets just always have this end with a '.'
        // It doesn't change anything of the problem, but saves me coding a case.
        unfolded_states.push(SpringState::Operational);
        groups.push(unfolded_groups);
        states.push(unfolded_states);
    }

    return (states, groups);
}

type Cache = HashMap<(usize, u32, usize), u64>;

fn recrusive_solve(
    state: &Vec<SpringState>,
    group: &Vec<u32>,
    state_index: usize,
    current_count: u32,
    group_index: usize,
    cache: &mut Cache,
) -> u64 {
    let cache_key = (state_index, current_count, group_index);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let next_state_index = state_index + 1;

    // End case
    if state_index == state.len() {
        if group.len() == group_index {
            // If we are on the last group
            // and have reached this end state
            // Then this is valid!
            cache.insert(cache_key, 1);
            return 1;
        }
        // This isn't valid.
        cache.insert(cache_key, 0);
        return 0;
    }

    // Counting case: hit a damage, count up current run
    if state[state_index] == SpringState::Damaged {
        let result = recrusive_solve(
            state,
            group,
            state_index + 1,
            current_count + 1,
            group_index,
            cache,
        );
        cache.insert(cache_key, result);
        return result;
    }

    // Counting case: Hit an operational, also handle if we have hit the last group - we don't
    // want more damaged then.
    if state[state_index] == SpringState::Operational || group_index == group.len() {
        if group_index < group.len() && current_count == group[group_index] {
            // Solved this run, move on to the next one.
            let result = recrusive_solve(state, group, next_state_index, 0, group_index + 1, cache);
            cache.insert(cache_key, result);
            return result;
        } else if current_count == 0 {
            // Recurse to find this group
            let result = recrusive_solve(state, group, next_state_index, 0, group_index, cache);
            cache.insert(cache_key, result);
            return result;
        } else {
            // This path is dead
            cache.insert(cache_key, 0);
            return 0;
        }
    }

    // Question mark, we need to check if damaged, and if operational.
    let damaged_count = recrusive_solve(
        state,
        group,
        next_state_index,
        current_count + 1,
        group_index,
        cache,
    );
    let mut operational_count = 0;
    // If we have solved the current group, and this is now an operational, congrats increase the group!
    if current_count == group[group_index] {
        operational_count =
            recrusive_solve(state, group, next_state_index, 0, group_index + 1, cache);
    }
    // We are starting this group
    else if current_count == 0 {
        operational_count = recrusive_solve(state, group, next_state_index, 0, group_index, cache);
    }
    // Otherwise, this group is invalid, and we can skip doing all those.
    let v = damaged_count + operational_count;
    cache.insert(cache_key, v);
    return v;
}

fn initial_recursive_solve(state: &Vec<SpringState>, group: &Vec<u32>) -> u64 {
    let mut cache = Cache::new();
    return recrusive_solve(state, group, 0, 0, 0, &mut cache);
}

/// Unfold the input, get sum of arrangements.
/// Unfold is done by multiplying by 5.
/// ```
/// let vec1: Vec<String> = vec![
///     "???.### 1,1,3",
///     ".??..??...?##. 1,1,3",
///     "?#?#?#?#?#?#?#? 1,3,1,6",
///     "????.#...#... 4,1,1",
///     "????.######..#####. 1,6,5",
///     "?###???????? 3,2,1",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day12::puzzle_b(&vec1), 525152);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u64 {
    let (state, groups) = parse_state_unfold(string_list, 5);
    let possibilities: Vec<u64> = state
        .iter()
        .zip(groups.iter())
        .map(|(cur_state, cur_group)| initial_recursive_solve(cur_state, cur_group))
        .collect();
    return possibilities.into_iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_possibliites() {
        let groups = vec![3, 2, 1];
        let states = vec![
            SpringState::Unknown,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
        ];
        let result = solve_possibliites(&states, &groups);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_is_line_solved_properly_solved() {
        let states = vec![3, 2, 1];
        let mut line = vec![
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Operational,
        ];
        let mut result = is_line_solved(&line, &states);
        assert_eq!(result, true);

        line = vec![
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
        ];
        result = is_line_solved(&line, &states);
        assert_eq!(result, true);

        line = vec![
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Operational,
        ];
        result = is_line_solved(&line, &states);
        assert_eq!(result, true);

        line = vec![
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Operational,
        ];
        result = is_line_solved(&line, &states);
        assert_eq!(result, true);
    }
}
