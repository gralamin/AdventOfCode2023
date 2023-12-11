extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::Grid;
use gridlib::GridCoordinate;
use gridlib::GridTraversable;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Cosmic {
    Space,
    Galaxy,
}

fn parse_grid(string_list: &Vec<String>) -> Grid<Cosmic> {
    let max_y = string_list.len();
    let max_x = string_list[0].len();
    let mut grid_values = vec![];

    for line in string_list.iter() {
        for c in line.chars() {
            let tile = match c {
                '#' => Cosmic::Galaxy,
                '.' => Cosmic::Space,
                _ => panic!("Unknown char"),
            };
            grid_values.push(tile);
        }
    }

    return Grid::new(max_x, max_y, grid_values);
}

fn expand_grid_rows(g: &Grid<Cosmic>, galaxy_coords: &Vec<GridCoordinate>) -> Grid<Cosmic> {
    let old_max_y = g.get_height();
    let max_x = g.get_width();
    let mut seen_ys = HashSet::new();
    for g in galaxy_coords {
        seen_ys.insert(g.y);
    }
    let mut new_max_y = g.get_height();
    let mut new_values = vec![];
    for y in 0..old_max_y {
        let row = &g.data_copy()[(y * max_x)..(y * max_x + max_x)];
        if seen_ys.contains(&y) {
            new_values.extend(row);
        } else {
            // Expand
            new_values.extend(row);
            new_values.extend(row);
            new_max_y += 1;
        }
    }
    return Grid::new(max_x, new_max_y, new_values);
}

fn expand_grid_columns(g: &Grid<Cosmic>, galaxy_coords: &Vec<GridCoordinate>) -> Grid<Cosmic> {
    let old_max_x = g.get_width();
    let max_y = g.get_height();
    let mut seen_xs = HashSet::new();
    for g in galaxy_coords {
        seen_xs.insert(g.x);
    }
    let mut new_max_x = g.get_width();
    // Columns are a bit more complicated, we need to add them "in place"
    // So start from the new_values.
    let mut new_values = g.data_copy();
    for x in 0..old_max_x {
        // Do the x backwards so we don't have to worry about how far in we are on both graphs.
        let backwards_x = old_max_x - x - 1;
        if seen_xs.contains(&backwards_x) {
            // Data is already there, don't touch this column
            continue;
        } else {
            // Expand, data is already there
            for y in 0..max_y {
                // Add these in backwards so we don't have to partially track x_offsets.
                let backwards_y = max_y - y - 1;
                // Location is defined as: x + y * width;
                // new_max_x is adjusted after we add this column, so its the current width
                let index = backwards_y * new_max_x + backwards_x;
                let before = &new_values[..index];
                let after = &new_values[index..];
                let mut a = before.to_vec();
                let b = after.to_vec();
                a.push(Cosmic::Space);
                a.extend(b);
                new_values = a;
            }
            new_max_x += 1;
        }
    }
    return Grid::new(new_max_x, max_y, new_values);
}

fn find_all_galaxies(g: &Grid<Cosmic>) -> Vec<GridCoordinate> {
    let mut galaxy_coords: Vec<GridCoordinate> = vec![];
    for coord in g.coord_iter() {
        if let Some(v) = g.get_value(coord) {
            if v == Cosmic::Galaxy {
                galaxy_coords.push(coord);
            }
        }
    }
    return galaxy_coords;
}

/* This is too slow
type Cache = HashSet<GridCoordinate>;

fn bfs_pathfind(g: &Grid<Cosmic>, start: GridCoordinate, end: GridCoordinate) -> usize {
    let mut seen = Cache::new();
    let mut queue = VecDeque::new();
    queue.push_back((vec![], start));

    while let Some((path_so_far, coord)) = queue.pop_front() {
        if seen.contains(&coord) {
            continue;
        }
        if coord == end {
            return path_so_far.len();
        }
        seen.insert(coord);

        let mut new_path = path_so_far.clone();
        new_path.push(coord);
        for new_coord in g.get_adjacent_coordinates(coord) {
            // Memory saving check
            if seen.contains(&new_coord) {
                continue;
            }
            queue.push_back((new_path.clone(), new_coord));
        }
    }
    // No path found
    return 0;
}
*/

fn manhattan_distance(start: GridCoordinate, end: GridCoordinate) -> usize {
    let x1: i32 = start.x.try_into().unwrap();
    let y1: i32 = start.y.try_into().unwrap();

    let x2: i32 = end.x.try_into().unwrap();
    let y2: i32 = end.y.try_into().unwrap();

    let a = x1 - x2;
    let b = y1 - y2;
    let result = a.abs() + b.abs();
    return result.try_into().unwrap();
}

/// Double any row or column that has no galaxies in size, then compute the pairwise
/// shortest path between all combinations of galaxies, only moving each empty space.
/// ```
/// let vec1: Vec<String> = vec![
///     "...#......",
///     ".......#..",
///     "#.........",
///     "..........",
///     "......#...",
///     ".#........",
///     ".........#",
///     "..........",
///     ".......#..",
///     "#...#....."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_a(&vec1), 374);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let grid = parse_grid(string_list);
    let mut galaxy_coords: Vec<GridCoordinate> = find_all_galaxies(&grid);
    // println!("Finished initial parse");
    let expanded_rows = expand_grid_rows(&grid, &galaxy_coords);
    // println!("done expanding rows");
    let expanded_columns = expand_grid_columns(&expanded_rows, &galaxy_coords);
    // println!("done expanding cols");
    galaxy_coords = find_all_galaxies(&expanded_columns);

    // 439
    // println!("{}", galaxy_coords.len());
    // 439 choose 2 = 96141 pairs, lol

    // Thanks itertools crate so I don't have to come up with the combinations myself.
    return galaxy_coords
        .into_iter()
        .combinations(2)
        //        .map(|v| bfs_pathfind(&expanded_columns, v[0], v[1]))
        .map(|v| manhattan_distance(v[0], v[1]))
        .sum();
}

/*
            111
  0123456789012
0 ....1........  CHECK
1 .........2... ERR off by x=1
2 3............ CHECK
3 .............
4 .............
5 ........4.... ERR off by x=1
6 .5........... CHECK
7 ............6 CHECK
8 .............
9 .............
0 .........7... ERR off by x=1
1 8....9....... Check, ERR off by x =1


*/

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_grid() -> Grid<Cosmic> {
        let s = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let grid = parse_grid(&s);
        return grid;
    }

    /*
      0123456789
    0 ...#......
    1 .......#..
    2 #.........
    3 ..........
    4 ......#...
    5 .#........
    6 .........#
    7 ..........
    8 .......#..
    9 #...#.....

    Row expand

      0123456789
    0 ...#......
    1 .......#..
    2 #.........
    3 ..........
    4 ..........
    5 ......#...
    6 .#........
    7 .........#
    8 ..........
    9 ..........
    0 .......#..
    1 #...#.....
    */

    #[test]
    fn test_grid_expand_rows() {
        let g = generate_grid();
        let og_galaxies = find_all_galaxies(&g);
        assert_eq!(
            og_galaxies,
            vec![
                GridCoordinate::new(3, 0),
                GridCoordinate::new(7, 1),
                GridCoordinate::new(0, 2),
                GridCoordinate::new(6, 4),
                GridCoordinate::new(1, 5),
                GridCoordinate::new(9, 6),
                GridCoordinate::new(7, 8),
                GridCoordinate::new(0, 9),
                GridCoordinate::new(4, 9),
            ]
        );
        let expanded = expand_grid_rows(&g, &og_galaxies);
        let expected_galaxies = vec![
            GridCoordinate::new(3, 0),
            GridCoordinate::new(7, 1),
            GridCoordinate::new(0, 2),
            GridCoordinate::new(6, 5),
            GridCoordinate::new(1, 6),
            GridCoordinate::new(9, 7),
            GridCoordinate::new(7, 10),
            GridCoordinate::new(0, 11),
            GridCoordinate::new(4, 11),
        ];
        let actual_galaxies = find_all_galaxies(&expanded);
        assert_eq!(actual_galaxies, expected_galaxies);
    }

    /*
        V  V  V
      0123456789
    0 ...#......
    1 .......#..
    2 #.........
    3 ..........
    4 ......#...
    5 .#........
    6 .........#
    7 ..........
    8 .......#..
    9 #...#.....

    Col Expand
                111
      0123456789012
    0 ....#........
    1 .........#...
    2 #............
    3 .............
    4 ........#....
    5 .#...........
    6 ............#
    7 .............
    8 .........#...
    9 #....#.......
    */

    #[test]
    fn test_grid_expand_cols() {
        let g = generate_grid();
        let og_galaxies = find_all_galaxies(&g);
        assert_eq!(
            og_galaxies,
            vec![
                GridCoordinate::new(3, 0),
                GridCoordinate::new(7, 1),
                GridCoordinate::new(0, 2),
                GridCoordinate::new(6, 4),
                GridCoordinate::new(1, 5),
                GridCoordinate::new(9, 6),
                GridCoordinate::new(7, 8),
                GridCoordinate::new(0, 9),
                GridCoordinate::new(4, 9),
            ]
        );
        let expanded = expand_grid_columns(&g, &og_galaxies);
        let expected_galaxies = vec![
            GridCoordinate::new(4, 0),
            GridCoordinate::new(9, 1),
            GridCoordinate::new(0, 2),
            GridCoordinate::new(8, 4),
            GridCoordinate::new(1, 5),
            GridCoordinate::new(12, 6),
            GridCoordinate::new(9, 8),
            GridCoordinate::new(0, 9),
            GridCoordinate::new(5, 9),
        ];
        let actual_galaxies = find_all_galaxies(&expanded);
        assert_eq!(actual_galaxies, expected_galaxies);
    }
}
