extern crate filelib;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub use filelib::load_no_blanks;
use gridlib::Direction;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

fn parse_terrain(input: &Vec<String>) -> Grid<Terrain> {
    let width = input[0].len();
    let height = input.len();
    let mut values = vec![];

    for line in input {
        for c in line.chars() {
            let v = match c {
                '.' => Terrain::Path,
                '#' => Terrain::Forest,
                '>' => Terrain::Slope(Direction::EAST),
                '^' => Terrain::Slope(Direction::NORTH),
                '<' => Terrain::Slope(Direction::WEST),
                'V' => Terrain::Slope(Direction::SOUTH),
                'v' => Terrain::Slope(Direction::SOUTH),
                _ => panic!("error, '{}'", c),
            };
            values.push(v);
        }
    }

    return Grid::new(width, height, values);
}

fn find_entrance(grid: &Grid<Terrain>) -> GridCoordinate {
    let y = 0;
    for x in 0..grid.get_width() {
        let c = GridCoordinate::new(x, y);
        let v = grid.get_value(c).unwrap();
        if v == Terrain::Path {
            return c;
        }
    }
    return GridCoordinate::new(99, 99);
}

fn find_exit(grid: &Grid<Terrain>) -> GridCoordinate {
    let y = grid.get_height() - 1;
    for x in 0..grid.get_width() {
        let c = GridCoordinate::new(x, y);
        let v = grid.get_value(c).unwrap();
        if v == Terrain::Path {
            return c;
        }
    }
    return GridCoordinate::new(99, 99);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueState {
    cur_location: GridCoordinate,
    distance: u32,
    previous_steps: Vec<GridCoordinate>,
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Make large distances happen first!!!
        self.distance
            .cmp(&other.distance)
            .then_with(|| other.previous_steps.len().cmp(&self.previous_steps.len()))
            .then_with(|| other.cur_location.cmp(&self.cur_location))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn find_longest_path(
    grid: &Grid<Terrain>,
    start: &GridCoordinate,
    end: &GridCoordinate,
) -> Vec<GridCoordinate> {
    let mut final_path = vec![];
    let mut dist = vec![0; grid.get_height() * grid.get_width()];

    let mut queue = BinaryHeap::new();
    queue.push(QueueState {
        cur_location: start.clone(),
        distance: 0,
        previous_steps: vec![],
    });

    while let Some(state) = queue.pop() {
        let index = state.cur_location.y * grid.get_width() + state.cur_location.x;

        // We have looped, throw out path
        if state.previous_steps.contains(&state.cur_location) {
            continue;
        }

        if state.distance < dist[index] {
            // We have a longer path to here, throw this out.
            continue;
        }

        dist[index] = state.distance;

        if state.cur_location == *end {
            final_path = state.previous_steps.clone();
            final_path.push(state.cur_location);
            continue;
        }

        let cur_terrain = grid.get_value(state.cur_location).unwrap();
        match cur_terrain {
            Terrain::Path => {
                // check all four directions.
                for possible_dir in [
                    Direction::NORTH,
                    Direction::EAST,
                    Direction::SOUTH,
                    Direction::WEST,
                ] {
                    let next_pos =
                        grid.get_coordinate_by_direction(state.cur_location, possible_dir);

                    if next_pos == None {
                        continue;
                    }

                    let next_loc = next_pos.unwrap();
                    if state.previous_steps.contains(&next_loc) {
                        continue;
                    }
                    let mut steps = state.previous_steps.clone();
                    steps.push(state.cur_location);
                    let next_state = QueueState {
                        cur_location: next_loc,
                        distance: state.distance + 1,
                        previous_steps: steps,
                    };
                    queue.push(next_state);
                }
            }
            Terrain::Forest => {
                // Got onto a forest, abandon path.
                continue;
            }
            Terrain::Slope(d) => {
                // Follow the slope direction
                let next_pos = grid.get_coordinate_by_direction(state.cur_location, d);
                // Fell off the edge from the slope
                if next_pos == None {
                    continue;
                }

                let next_loc = next_pos.unwrap();
                let mut steps = state.previous_steps.clone();
                steps.push(state.cur_location);
                let next_state = QueueState {
                    cur_location: next_loc,
                    distance: state.distance + 1,
                    previous_steps: steps,
                };
                queue.push(next_state);
            }
        }
    }
    return final_path;
}

/// Find LONGEST path without steping on a tile twice.
/// ```
/// let vec1: Vec<String> = vec![
///     "#.#####################",
///     "#.......#########...###",
///     "#######.#########.#.###",
///     "###.....#.>.>.###.#.###",
///     "###v#####.#v#.###.#.###",
///     "###.>...#.#.#.....#...#",
///     "###v###.#.#.#########.#",
///     "###...#.#.#.......#...#",
///     "#####.#.#.#######.#.###",
///     "#.....#.#.#.......#...#",
///     "#.#####.#.#.#########v#",
///     "#.#...#...#...###...>.#",
///     "#.#.#v#######v###.###v#",
///     "#...#.>.#...>.>.#.###.#",
///     "#####v#.#.###v#.#.###.#",
///     "#.....#...#...#.#.#...#",
///     "#.#########.###.#.#.###",
///     "#...###...#...#...#.###",
///     "###.###.#.###v#####v###",
///     "#...#...#.#.>.>.#.>.###",
///     "#.###.###.#.###.#.#v###",
///     "#.....###...###...#...#",
///     "#####################.#"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_a(&vec1), 94);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let grid = parse_terrain(string_list);
    let entrance = find_entrance(&grid);
    let exit = find_exit(&grid);
    // We don't include The start square for some reason.
    return find_longest_path(&grid, &entrance, &exit).len() - 1;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day23::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}
