extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::Direction;
use gridlib::GridCoordinateInf64;

// Directions are Up = North, Down = South, Right = east, Left = west
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PlanStep {
    dir: Direction,
    distance: u64,
}

fn parse_plan(input: &Vec<String>) -> Vec<PlanStep> {
    let mut result = vec![];

    for line in input {
        let (dir_s, rest) = line.split_once(" ").unwrap();
        let (dist_s, _) = rest.split_once(" ").unwrap();

        let direction = match dir_s {
            "R" => Direction::EAST,
            "D" => Direction::SOUTH,
            "L" => Direction::WEST,
            "U" => Direction::NORTH,
            _ => panic!("Unsupported dir"),
        };
        let distance: u64 = dist_s.parse().unwrap();
        let step = PlanStep {
            dir: direction,
            distance: distance,
        };
        result.push(step);
    }

    return result;
}

fn get_points(plan: &Vec<PlanStep>) -> Vec<GridCoordinateInf64> {
    // Assume start at 0,0
    // follow each plan step to get a list of grid coordinates.
    // Return those
    let start = GridCoordinateInf64::new(0, 0);
    let mut result = vec![start];
    let mut cur = start;
    for step in plan {
        for _ in 0..step.distance {
            cur = cur.move_dir(step.dir);
        }
        result.push(cur);
    }
    return result;
}

fn compute_area(plan: &Vec<PlanStep>) -> u64 {
    // First, convert the planSteps to a list of
    let points = get_points(plan);
    //println!("{:?}", points);
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    // Then hey, its the shoelace formula again!
    // Reverse the order to reverse the orientation to be Positive, this is important.
    let double_area: i64 = points
        .clone()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .map(|window| {
            let (coord_1, coord_2) = (window[0], window[1]);
            let x1 = coord_1.x;
            let x2 = coord_2.x;
            let mut y1 = coord_1.y;
            let mut y2 = coord_2.y;
            // We need to caluclate the determinant
            // x1 x2
            // y1 y2
            // x1 * y2 - y1 * x2
            // However, this assumes a different coordinate space then we are in
            // this will map it.
            y1 = max_y - y1;
            y2 = max_y - y2;
            return (x1 * y2) - (x2 * y1);
        })
        .sum();

    //println!("double area: {}", double_area);
    let single_area: u64 = (double_area / 2).try_into().unwrap();
    let perimeter: i64 = points
        .clone()
        .into_iter()
        .as_slice()
        .windows(2)
        .map(|window| {
            let (coord_1, coord_2) = (window[0], window[1]);
            let x1 = coord_1.x;
            let x2 = coord_2.x;
            let y1 = coord_1.y;
            let y2 = coord_2.y;
            let inner = (x2 - x1).pow(2) + (y2 - y1).pow(2);
            let outer: i64 = f64::sqrt(inner as f64) as i64;
            return outer;
        })
        .sum();

    let perm_32: u64 = perimeter.try_into().unwrap();

    // Now we need to use Pick's theorem to add in the perimeter.
    return single_area + perm_32 / 2 + 1;
}

/*
  0123456
0 X#####X
1 #.....#
2 X#X...#
3 ..#...#
4 ..#...#
5 X#X.X#X
6 #...#..
7 XX..X#X
8 .#....#
9 .X####X
[GridCoordinateInf { x: 0, y: 0 },
GridCoordinateInf { x: 6, y: 0 },
GridCoordinateInf { x: 6, y: 5 },
GridCoordinateInf { x: 4, y: 5 },
GridCoordinateInf { x: 4, y: 7 },
GridCoordinateInf { x: 6, y: 7 },
GridCoordinateInf { x: 6, y: 9 },
GridCoordinateInf { x: 1, y: 9 },
 GridCoordinateInf { x: 1, y: 7 },
 GridCoordinateInf { x: 0, y: 7 },
 GridCoordinateInf { x: 0, y: 5 },
 GridCoordinateInf { x: 2, y: 5 },
 GridCoordinateInf { x: 2, y: 2 },
 GridCoordinateInf { x: 0, y: 2 },
 GridCoordinateInf { x: 0, y: 0 }]
*/

/// Get how much lava the dig plan could hold
/// ```
/// let vec1: Vec<String> = vec![
///    "R 6 (#70c710)",
///    "D 5 (#0dc571)",
///    "L 2 (#5713f0)",
///    "D 2 (#d2c081)",
///    "R 2 (#59c680)",
///    "D 2 (#411b91)",
///    "L 5 (#8ceee2)",
///    "U 2 (#caa173)",
///    "L 1 (#1b58a2)",
///    "U 2 (#caa171)",
///    "R 2 (#7807d2)",
///    "U 3 (#a77fa3)",
///    "L 2 (#015232)",
///    "U 2 (#7a21e3)",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_a(&vec1), 62);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u64 {
    let plan = parse_plan(string_list);
    return compute_area(&plan);
}

fn parse_plan_hex(input: &Vec<String>) -> Vec<PlanStep> {
    let mut result = vec![];

    for line in input {
        let (_, rgb_str_with_endbracket) = line.split_once("(#").unwrap();
        // 012345
        // 70c710
        //      ^ Direction
        // ^---^  Distance
        let dir_s = rgb_str_with_endbracket.chars().nth(5).unwrap();
        let dist_s = &rgb_str_with_endbracket[0..=4];

        let direction = match dir_s {
            '0' => Direction::EAST,
            '1' => Direction::SOUTH,
            '2' => Direction::WEST,
            '3' => Direction::NORTH,
            _ => panic!("Unsupported dir"),
        };
        let distance: u64 = u64::from_str_radix(dist_s, 16).unwrap();
        let step = PlanStep {
            dir: direction,
            distance: distance,
        };
        result.push(step);
    }

    return result;
}

/// Parse from the hex instead
/// ```
/// let vec1: Vec<String> = vec![
///    "R 6 (#70c710)",
///    "D 5 (#0dc571)",
///    "L 2 (#5713f0)",
///    "D 2 (#d2c081)",
///    "R 2 (#59c680)",
///    "D 2 (#411b91)",
///    "L 5 (#8ceee2)",
///    "U 2 (#caa173)",
///    "L 1 (#1b58a2)",
///    "U 2 (#caa171)",
///    "R 2 (#7807d2)",
///    "U 3 (#a77fa3)",
///    "L 2 (#015232)",
///    "U 2 (#7a21e3)",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day18::puzzle_b(&vec1), 952408144115);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u64 {
    let plan = parse_plan_hex(string_list);
    return compute_area(&plan);
}
