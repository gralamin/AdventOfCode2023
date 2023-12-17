extern crate filelib;

pub use filelib::load_no_blanks;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use gridlib::Direction;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;

fn parse_grid(string_list: &Vec<String>) -> Grid<u32> {
    let width = string_list[0].len();
    let height = string_list.len();
    let mut values = vec![];
    for line in string_list {
        for c in line.chars() {
            let digit = c.to_digit(10).unwrap();
            values.push(digit);
        }
    }
    return Grid::new(width, height, values);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PathStep {
    coord: GridCoordinate,
    dir: Option<Direction>,
}

// Implement a custom Queue state to handle priority queue aspects
#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueState {
    cur_direction: Option<Direction>,
    cur_cost: u32,
    cur_location: GridCoordinate,
    cur_streak: usize,
    previous_steps: Vec<PathStep>,
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cur_cost
            .cmp(&self.cur_cost)
            .then_with(|| other.previous_steps.len().cmp(&self.previous_steps.len()))
            .then_with(|| other.cur_location.cmp(&self.cur_location))
            .then_with(|| other.cur_streak.cmp(&self.cur_streak))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

// Variant on Dijkstra's, to handle the maximum in one direction bit.
// A bit more of a BFS, we aren't handling distances really.
fn pathfind(
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    grid: &Grid<u32>,
    min_streak: usize,
    max_streak: usize,
) -> (u32, Vec<PathStep>) {
    let start = GridCoordinate::new(start_x, start_y);
    let end = GridCoordinate::new(end_x, end_y);
    //println!("Going from {} to {}", start, end);
    let width = grid.get_width();
    let start_path = vec![PathStep {
        coord: start,
        dir: None,
    }];
    let mut final_path = vec![];

    // Set all distances to max
    // coordinate, direction, streak
    let mut visited: HashSet<(GridCoordinate, Direction, usize)> = HashSet::new();
    let mut dist = vec![u32::MAX; grid.get_height() * grid.get_width()];
    // There is a rule the start pos doesn't count.
    //let start_cost = grid.get_value(start).unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(QueueState {
        cur_direction: None,
        cur_cost: 0,
        cur_location: start,
        cur_streak: 0,
        previous_steps: start_path,
    });
    while let Some(state) = queue.pop() {
        //println!("Cur state {:?}", state);
        //println!("Queue length debug: {}", queue.len());
        if state.cur_streak > max_streak {
            //println!("Rejected due to number in dir");
            continue;
        }
        let index = state.cur_location.y * width + state.cur_location.x;

        if let Some(cur_dir) = state.cur_direction{
            if visited.contains(&(state.cur_location, cur_dir, state.cur_streak)) {
                continue
            }
            visited.insert((state.cur_location, cur_dir, state.cur_streak));
        }
        
        if state.cur_cost < dist[index] {
            dist[index] = state.cur_cost;
            if state.cur_location == end {
                final_path = state.previous_steps;
                //println!("Possible solution found!");
                continue;
            }
        }
        if state.cur_location == end {
            // Just an explicit guard to prevent iterations off of the end
            //println!("Possible solution found!");
            continue;
        }

        for possible_dir in [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ] {
            // Don't bother checking if its off the grid
            //println!("Checking {}", possible_dir);
            let next_pos = grid.get_coordinate_by_direction(state.cur_location, possible_dir);
            if next_pos == None {
                //println!("off the edge");
                continue;
            }
            let next_loc = next_pos.unwrap();

            // Costs are calculated on entering the block.
            let add_cost = grid.get_value(next_loc).unwrap();
            let next_cost = state.cur_cost + add_cost;

            let mut steps = state.previous_steps.clone();
            steps.push(PathStep {
                coord: next_loc,
                dir: Some(possible_dir),
            });

            // Handling for all places but start
            if let Some(cur_direction) = state.cur_direction {
                // Can't go backwards.
                if possible_dir != cur_direction && state.cur_streak >= min_streak {
                    let is_backwards = match cur_direction {
                        Direction::NORTH => possible_dir == Direction::SOUTH,
                        Direction::EAST => possible_dir == Direction::WEST,
                        Direction::SOUTH => possible_dir == Direction::NORTH,
                        Direction::WEST => possible_dir == Direction::EAST,
                        _ => panic!("Not supported diag"),
                    };
                    if is_backwards {
                        //println!("Rejecting due to backwards");
                        continue;
                    }

                    // Turn case
                    let next_state = QueueState {
                        cur_direction: Some(possible_dir),
                        cur_cost: next_cost,
                        cur_location: next_loc,
                        cur_streak: 1, // We have gone one step in this direction
                        previous_steps: steps,
                    };
                    queue.push(next_state);
                } else {
                    let num_dir = state.cur_streak + 1;
                    // Go straight case
                    let next_state = QueueState {
                        cur_direction: Some(possible_dir),
                        cur_cost: next_cost,
                        cur_location: next_loc,
                        cur_streak: num_dir,
                        previous_steps: steps,
                    };
                    queue.push(next_state);
                }
            } else {
                // Start handling
                let next_state = QueueState {
                    cur_direction: Some(possible_dir),
                    cur_cost: next_cost,
                    cur_location: next_loc,
                    cur_streak: 1, // We have gone one step in this direction
                    previous_steps: steps,
                };
                queue.push(next_state);
            }
        }
    }
    /*
    for p in final_path {
        println!("{} via {}, cost: {}", p.coord, p.dir, grid.get_value(p.coord).unwrap());
    }
    */
    return (dist[end.y * width + end.x], final_path);
}

fn debug_print_path(path: &Vec<PathStep>, grid: &Grid<u32>) {
    let debug = true;
    if !debug {
        return;
    }
    let mut cur_y = 0;
    let mut recalc_cost = 0;
    for coord in grid.coord_iter() {
        let v = grid.get_value(coord).unwrap();
        if coord.y != cur_y {
            println!("");
            cur_y = coord.y;
        }
        let mut value = format!("{}", v);
        for p in path.iter() {
            if p.coord == coord {
                if coord.x != 0 || coord.y != 0 {
                    recalc_cost += v;
                }
                if let Some(dir) = p.dir {
                    value = match dir {
                        Direction::NORTH => "^",
                        Direction::SOUTH => "V",
                        Direction::EAST => ">",
                        Direction::WEST => "<",
                        _ => panic!("Not supported"),
                    }
                    .to_string();
                }
                break;
            }
        }
        print!("{}", value);
    }
    println!("\nTotal cost: {}", recalc_cost);
}

/// Pathfind a weird graph
/// First, you can only go left, straight or right
/// Then you can't go more than 3 steps in direction
/// Then you want to minimize the heat loss
/// ```
/// let vec1: Vec<String> = vec![
///     "2413432311323",
///     "3215453535623",
///     "3255245654254",
///     "3446585845452",
///     "4546657867536",
///     "1438598798454",
///     "4457876987766",
///     "3637877979653",
///     "4654967986887",
///     "4564679986453",
///     "1224686865563",
///     "2546548887735",
///     "4322674655533"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day17::puzzle_a(&vec1), 102);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let grid = parse_grid(string_list);
    let (result, path) = pathfind(
        0,
        0,
        grid.get_width() - 1,
        grid.get_height() - 1,
        &grid,
        1,
        3,
    );
    debug_print_path(&path, &grid);
    return result;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day17::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_path() {
        /*
          012345678
        0 2>>34^>>>
        1 32V>>>353
        29 to top, 32 to bottom (right)
        */
        let vec1: Vec<String> = vec!["241343231", "321545353"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let grid = parse_grid(&vec1);
        let (result, path) = pathfind(0, 0, grid.get_width() - 1, 0, &grid, 1, 3);
        debug_print_path(&path, &grid);
        assert_eq!(result, 29);
        let (result, _path) = pathfind(0, 0, grid.get_width() - 1, 1, &grid, 1, 3);
        debug_print_path(&path, &grid);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_queue_order() {
        let zero_cost = QueueState {
            cur_direction: Some(Direction::NORTH),
            cur_cost: 0,
            cur_location: GridCoordinate::new(1, 0),
            cur_streak: 0,
            previous_steps: vec![],
        };
        let one_cost_a = QueueState {
            cur_direction: Some(Direction::EAST),
            cur_cost: 1,
            cur_location: GridCoordinate::new(2, 0),
            cur_streak: 1,
            previous_steps: vec![],
        };
        let one_cost_b = QueueState {
            cur_direction: Some(Direction::EAST),
            cur_cost: 1,
            cur_location: GridCoordinate::new(2, 0),
            cur_streak: 1,
            previous_steps: vec![PathStep {
                dir: Some(Direction::NORTH),
                coord: GridCoordinate::new(0, 0),
            }],
        };
        let two_cost = QueueState {
            cur_direction: Some(Direction::NORTH),
            cur_cost: 2,
            cur_location: GridCoordinate::new(1, 0),
            cur_streak: 0,
            previous_steps: vec![],
        };

        let mut to_order = vec![
            one_cost_a.clone(),
            zero_cost.clone(),
            two_cost.clone(),
            one_cost_b.clone(),
        ];
        to_order.sort();
        let visit_order = vec![
            zero_cost.clone(),
            one_cost_a.clone(),
            one_cost_b.clone(),
            two_cost.clone(),
        ];
        let sorted_order: Vec<QueueState> = visit_order.into_iter().rev().collect();

        assert_eq!(to_order, sorted_order);
    }
}
