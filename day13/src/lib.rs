extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum LavaTerrain {
    Ash,
    Rock,
}

fn parse_input(list_of_string_list: &Vec<Vec<String>>) -> Vec<Grid<LavaTerrain>> {
    let mut result = vec![];

    for grid_string in list_of_string_list {
        let max_y = grid_string.len();
        let max_x = grid_string[0].len();
        let mut grid_values = vec![];
        for line in grid_string {
            for c in line.chars() {
                let tile = match c {
                    '#' => LavaTerrain::Rock,
                    '.' => LavaTerrain::Ash,
                    _ => panic!("Unknown char '{}'", c),
                };
                grid_values.push(tile);
            }
        }
        let grid = Grid::new(max_x, max_y, grid_values);
        result.push(grid);
    }

    return result;
}

fn solve_for_horz_or_vert_sym(grid: &Grid<LavaTerrain>) -> u32 {
    //println!();
    //println!("{:?}", grid);
    let v = solve_for_horz_sym(grid);
    if let Some(g) = v {
        //println!("Found horz {}", g);
        return g * 100;
    }
    let vert = solve_for_vert_sym(grid).unwrap();
    //println!("Found vert {}", vert);
    return vert;
}

fn solve_for_vert_sym(grid: &Grid<LavaTerrain>) -> Option<u32> {
    let width = grid.get_width();
    let mut cols = vec![];
    for _ in 0..width {
        cols.push(vec![]);
    }
    for coord in grid.coord_iter() {
        cols[coord.x].push(grid.get_value(coord).unwrap());
    }

    return solve_for_sym(&cols);
}

fn solve_for_sym(list: &Vec<Vec<LavaTerrain>>) -> Option<u32> {
    // Find a perfect reflection on grid
    // Find two adjacent items on the list that are the same
    // Then check forward and back they are the same
    // continue until run out of spaces.

    for (index, cur_list) in list.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let last_index = index - 1;
        if *cur_list != list[last_index] {
            continue;
        }
        // println!("Possible reflection {} -\n{:?},\n{:?}", index, cur_list, list[index]);
        // Possible reflection
        let mut index_offset = 1;
        let mut did_match = true;
        while index_offset <= last_index && index + index_offset < list.len() {
            let early_index = last_index - index_offset;
            let late_index = index + index_offset;
            let compare_one = list[early_index].clone();
            let compare_two = list[late_index].clone();
            assert_eq!(compare_one.len(), compare_two.len());
            // println!("Checking {} vs {} \n{:?},\n{:?}",  early_index, late_index, compare_one, compare_two);
            if compare_one != compare_two {
                did_match = false;
                break;
            }
            index_offset += 1;
        }
        if did_match {
            let result: u32 = index.try_into().unwrap();
            return Some(result);
        }
    }
    return None;
}

fn solve_for_horz_sym(grid: &Grid<LavaTerrain>) -> Option<u32> {
    let mut rows = vec![];
    let mut cur_row = vec![];
    let mut cur_y = 0;
    for coord in grid.coord_iter() {
        if coord.y == cur_y {
            cur_row.push(grid.get_value(coord).unwrap());
        } else {
            rows.push(cur_row);
            cur_row = vec![grid.get_value(coord).unwrap()];
            cur_y += 1;
        }
    }
    rows.push(cur_row);

    return solve_for_sym(&rows);
}

/// Find a vertical or horizontal line of reflection in each input
/// Count columns to the left, and 100 times the rows above.
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "#.##..##.",
///     "..#.##.#.",
///     "##......#",
///     "##......#",
///     "..#.##.#.",
///     "..##..##.",
///     "#.#.##.#.",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "#...##..#",
///     "#....#..#",
///     "..##..###",
///     "#####.##.",
///     "#####.##.",
///     "..##..###",
///     "#....#..#"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day13::puzzle_a(&vec1), 405);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> u32 {
    let input = parse_input(string_list);
    return input
        .into_iter()
        .map(|grid| solve_for_horz_or_vert_sym(&grid))
        .sum();
}

/// Foo
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "#.##..##.",
///     "..#.##.#.",
///     "##......#",
///     "##......#",
///     "..#.##.#.",
///     "..##..##.",
///     "#.#.##.#.",
///     ].iter().map(|s| s.to_string()).collect(), vec![
///     "#...##..#",
///     "#....#..#",
///     "..##..###",
///     "#####.##.",
///     "#####.##.",
///     "..##..###",
///     "#....#..#"
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day13::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<Grid<LavaTerrain>> {
        let vec1: Vec<Vec<String>> = vec![
            vec![
                "#.##..##.",
                "..#.##.#.",
                "##......#",
                "##......#",
                "..#.##.#.",
                "..##..##.",
                "#.#.##.#.",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            vec![
                "#...##..#",
                "#....#..#",
                "..##..###",
                "#####.##.",
                "#####.##.",
                "..##..###",
                "#....#..#",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ];
        return parse_input(&vec1);
    }

    #[test]
    fn test_vertical() {
        let input = test_input();
        let a = solve_for_vert_sym(&input[0]);
        assert_eq!(a, Some(5));
        let b = solve_for_vert_sym(&input[1]);
        assert_eq!(b, None);
    }

    #[test]
    fn test_horiz() {
        let input = test_input();
        let a = solve_for_horz_sym(&input[0]);
        assert_eq!(a, None);
        let b = solve_for_horz_sym(&input[1]);
        assert_eq!(b, Some(4));
    }

    #[test]
    fn test_horiz_tricky() {
        let a = LavaTerrain::Ash;
        let r = LavaTerrain::Rock;
        let input = Grid::new(
            13,
            7,
            vec![
                a, r, r, r, r, r, r, a, a, a, r, r, r, r, a, a, r, r, a, a, r, r, r, a, a, a, r, a,
                a, r, r, a, a, r, r, r, a, a, a, a, r, r, r, r, r, r, a, a, a, r, r, r, r, a, r, r,
                r, r, a, r, a, r, r, a, r, a, a, a, a, a, a, r, a, a, r, r, r, a, r, r, a, a, a, a,
                r, r, r, r, r, r, a,
            ],
        );
        let z = solve_for_horz_sym(&input);
        assert_eq!(z, Some(2));
    }
}
