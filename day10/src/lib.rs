extern crate filelib;

pub use filelib::load_no_blanks;
use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DEBUG: bool = false;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}", self.x, self.y)
    }
}

struct GraphManager {
    graph: Graph<Coord, u32, Directed>,
    coord_to_nodes: HashMap<Coord, NodeIndex<u32>>,
    edges: HashSet<(Coord, Coord)>,
}

impl GraphManager {
    fn has_edge(&self, from: Coord, to: Coord) -> bool {
        if self.edges.contains(&(from, to)) {
            return true;
        }
        return false;
    }

    fn add(&mut self, x: i32, y: i32) {
        let src_coord = Coord { x: x, y: y };
        if !self.coord_to_nodes.contains_key(&src_coord) {
            let src = self.graph.add_node(src_coord);
            self.coord_to_nodes.insert(src_coord, src);
        }
    }

    fn add_target(&mut self, x: i32, y: i32, x_t: i32, y_t: i32) {
        let src_coord = Coord { x: x, y: y };
        let tgt_coord = Coord { x: x_t, y: y_t };
        if self.has_edge(src_coord, tgt_coord) {
            return;
        }
        let tgt: NodeIndex<u32>;
        if !self.coord_to_nodes.contains_key(&tgt_coord) {
            tgt = self.graph.add_node(tgt_coord);
            self.coord_to_nodes.insert(tgt_coord, tgt);
        } else {
            tgt = *self.coord_to_nodes.get(&tgt_coord).unwrap();
        }
        let src: NodeIndex<u32> = *self.coord_to_nodes.get(&src_coord).unwrap();
        self.graph.add_edge(src, tgt, 1);
        self.edges.insert((src_coord, tgt_coord));
    }

    fn add_north_of(&mut self, x: i32, y: i32) {
        self.add_target(x, y, x, y - 1);
    }

    fn add_south_of(&mut self, x: i32, y: i32) {
        self.add_target(x, y, x, y + 1);
    }

    fn add_east_of(&mut self, x: i32, y: i32) {
        self.add_target(x, y, x + 1, y);
    }

    fn add_west_of(&mut self, x: i32, y: i32) {
        self.add_target(x, y, x - 1, y);
    }
}

fn parse_pipes(string_list: &Vec<String>) -> (Graph<Coord, u32, Directed, u32>, Coord) {
    let graph: Graph<Coord, u32, Directed, u32> = Graph::new();
    let mut origin_x: i32 = 0;
    let mut origin_y: i32 = 0;
    let mut manager = GraphManager {
        graph: graph,
        coord_to_nodes: HashMap::new(),
        edges: HashSet::new(),
    };
    // Unfortunately can't just add blindly undirected.
    // This can happen:
    // 7
    // F
    // What can I do about this. Make it directed.
    for (y, line) in string_list.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if DEBUG {
                println!("Parsing {}", c);
            }
            match c {
                '.' => continue,
                '|' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_north_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_south_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                '-' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_east_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_west_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                'L' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_east_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_north_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                'J' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_west_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_north_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                '7' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_west_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_south_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                'F' => {
                    manager.add(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_east_of(x.try_into().unwrap(), y.try_into().unwrap());
                    manager.add_south_of(x.try_into().unwrap(), y.try_into().unwrap());
                }
                'S' => {
                    origin_x = x.try_into().unwrap();
                    origin_y = y.try_into().unwrap();
                }
                _ => panic!("Unknown char '{}'", c),
            }
        }
    }

    // The start is guaranteed to have exactly two connections to it, find those and add the reverse
    for (a, b) in manager.edges.clone().iter() {
        // These are TO the origin
        if b.x == origin_x && b.y == origin_y {
            if a.x > origin_x {
                manager.add_east_of(origin_x, origin_y);
            }
            if a.x < origin_x {
                manager.add_west_of(origin_x, origin_y);
            }
            if a.y < origin_y {
                manager.add_north_of(origin_x, origin_y);
            }
            if a.x > origin_y {
                manager.add_south_of(origin_x, origin_y);
            }
        }
    }

    return (
        manager.graph,
        Coord {
            x: origin_x,
            y: origin_y,
        },
    );
}

/// How many steps along the loop does it take to get from S to the point farthest from the starting position?
/// So we need to think how we represent this, if you think of it as the space you are in is a node, and
/// the input represents its connections (eg: "-" means its connected to x - 1 and x + 1), then each node
/// can have, at most, two connections.
/// Then you can make that into an undirected graph, and it will "automatically" fill in what S should be.
/// But if you have a graph, you are looking for is a hamilton cycle, which is NP hard, so there must be
/// special properties here.
/// Listed in the problem is that there is a SINGLE loop, which means we can probably path find.
/// Specifically, from the origin there will be two points it can talk to. If we needed to prove this exists
/// we could use DFS, but because WE KNOW it exists, we just need to BFS, since we can't leave it.
/// Half the length of the BFS should be the answer.
/// ```
/// let vec1: Vec<String> = vec![
///     "..F7.",
///     ".FJ|.",
///     "SJ.L7",
///     "|F--J",
///     "LJ..."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_a(&vec1), 8);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let (mut graph, start_coord) = parse_pipes(string_list);
    // Get edges connected to origin
    let mut src = graph
        .node_indices()
        .find(|i| graph[*i] == start_coord)
        .unwrap();
    let mut length = 0;
    let mut bfs = Bfs::new(&graph, src);
    while let Some(nx) = bfs.next(&graph) {
        length += 1;
        let cur_coord: Coord = graph[nx];
    }
    if DEBUG {
        println!("Length: {}", length);
    }
    return length / 2;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_manager_edges() {
        let graph: Graph<Coord, u32, Directed, u32> = Graph::new();
        let mut gm = GraphManager {
            graph: graph,
            coord_to_nodes: HashMap::new(),
            edges: HashSet::new(),
        };
        gm.add(1, 2);
        gm.add(2, 2);
        assert_eq!(
            gm.has_edge(Coord { x: 1, y: 2 }, Coord { x: 2, y: 2 }),
            false
        );
        assert_eq!(
            gm.has_edge(Coord { x: 2, y: 2 }, Coord { x: 1, y: 2 }),
            false
        );
        gm.add_east_of(1, 2);
        assert_eq!(
            gm.has_edge(Coord { x: 1, y: 2 }, Coord { x: 2, y: 2 }),
            true
        );
        assert_eq!(
            gm.has_edge(Coord { x: 2, y: 2 }, Coord { x: 1, y: 2 }),
            false
        );
        gm.add_west_of(2, 2);
        assert_eq!(
            gm.has_edge(Coord { x: 1, y: 2 }, Coord { x: 2, y: 2 }),
            true
        );
        assert_eq!(
            gm.has_edge(Coord { x: 2, y: 2 }, Coord { x: 1, y: 2 }),
            true
        );
        assert_eq!(
            gm.has_edge(Coord { x: 1, y: 2 }, Coord { x: 7, y: 2 }),
            false
        );
    }

    #[test]
    fn test_square_loop() {
        let vec1: Vec<String> = vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(puzzle_a(&vec1), 4);
    }

    #[test]
    fn test_square_loop_parse() {
        let vec1: Vec<String> = vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (_, start_coord) = parse_pipes(&vec1);
        assert_eq!(start_coord.x, 1);
        assert_eq!(start_coord.y, 1);
    }

    #[test]
    fn test_complex_loop_parse() {
        let vec1: Vec<String> = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (_, start_coord) = parse_pipes(&vec1);
        assert_eq!(start_coord.x, 0);
        assert_eq!(start_coord.y, 2);
    }

    #[test]
    #[should_panic]
    fn test_parse_nonsense() {
        let vec1: Vec<String> = vec!["Z....", ".S-7.", ".|.|.", ".L-J.", "....."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (_, start_coord) = parse_pipes(&vec1);
        assert_eq!(start_coord.x, 1);
        assert_eq!(start_coord.y, 1);
    }
}
