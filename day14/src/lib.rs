extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::Direction;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Terrain {
    RoundRock,
    CubeRock,
    Empty,
}

fn parse_input(string_list: &Vec<String>) -> Grid<Terrain> {
    let mut grid_values = vec![];
    let max_y = string_list.len();
    let max_x = string_list[0].len();
    for line in string_list {
        for c in line.chars() {
            let tile = match c {
                'O' => Terrain::RoundRock,
                '#' => Terrain::CubeRock,
                '.' => Terrain::Empty,
                _ => panic!("Unknown char '{}'", c),
            };
            grid_values.push(tile);
        }
    }
    return Grid::new(max_x, max_y, grid_values);
}

fn rotate_grid_clockwise(grid: &Grid<Terrain>) -> Grid<Terrain> {
    let n = grid.get_height();
    let m = grid.get_width();
    let new_height = m;
    let new_width = n;
    let mut rotated_data = grid.data_copy();
    for i in 0..n {
        for j in 0..m {
            let old_value = grid.get_value(GridCoordinate::new(j, i)).unwrap();
            let col = n - 1 - i;
            rotated_data[j * new_width + col] = old_value;
        }
    }

    return Grid::new(new_width, new_height, rotated_data);
}

fn tilt_grid(grid: Grid<Terrain>, dir: Direction) -> Grid<Terrain> {
    let mut rotated_grid = Grid::new(grid.get_width(), grid.get_height(), grid.data_copy());

    // Rotate it until its tilting north
    if dir != Direction::NORTH {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }
    if dir != Direction::WEST && dir != Direction::NORTH {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }
    if dir == Direction::EAST {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }
    let mut new_grid = Grid::new(
        rotated_grid.get_width(),
        rotated_grid.get_height(),
        rotated_grid.data_copy(),
    );

    for coord in rotated_grid.coord_iter() {
        let value = rotated_grid.get_value(coord).unwrap();
        if value != Terrain::RoundRock {
            continue;
        }
        let mut next_coord = coord.clone();
        let mut last_coord;
        while next_coord.y > 0 {
            last_coord = next_coord;
            next_coord = new_grid
                .get_coordinate_by_direction(last_coord, Direction::NORTH)
                .unwrap();
            let value = new_grid.get_value(next_coord).unwrap();
            if value != Terrain::Empty {
                // We have hit something, stop moving it.
                break;
            }
            new_grid.set_value(last_coord, Terrain::Empty);
            new_grid.set_value(next_coord, Terrain::RoundRock);
        }
    }

    // Rotate back
    // If east -> We have rotated 3 times, do it once more
    // If south -> Twice
    // West -> three
    rotated_grid = Grid::new(
        new_grid.get_width(),
        new_grid.get_height(),
        new_grid.data_copy(),
    );
    if dir != Direction::NORTH {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }
    if dir != Direction::EAST && dir != Direction::NORTH {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }
    if dir == Direction::WEST {
        rotated_grid = rotate_grid_clockwise(&rotated_grid);
    }

    return rotated_grid;
}

fn calc_load(grid: Grid<Terrain>) -> u32 {
    let mut result: u32 = 0;
    let height: u32 = grid.get_height().try_into().unwrap();
    for coord in grid.coord_iter() {
        let y_u32: u32 = coord.y.try_into().unwrap();
        result += match grid.get_value(coord).unwrap() {
            Terrain::RoundRock => height - y_u32,
            _ => 0,
        }
    }
    return result;
}

/// Sum of the loads when you make all round rocks roll north.
/// ```
/// let vec1: Vec<String> = vec![
///     "O....#....",
///     "O.OO#....#",
///     ".....##...",
///     "OO.#O....O",
///     ".O.....O#.",
///     "O.#..O.#.#",
///     "..O..#O..O",
///     ".......O..",
///     "#....###..",
///     "#OO..#....",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_a(&vec1), 136);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let g = parse_input(string_list);
    let h = tilt_grid(g, Direction::NORTH);
    return calc_load(h);
}

type Cache = HashMap<Vec<Terrain>, (usize, Vec<Terrain>)>;

fn _spin_once(grid: Grid<Terrain>) -> Grid<Terrain> {
    let start = tilt_grid(grid, Direction::NORTH);
    let counter_clock = tilt_grid(start, Direction::WEST);
    let one_eighty = tilt_grid(counter_clock, Direction::SOUTH);
    return tilt_grid(one_eighty, Direction::EAST);
}

fn spin_grid(grid: Grid<Terrain>, iterations: usize) -> Grid<Terrain> {
    let mut cache: Cache = Cache::new();
    let mut new_grid = Grid::new(grid.get_width(), grid.get_height(), grid.data_copy());

    let mut number_left = 0;

    for i in 0..iterations {
        let data_copy = new_grid.data_copy();
        if cache.contains_key(&data_copy) {
            let (last_index, next) = cache[&data_copy].clone();
            let cycle_size = i - last_index;
            number_left = (iterations - i) % cycle_size - 1;
            new_grid = Grid::new(new_grid.get_width(), new_grid.get_height(), next);
            break;
        }
        let next = _spin_once(new_grid);
        cache.insert(data_copy, (i, next.data_copy()));
        new_grid = next;
    }
    if number_left == 0 {
        return new_grid;
    }
    while number_left > 0 {
        let next = _spin_once(new_grid);
        new_grid = next;
        number_left -= 1;
    }

    return new_grid;
}

/// Spin cycle 1000000000 times, then calculate load.
/// ```
/// let vec1: Vec<String> = vec![
///     "O....#....",
///     "O.OO#....#",
///     ".....##...",
///     "OO.#O....O",
///     ".O.....O#.",
///     "O.#..O.#.#",
///     "..O..#O..O",
///     ".......O..",
///     "#....###..",
///     "#OO..#....",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day14::puzzle_b(&vec1), 64);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let g = parse_input(string_list);
    let h = spin_grid(g, 1000000000);
    return calc_load(h);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Grid<Terrain> {
        let vec1: Vec<String> = vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        return parse_input(&vec1);
    }

    #[test]
    fn test_spin_one() {
        let input = test_input();
        let r = spin_grid(input, 1);
        let x = r.data_copy();
        let expected: Vec<String> = vec![
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(x, parsed_expected.data_copy());
    }

    #[test]
    fn test_spin_two() {
        let input = test_input();
        let r = spin_grid(input, 2);
        let x = r.data_copy();
        let expected: Vec<String> = vec![
            ".....#....",
            "....#...O#",
            ".....##...",
            "..O#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#...O",
            ".......OOO",
            "#..OO###..",
            "#.OOO#...O",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(x, parsed_expected.data_copy());
    }

    #[test]
    fn test_spin_three() {
        let input = test_input();
        let r = spin_grid(input, 3);
        let x = r.data_copy();
        let expected: Vec<String> = vec![
            ".....#....",
            "....#...O#",
            ".....##...",
            "..O#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#...O",
            ".......OOO",
            "#...O###.O",
            "#.OOO#...O",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(x, parsed_expected.data_copy());
    }

    #[test]
    fn test_rotate_once() {
        let start: Vec<String> = vec![".##", "OO."].iter().map(|s| s.to_string()).collect();
        let grid = parse_input(&start);
        let rotated = rotate_grid_clockwise(&grid);
        assert_eq!(rotated.get_width(), 2);
        assert_eq!(rotated.get_height(), 3);

        let expected: Vec<String> = vec!["O.", "O#", ".#"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(rotated.data_copy(), parsed_expected.data_copy());
    }

    #[test]
    fn test_rotate_twice() {
        let start: Vec<String> = vec![".##", "OO."].iter().map(|s| s.to_string()).collect();
        let grid = parse_input(&start);
        let mut rotated = rotate_grid_clockwise(&grid);
        rotated = rotate_grid_clockwise(&rotated);
        assert_eq!(rotated.get_width(), 3);
        assert_eq!(rotated.get_height(), 2);

        let expected: Vec<String> = vec![".OO", "##."].iter().map(|s| s.to_string()).collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(rotated.data_copy(), parsed_expected.data_copy());
    }

    #[test]
    fn test_rotate_three() {
        let start: Vec<String> = vec![".##", "OO."].iter().map(|s| s.to_string()).collect();
        let grid = parse_input(&start);
        let mut rotated = rotate_grid_clockwise(&grid);
        rotated = rotate_grid_clockwise(&rotated);
        rotated = rotate_grid_clockwise(&rotated);
        assert_eq!(rotated.get_width(), 2);
        assert_eq!(rotated.get_height(), 3);

        let expected: Vec<String> = vec!["#.", "#O", ".O"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let parsed_expected = parse_input(&expected);

        assert_eq!(rotated.data_copy(), parsed_expected.data_copy());
    }

    #[test]
    fn test_rotate_360() {
        let start: Vec<String> = vec![".##", "OO."].iter().map(|s| s.to_string()).collect();
        let grid = parse_input(&start);
        let mut rotated = rotate_grid_clockwise(&grid);
        rotated = rotate_grid_clockwise(&rotated);
        rotated = rotate_grid_clockwise(&rotated);
        rotated = rotate_grid_clockwise(&rotated);
        assert_eq!(rotated.get_width(), 3);
        assert_eq!(rotated.get_height(), 2);
        assert_eq!(rotated.data_copy(), grid.data_copy());
    }
}
