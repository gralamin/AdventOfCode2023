extern crate filelib;
extern crate gridlib;

use crate::gridlib::GridTraversable;
pub use filelib::load_no_blanks;

use std::collections::HashMap;

type PartNumber = u32;

#[derive(PartialEq, Debug, Copy, Clone)]
enum ParsedSpace {
    Blank,
    Numeral(PartNumber),
    Symbol(char),
}

fn parse_grid(string_list: &Vec<String>) -> gridlib::Grid<ParsedSpace> {
    let width = string_list.iter().nth(0).unwrap().len();
    let height = string_list.len();
    let mut values: Vec<ParsedSpace> = vec![];
    for s in string_list {
        let parsed_line: Vec<ParsedSpace> = s
            .chars()
            .map(|c| match c {
                '.' => ParsedSpace::Blank,
                '0' => ParsedSpace::Numeral(0),
                '1' => ParsedSpace::Numeral(1),
                '2' => ParsedSpace::Numeral(2),
                '3' => ParsedSpace::Numeral(3),
                '4' => ParsedSpace::Numeral(4),
                '5' => ParsedSpace::Numeral(5),
                '6' => ParsedSpace::Numeral(6),
                '7' => ParsedSpace::Numeral(7),
                '8' => ParsedSpace::Numeral(8),
                '9' => ParsedSpace::Numeral(9),
                _ => ParsedSpace::Symbol(c),
            })
            .collect();
        values.extend(parsed_line);
    }

    return gridlib::Grid::new(width, height, values);
}

fn get_parts_adjacent_to_symbols(grid: &gridlib::Grid<ParsedSpace>) -> Vec<PartNumber> {
    let mut result: Vec<PartNumber> = vec![];
    let mut cur_number: Option<PartNumber> = None;
    let mut cur_y = 0;
    let mut cur_part_num = false;
    for coord in grid.coord_iter() {
        if coord.y != cur_y {
            // We switched rows
            // First if there is a number we should add, do so
            if let Some(n) = cur_number {
                if cur_part_num {
                    result.push(n);
                }
            }
            // set back to defaults with the new y.
            cur_y = coord.y;
            cur_number = None;
            cur_part_num = false;
        }
        let v = grid.get_value(coord).unwrap();
        match v {
            ParsedSpace::Blank => {
                if cur_part_num {
                    // we aren't on a number anymore
                    if let Some(n) = cur_number {
                        if cur_part_num {
                            result.push(n);
                        }
                    }
                    cur_part_num = false;
                }
                // cur_number should always be reset
                cur_number = None;
            }
            ParsedSpace::Numeral(new_ones_place) => {
                if let Some(x) = cur_number {
                    cur_number = Some(x * 10 + new_ones_place);
                } else {
                    cur_number = Some(new_ones_place);
                }
            }
            ParsedSpace::Symbol(_) => {
                if cur_part_num {
                    // we aren't on a number anymore
                    if let Some(n) = cur_number {
                        if cur_part_num {
                            result.push(n);
                        }
                    }
                    cur_part_num = false;
                }
                // cur_number should always be reset
                cur_number = None;
            }
        }
        if let Some(_) = cur_number {
            if !cur_part_num {
                // Check if we should be a part number
                for possible_loc in grid.get_adjacent_coordinates(coord) {
                    if cur_part_num {
                        break;
                    }
                    let z = grid.get_value(possible_loc).unwrap();
                    match z {
                        ParsedSpace::Symbol(_) => {
                            cur_part_num = true;
                        }
                        _ => {}
                    }
                }
                for possible_loc in grid.get_diag_adjacent_coordinates(coord) {
                    if cur_part_num {
                        break;
                    }
                    let z = grid.get_value(possible_loc).unwrap();
                    match z {
                        ParsedSpace::Symbol(_) => {
                            cur_part_num = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Check if we have a last value, if so add it
    if let Some(n) = cur_number {
        if cur_part_num {
            result.push(n);
        }
    }

    return result;
}

/// Add up all parts on the grid that are adjacent to a symbol thats not a period.
/// ```
/// let vec1: Vec<String> = vec![
///    "467..114..",
///    "...*......",
///    "..35..633.",
///    "......#...",
///    "617*......",
///    ".....+.58.",
///    "..592.....",
///    "......755.",
///    "...$.*....",
///    ".664.598..",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_a(&vec1), 4361);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> PartNumber {
    let grid = parse_grid(string_list);
    let part_nums = get_parts_adjacent_to_symbols(&grid);
    return part_nums.iter().sum();
}

type SymbolCoord = gridlib::GridCoordinate;
type GearMap = HashMap<SymbolCoord, Vec<PartNumber>>;

fn get_parts_adjacent_to_symbol(grid: &gridlib::Grid<ParsedSpace>, symbol: char) -> GearMap {
    let mut result: GearMap = GearMap::new();
    let mut cur_number: Option<PartNumber> = None;
    let mut cur_y = 0;
    let mut gear_symbol_coord: Option<SymbolCoord> = None;
    for coord in grid.coord_iter() {
        if coord.y != cur_y {
            // We switched rows
            // First if there is a number we should add, do so
            if let Some(n) = cur_number {
                if let Some(gear_coord) = gear_symbol_coord {
                    let cur_vector = result.entry(gear_coord).or_insert(vec![]);
                    cur_vector.push(n);
                }
            }
            // set back to defaults with the new y.
            cur_y = coord.y;
            cur_number = None;
            gear_symbol_coord = None;
        }
        let v = grid.get_value(coord).unwrap();
        match v {
            ParsedSpace::Blank => {
                if let Some(n) = cur_number {
                    if let Some(gear_coord) = gear_symbol_coord {
                        let cur_vector = result.entry(gear_coord).or_insert(vec![]);
                        cur_vector.push(n);
                    }
                }
                gear_symbol_coord = None;
                cur_number = None;
            }
            ParsedSpace::Numeral(new_ones_place) => {
                if let Some(x) = cur_number {
                    cur_number = Some(x * 10 + new_ones_place);
                } else {
                    cur_number = Some(new_ones_place);
                }
            }
            ParsedSpace::Symbol(_) => {
                if let Some(n) = cur_number {
                    if let Some(gear_coord) = gear_symbol_coord {
                        let cur_vector = result.entry(gear_coord).or_insert(vec![]);
                        cur_vector.push(n);
                    }
                }
                gear_symbol_coord = None;
                cur_number = None;
            }
        }
        if let Some(_) = cur_number {
            if let None = gear_symbol_coord {
                // Check if we should be a part number
                for possible_loc in grid.get_adjacent_coordinates(coord) {
                    if let Some(_) = gear_symbol_coord {
                        break;
                    }
                    let z = grid.get_value(possible_loc).unwrap();
                    match z {
                        ParsedSpace::Symbol(s) => {
                            if s == symbol {
                                gear_symbol_coord = Some(possible_loc);
                            }
                        }
                        _ => {}
                    }
                }
                for possible_loc in grid.get_diag_adjacent_coordinates(coord) {
                    if let Some(_) = gear_symbol_coord {
                        break;
                    }
                    let z = grid.get_value(possible_loc).unwrap();
                    match z {
                        ParsedSpace::Symbol(s) => {
                            if s == symbol {
                                gear_symbol_coord = Some(possible_loc);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Check if we have a last value, if so add it
    if let Some(n) = cur_number {
        if let Some(gear_coord) = gear_symbol_coord {
            let cur_vector = result.entry(gear_coord).or_insert(vec![]);
            cur_vector.push(n);
        }
    }

    return result;
}

/// Find Gear ratios. Because these need to be beside exactly two part numbers, we can
/// simply use a dictionary with a key of the * coordinate, then filter it to only items
/// with exactly two items, and multiply the two of them.
/// ```
/// let vec1: Vec<String> = vec![
///    "467..114..",
///    "...*......",
///    "..35..633.",
///    "......#...",
///    "617*......",
///    ".....+.58.",
///    "..592.....",
///    "......755.",
///    "...$.*....",
///    ".664.598..",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 467835);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> PartNumber {
    let grid = parse_grid(string_list);
    let part_nums_by_gear_coords = get_parts_adjacent_to_symbol(&grid, '*');
    return part_nums_by_gear_coords
        .iter()
        .filter(|(_, value)| value.len() == 2)
        .map(|(_, value)| value.iter().product::<PartNumber>())
        .sum();
}
