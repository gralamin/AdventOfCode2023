extern crate filelib;

pub use filelib::load_no_blanks;

type IntValue = u64;
type MathValue = f64;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Race {
    distance: IntValue,
    time: IntValue,
}

fn parse_races(string_list: &Vec<String>) -> Vec<Race> {
    let mut result: Vec<Race> = vec![];
    let mut times: Vec<IntValue> = vec![];
    let mut distances: Vec<IntValue> = vec![];
    let mut iter = string_list.iter();
    let time_line = iter.next().unwrap();
    let distance_line = iter.next().unwrap();
    let (_, time_nums) = time_line.split_once("Time:").unwrap();
    let (_, distance_nums) = distance_line.split_once("Distance:").unwrap();
    for t in time_nums.trim().split(" ") {
        if t.len() == 0 {
            continue;
        }
        let v: IntValue = t.trim().parse().unwrap();
        times.push(v);
    }

    for d in distance_nums.trim().split(" ") {
        if d.len() == 0 {
            continue;
        }
        let v: IntValue = d.trim().parse().unwrap();
        distances.push(v);
    }

    for (t, d) in times.into_iter().zip(distances) {
        let race = Race {
            distance: d,
            time: t,
        };
        result.push(race);
    }
    return result;
}

fn parse_as_one_race(string_list: &Vec<String>) -> Race {
    let mut iter = string_list.iter();
    let time_line = iter.next().unwrap();
    let distance_line = iter.next().unwrap();
    let (_, time_nums) = time_line.split_once("Time:").unwrap();
    let (_, distance_nums) = distance_line.split_once("Distance:").unwrap();
    let mut time_string: String = "".to_string();
    let mut distance_string: String = "".to_string();
    //println!("time_nums {}", time_nums);
    for t in time_nums.trim().split(" ") {
        if t.len() == 0 {
            continue;
        }
        time_string = time_string + t.trim();
        //println!("Time so far {}", time_string);
    }
    let time = time_string.parse().unwrap();

    //println!("distance_nums {}", distance_nums);
    for d in distance_nums.trim().split(" ") {
        if d.len() == 0 {
            continue;
        }
        distance_string = distance_string + d.trim();
        //println!("Time so far {}", distance_string);
    }
    let distance = distance_string.parse().unwrap();

    //println!("Parsed Race d: {}, t: {}", distance, time);
    return Race {
        distance: distance,
        time: time,
    };
}

// You can hold to charge
// release to move. (Including the turn you release!)
// you move faster if button held longer (Increase velocity by 1)
// start at 0 distance per time increment
fn get_solutions_greater_distance(min_distance: IntValue, time: IntValue) -> IntValue {
    // Its literally impossible to score 9 on their example, so analyze this closer.
    // We can think of this geometrically
    // If we consider distance as the y value on a graph, and hold time as x
    // then we have a parabolla that arches at the top.
    //       /\
    //      /  \
    //     /    \
    // And we can think of the min distance as a line that intersects this
    //       /\
    //    --+--+--
    //     /    \
    // If we find the intersection points, then every whole number between them is an answer.
    // we have two lines that we need to make equal:
    // The parabolla: y = (time - x) * x = -(x^2) + time*x
    // And the line: y = distance
    // Then the intersection should be distance = -(x^2) + time*x -> 0 = -(x^2) + time*x - distance
    // Quadratic formula, some math, we get two solutions:
    // 0.5 * (time - sqrt(time^2 - 4 * distance))
    // 0.5 * (sqrt(time^2 - 4 * distance) + time)
    // Thats pretty easy to compute.

    let time_as_math: MathValue = time as MathValue;

    // 7^2 - 4*9 = 49 - 36 = 13
    let quadratic_mid: MathValue = (time.pow(2)) as MathValue - (4.0 * min_distance as MathValue);

    // sqrt(13) = 3.605
    let square_root_mid: MathValue = MathValue::sqrt(quadratic_mid);

    // 1.697
    let ans_one: MathValue = 0.5 * (time_as_math - square_root_mid);

    // 5.302
    let ans_two: MathValue = 0.5 * (square_root_mid + time_as_math);

    // So ceiling ans_one
    // floor ans_two
    // distance between them is answer
    /*
    println!(
        "d: {}, t: {} -> {} to {}",
        min_distance, time, ans_one, ans_two
    );
    */
    let ans_one_ceil = ans_one.ceil();
    let ans_two_floor = ans_two.floor();

    // normally just add one
    let mut range_offset: MathValue = 1.0;

    if ans_one_ceil == ans_one {
        // If already an integer, add one less
        range_offset -= 1.0;
    }
    if ans_two_floor == ans_two {
        // If already an integer, add one less
        range_offset -= 1.0;
    }

    return (ans_two_floor - ans_one_ceil + range_offset) as IntValue;
}

/// Deterimine how many ways you could beat the record of each race, then multiply together.
/// ```
/// let vec1: Vec<String> = vec![
///    "Time:      7  15   30",
///    "Distance:  9  40  200"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_a(&vec1), 288);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> IntValue {
    let races = parse_races(string_list);
    return races
        .iter()
        .map(|r| get_solutions_greater_distance(r.distance, r.time))
        .product();
}

/// As above, but treat time/distance as one long number string (ignore the spaces)
/// so we just have one simple one to solve.
/// ```
/// let vec1: Vec<String> = vec![
///    "Time:      7  15   30",
///    "Distance:  9  40  200"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_b(&vec1), 71503);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> IntValue {
    let race = parse_as_one_race(string_list);
    return get_solutions_greater_distance(race.distance, race.time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_get_solutions_greater_distance_samples() {
        assert_eq!(4, get_solutions_greater_distance(9, 7));
        assert_eq!(8, get_solutions_greater_distance(40, 15));
        assert_eq!(9, get_solutions_greater_distance(200, 30));
        assert_eq!(71503, get_solutions_greater_distance(940200, 71530));
    }
}
