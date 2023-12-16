extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::Direction;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::thread;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum MirrorDir {
    SouthwestToNortheast,
    NorthwestToSoutheast,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum SplitterDir {
    HorizontalToVertical,
    VerticalToHorizontal,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Terrain {
    Empty,
    Mirror(MirrorDir),
    Splitter(SplitterDir),
}

fn parse_grid(lines: &Vec<String>) -> Grid<Terrain> {
    let width = lines[0].len();
    let height = lines.len();
    let mut grid_values = vec![];

    for line in lines {
        for c in line.chars() {
            let tile = match c {
                '.' => Terrain::Empty,
                '/' => Terrain::Mirror(MirrorDir::SouthwestToNortheast),
                '\\' => Terrain::Mirror(MirrorDir::NorthwestToSoutheast),
                '|' => Terrain::Splitter(SplitterDir::HorizontalToVertical),
                '-' => Terrain::Splitter(SplitterDir::VerticalToHorizontal),
                _ => panic!("SAVE MEEEEEE"),
            };
            grid_values.push(tile);
        }
    }

    return Grid::new(width, height, grid_values);
}

fn ray_trace(grid: &Grid<Terrain>) -> Vec<GridCoordinate> {
    // It starts at 0,0 with direction East
    let start = GridCoordinate::new(0, 0);
    let start_direction = Direction::EAST;
    return ray_trace_inner(grid, start, start_direction);
}

// Factor out for part b
fn ray_trace_inner(
    grid: &Grid<Terrain>,
    start: GridCoordinate,
    start_direction: Direction,
) -> Vec<GridCoordinate> {
    // count how many tiles the ray goes through
    let mut visited_coords: HashSet<GridCoordinate> = HashSet::new();
    let mut cache: HashSet<(Direction, GridCoordinate)> = HashSet::new();
    let mut to_visit: VecDeque<(Direction, GridCoordinate)> = VecDeque::new();
    to_visit.push_back((start_direction, start));

    while let Some((cur_direction, coord)) = to_visit.pop_front() {
        // if we have been here and in this direction, nothing has changed so move on
        if cache.contains(&(cur_direction, coord)) {
            continue;
        }
        visited_coords.insert(coord);
        cache.insert((cur_direction, coord));
        let terrain = grid.get_value(coord).unwrap();
        match terrain {
            Terrain::Empty => {
                // Keep going in current direction
                if let Some(next_coord) = grid.get_coordinate_by_direction(coord, cur_direction) {
                    to_visit.push_back((cur_direction, next_coord));
                }
            }
            Terrain::Mirror(dir) => {
                match dir {
                    MirrorDir::SouthwestToNortheast => {
                        // This is a /
                        // If dir = East, become North, if dir = south, become West
                        // If dir = west, become south, if dir = north, become east
                        let next_direction = match cur_direction {
                            Direction::EAST => Direction::NORTH,
                            Direction::NORTH => Direction::EAST,
                            Direction::SOUTH => Direction::WEST,
                            Direction::WEST => Direction::SOUTH,
                            _ => panic!("Not supported"),
                        };
                        if let Some(next_coord) =
                            grid.get_coordinate_by_direction(coord, next_direction)
                        {
                            to_visit.push_back((next_direction, next_coord));
                        }
                    }
                    MirrorDir::NorthwestToSoutheast => {
                        // This is a \
                        // If dir = East, become South, if dir = south, become east
                        // If dir = west, become North, if dir = north, become West
                        let next_direction = match cur_direction {
                            Direction::EAST => Direction::SOUTH,
                            Direction::SOUTH => Direction::EAST,
                            Direction::NORTH => Direction::WEST,
                            Direction::WEST => Direction::NORTH,
                            _ => panic!("Not supported"),
                        };
                        if let Some(next_coord) =
                            grid.get_coordinate_by_direction(coord, next_direction)
                        {
                            to_visit.push_back((next_direction, next_coord));
                        }
                    }
                }
            }
            Terrain::Splitter(dir) => {
                match dir {
                    SplitterDir::HorizontalToVertical => {
                        // This is a |
                        // If north south, treat as empty
                        // If east west, split into two beams, one north, one south
                        match cur_direction {
                            Direction::NORTH | Direction::SOUTH => {
                                if let Some(next_coord) =
                                    grid.get_coordinate_by_direction(coord, cur_direction)
                                {
                                    to_visit.push_back((cur_direction, next_coord));
                                }
                            }
                            Direction::EAST | Direction::WEST => {
                                if let Some(north_coord) =
                                    grid.get_coordinate_by_direction(coord, Direction::NORTH)
                                {
                                    to_visit.push_back((Direction::NORTH, north_coord));
                                }
                                if let Some(south_coord) =
                                    grid.get_coordinate_by_direction(coord, Direction::SOUTH)
                                {
                                    to_visit.push_back((Direction::SOUTH, south_coord));
                                }
                            }
                            _ => panic!("Not supported"),
                        }
                    }
                    SplitterDir::VerticalToHorizontal => {
                        // This is a -
                        // If north south, split into two beams, one east, one west
                        // If east west, treat as empty
                        match cur_direction {
                            Direction::NORTH | Direction::SOUTH => {
                                if let Some(east_coord) =
                                    grid.get_coordinate_by_direction(coord, Direction::EAST)
                                {
                                    to_visit.push_back((Direction::EAST, east_coord));
                                }
                                if let Some(west_coord) =
                                    grid.get_coordinate_by_direction(coord, Direction::WEST)
                                {
                                    to_visit.push_back((Direction::WEST, west_coord));
                                }
                            }
                            Direction::EAST | Direction::WEST => {
                                if let Some(next_coord) =
                                    grid.get_coordinate_by_direction(coord, cur_direction)
                                {
                                    to_visit.push_back((cur_direction, next_coord));
                                }
                            }
                            _ => panic!("Not supported"),
                        }
                    }
                }
            }
        }
    }

    return Vec::from_iter(visited_coords);
}

/// Energize the mirrors
/// ```
/// let vec1: Vec<String> = vec![
///    r".|...\....",
///    r"|.-.\.....",
///    r".....|-...",
///    r"........|.",
///    r"..........",
///    r".........\",
///    r"..../.\\..",
///    r".-.-/..|..",
///    r".|....-|.\",
///    r"..//.|....",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day16::puzzle_a(&vec1), 46);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let grid = parse_grid(string_list);
    let energized_tiles = ray_trace(&grid);
    return energized_tiles.len();
}

fn generate_entrances(grid: &Grid<Terrain>) -> Vec<(Direction, GridCoordinate)> {
    let mut results = vec![];
    let max_x = grid.get_width() - 1;
    let max_y = grid.get_height() - 1;
    for coord in grid.coord_iter() {
        if coord.x == 0 {
            // Left side, can enter here from the east
            results.push((Direction::EAST, coord));
        } else if coord.x == max_x {
            results.push((Direction::WEST, coord));
        }
        if coord.y == 0 {
            // Top, enter to the south
            results.push((Direction::SOUTH, coord));
        } else if coord.y == max_y {
            results.push((Direction::NORTH, coord));
        }
    }
    return results;
}

/// Try from every possible start point
/// ```
/// let vec1: Vec<String> = vec![
///    r".|...\....",
///    r"|.-.\.....",
///    r".....|-...",
///    r"........|.",
///    r"..........",
///    r".........\",
///    r"..../.\\..",
///    r".-.-/..|..",
///    r".|....-|.\",
///    r"..//.|....",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day16::puzzle_b(&vec1), 51);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let grid = parse_grid(string_list);
    let entrances = generate_entrances(&grid);
    return entrances
        .into_iter()
        .map(|(direction, origin)| {
            let thread_grid = grid.clone();
            let handle = thread::spawn(move || {
                return ray_trace_inner(&thread_grid, origin, direction).len();
            });
            return handle;
        })
        .map(|handle| handle.join().unwrap())
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_trace_simple_empty_three_mirror() {
        let vec1: Vec<String> = vec![r".\", r"\/"].iter().map(|s| s.to_string()).collect();
        let grid = parse_grid(&vec1);
        assert_eq!(ray_trace(&grid).len(), 4);
    }
}
