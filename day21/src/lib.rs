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
    GardenPlot,
    Rock,
}

fn parse_grid(string_list: &Vec<String>) -> (Grid<Terrain>, GridCoordinate) {
    let width = string_list[0].len();
    let height = string_list.len();
    let mut values = vec![];
    let mut start_coord = GridCoordinate::new(0, 0);
    for (y, line) in string_list.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let t = match c {
                '.' => Terrain::GardenPlot,
                '#' => Terrain::Rock,
                'S' => {
                    start_coord = GridCoordinate::new(x, y);
                    Terrain::GardenPlot
                }
                _ => panic!("Unknown char"),
            };
            values.push(t);
        }
    }
    return (Grid::new(width, height, values), start_coord);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueState {
    distance: u32,
    location: GridCoordinate,
    previous_steps: Vec<GridCoordinate>,
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.previous_steps.len().cmp(&self.previous_steps.len()))
            .then_with(|| other.location.cmp(&self.location))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

// Dijkstra to find distances.
fn find_places_x_steps_from_you(
    grid: &Grid<Terrain>,
    origin: &GridCoordinate,
    num_steps: u32,
) -> usize {
    let width = grid.get_width();
    let start_path = vec![origin.clone()];

    // Set all distances to max
    // coordinate, direction, streak
    let mut dist: Vec<u32> = vec![u32::MAX; grid.get_height() * grid.get_width()];

    let mut queue = BinaryHeap::new();
    queue.push(QueueState {
        distance: 0,
        location: origin.clone(),
        previous_steps: start_path,
    });

    while let Some(state) = queue.pop() {
        // Because this is a priority queue, we do all the closest paths first
        // So if we ever hit too high, break.
        if state.distance > num_steps {
            break;
        }

        let index = state.location.y * width + state.location.x;
        if state.distance >= dist[index] {
            continue;
        }
        dist[index] = state.distance;

        for possible_dir in [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ] {
            let next_pos = grid.get_coordinate_by_direction(state.location, possible_dir);
            if next_pos == None {
                // Off the edge, skip it.
                continue;
            }
            let next_loc = next_pos.unwrap();
            let next_distance = state.distance + 1;
            let mut steps = state.previous_steps.clone();
            steps.push(next_loc);

            // Get the terrain type
            let terrain_type = grid.get_value(next_loc).unwrap();
            match terrain_type {
                Terrain::GardenPlot => {
                    let next_state = QueueState {
                        distance: next_distance,
                        location: next_loc,
                        previous_steps: steps,
                    };
                    queue.push(next_state);
                }
                Terrain::Rock => {
                    // Skip it
                    continue;
                }
            }
        }
    }

    return dist
        .into_iter()
        .filter(|x| *x <= num_steps)
        .filter(|x| x % 2 == num_steps % 2)
        .collect::<Vec<_>>()
        .len();
}

/// Where can you get with 64 plots
/// ```
/// let vec1: Vec<String> = vec![
///     "...........",
///     ".....###.#.",
///     ".###.##..#.",
///     "..#.#...#..",
///     "....#.#....",
///     ".##..S####.",
///     ".##..#...#.",
///     ".......##..",
///     ".##.#.####.",
///     ".##..##.##.",
///     "..........."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day21::puzzle_a(&vec1, 6), 16);
/// ```
pub fn puzzle_a(string_list: &Vec<String>, num_steps: u32) -> usize {
    let (grid, origin) = parse_grid(string_list);
    return find_places_x_steps_from_you(&grid, &origin, num_steps);
}

fn solve_quadratic(n: i64, v0: i64, v1: i64, v2: i64) -> i64 {
    return v0 + n * (v1 - v0) + n * (n - 1) / 2 * ((v2 - v1) - (v1 - v0));
}

fn expand_grid(grid: &Grid<Terrain>, factor: usize) -> Grid<Terrain> {
    let old_height = grid.get_height();
    let old_width = grid.get_width();
    let new_height = old_height * factor;
    let new_width = old_width * factor;
    let mut values = vec![];
    for y in 0..new_height {
        let old_y = y % old_height;
        for x in 0..new_width {
            let old_x = x % old_width;
            values.push(grid.get_value(GridCoordinate::new(old_x, old_y)).unwrap());
        }
    }
    return Grid::new(new_width, new_height, values);
}

/// Hahaha infinite grid now, slightly more complicated.
/// I failed to get this working
/// And solved manually. Not sure what I'm doing wrong to find the variables
/// for this quadratic.
pub fn puzzle_b(string_list: &Vec<String>, num_steps: u32) -> i64 {
    let (grid, origin) = parse_grid(string_list);
    let y_u32: u32 = origin.y.try_into().unwrap();
    let height_u32: u32 = grid.get_height().try_into().unwrap();
    let expanded_grid = expand_grid(&grid, 7);

    /*
      factor of 1:
      A -> 0 move
      Factor of 2:
      AA
      AA
      0 move
      factor of 3:
      AAA
      AAA -> 1 down, 1 right
      AAA
      factor of 4:
      AAAA
      AAAA  -> 1 down, 1 right
      AAAA
      AAAA
      factor of 5:
      AAAAA
      AAAAA
      AAAAA -> 2 down, 2 right
      AAAAA
      AAAAA
     */
    let new_start = GridCoordinate::new(origin.x + grid.get_height() *3, origin.y + grid.get_height() *3);

    let v0 = find_places_x_steps_from_you(&expanded_grid, &new_start, y_u32);
    let v1 = find_places_x_steps_from_you(&expanded_grid, &new_start, y_u32 + height_u32);
    let v2 = find_places_x_steps_from_you(&expanded_grid, &new_start, y_u32 + height_u32 * 2);
    let num_traverse = (num_steps - y_u32) / height_u32;
    return solve_quadratic(num_traverse as i64, v0 as i64, v1 as i64, v2 as i64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_grid() {
        let small_grid = Grid::new(
            2,
            2,
            vec![
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::GardenPlot,
                Terrain::Rock,
            ],
        );
        let expanded_grid = expand_grid(&small_grid, 2);
        assert_eq!(expanded_grid.get_height(), 4);
        assert_eq!(expanded_grid.get_width(), 4);
        assert_eq!(
            expanded_grid.data_copy(),
            vec![
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::GardenPlot,
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::Rock,
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::GardenPlot,
                Terrain::Rock,
                Terrain::GardenPlot,
                Terrain::Rock
            ]
        )
    }
}
