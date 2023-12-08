extern crate filelib;

pub use filelib::load_no_blanks;
use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

fn direction_to_cost(d: Direction) -> u32 {
    match d {
        Direction::Left => 0,
        Direction::Right => 1,
    }
}

// Return (instructions, adjacency list)
fn parse_instructions(string_list: &Vec<String>) -> (Vec<Direction>, Graph<&str, u32>) {
    let mut iter = string_list.iter();
    let instruction_line = iter.next().unwrap();
    let mut directions: Vec<Direction> = vec![];
    for c in instruction_line.chars() {
        let d = match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Bad char"),
        };
        directions.push(d);
    }

    let mut graph = Graph::new(); // directed and unlabeled
    let mut seen_nodes = HashSet::new();
    // Graph construction
    for line in iter {
        let (identifier, values1) = line.split_once(" = ").unwrap();
        let (_, values2) = values1.split_once("(").unwrap();
        let (values3, _) = values2.split_once(")").unwrap();
        let (left_value, right_value) = values3.split_once(", ").unwrap();
        if !seen_nodes.contains(identifier) {
            seen_nodes.insert(identifier);
            graph.add_node(identifier);
        }
        if !seen_nodes.contains(left_value) {
            seen_nodes.insert(left_value);
            graph.add_node(left_value);
        }
        if !seen_nodes.contains(right_value) {
            seen_nodes.insert(right_value);
            graph.add_node(right_value);
        }
        let src = graph
            .node_indices()
            .find(|i| graph[*i] == identifier)
            .unwrap();
        let left = graph
            .node_indices()
            .find(|i| graph[*i] == left_value)
            .unwrap();
        let right = graph
            .node_indices()
            .find(|i| graph[*i] == right_value)
            .unwrap();
        graph.add_edge(src, left, direction_to_cost(Direction::Left));
        graph.add_edge(src, right, direction_to_cost(Direction::Right));
    }
    return (directions, graph);
}

/// Follow instructions from AAA until ZZZ
/// ```
/// let vec1: Vec<String> = vec![
///    "LLR",
///    "AAA = (BBB, BBB)",
///    "BBB = (AAA, ZZZ)",
///    "ZZZ = (ZZZ, ZZZ)",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1), 6);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let (ins, graph) = parse_instructions(string_list);
    let origin = graph.node_indices().find(|i| graph[*i] == "AAA").unwrap();
    let end = graph.node_indices().find(|i| graph[*i] == "ZZZ").unwrap();
    let mut count = 0;
    let mut cur_node = origin;
    for dir in ins.iter().cycle() {
        if cur_node == end {
            break;
        }
        count += 1;
        let cur_cost = direction_to_cost(*dir);

        for e in graph.edges(cur_node) {
            if *e.weight() == cur_cost {
                // Found our edge, traverse
                cur_node = e.target();
                break;
            }
        }
    }
    return count;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}
