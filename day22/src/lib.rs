extern crate filelib;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

pub use filelib::load_no_blanks;

use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::Undirected;

// Bricks must remain above GROUND_Z, aka, they must be at least 1.
const GROUND_Z: u32 = 0;

type Coord = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ThreeCoord {
    x: Coord,
    y: Coord,
    z: Coord,
}

// 2,2,2~2,2,2 = single cube
// 0, 0, 10~1,0,10 or 0,0,10~0,1,10 are two cubes, horizontal (x or y)
// 0,0,1~0,0,10 Ten cubes, vertical
#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    front: ThreeCoord,
    back: ThreeCoord,
    debug_id: usize,
    cube_length: usize,
    cubes: HashSet<ThreeCoord>,
}

impl Brick {
    fn new(front: ThreeCoord, back: ThreeCoord, debug_id: usize) -> Brick {
        // Calc length
        let x_diff;
        let y_diff;
        let z_diff;

        if front.x > back.x {
            x_diff = front.x - back.x;
        } else {
            x_diff = back.x - front.x;
        }

        if front.y > back.y {
            y_diff = front.y - back.y;
        } else {
            y_diff = back.y - front.y;
        }

        if front.z > back.z {
            z_diff = front.z - back.z;
        } else {
            z_diff = back.z - front.z;
        }

        let cube_length = x_diff + y_diff + z_diff + 1;
        let cube_length_usize: usize = cube_length.try_into().unwrap();

        // Generate cubes
        let mut cubes: HashSet<ThreeCoord> = HashSet::new();

        if front.x != back.x {
            if front.x < back.x {
                for x in front.x..=back.x {
                    cubes.insert(ThreeCoord {
                        x: x,
                        y: front.y,
                        z: front.z,
                    });
                }
            } else {
                for x in back.x..=front.x {
                    cubes.insert(ThreeCoord {
                        x: x,
                        y: front.y,
                        z: front.z,
                    });
                }
            }
        } else if front.y != back.y {
            if front.y < back.y {
                for y in front.y..=back.y {
                    cubes.insert(ThreeCoord {
                        x: front.x,
                        y: y,
                        z: front.z,
                    });
                }
            } else {
                for y in back.y..=front.y {
                    cubes.insert(ThreeCoord {
                        x: front.x,
                        y: y,
                        z: front.z,
                    });
                }
            }
        } else if front.z != back.z {
            if front.z < back.z {
                for z in front.z..=back.z {
                    cubes.insert(ThreeCoord {
                        x: front.x,
                        y: front.y,
                        z: z,
                    });
                }
            } else {
                for z in back.z..=front.z {
                    cubes.insert(ThreeCoord {
                        x: front.x,
                        y: front.y,
                        z: z,
                    });
                }
            }
        }
        cubes.insert(front.clone());

        return Brick {
            front: front,
            back: back,
            debug_id: debug_id,
            cube_length: cube_length_usize,
            cubes: cubes,
        };
    }

    fn fall(&self) -> Brick {
        let mut front = self.front.clone();
        let mut back = self.back.clone();
        front.z -= 1;
        back.z -= 1;
        return Brick::new(front, back, self.debug_id);
    }

    fn does_intersect(&self, other: &Brick) -> bool {
        // Bricks, thankfully, can't be anything but straight
        // This makes it much easier to reason on what they are.
        let my_length = self.cube_length;
        let other_length = other.cube_length;

        // Consider a one cube - needs to be in the same coordinate
        if my_length == 1 && other_length == 1 {
            return self.front == other.front;
        }

        // There is clever math here, but just going to hack it and generate the cubes.
        return !self.cubes.is_disjoint(&other.cubes);
    }

    fn does_support(&self, other: &Brick) -> bool {
        // I support another brick, if it would fall and intersect me.
        let fallen = other.fall();
        return self.does_intersect(&fallen);
    }
}

impl Hash for Brick {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.debug_id.hash(state);
    }
}

fn partial_to_coord(s: &str) -> ThreeCoord {
    let (x, yz) = s.split_once(",").unwrap();
    let (y, z) = yz.split_once(",").unwrap();
    let x_c: Coord = x.parse().unwrap();
    let y_c: Coord = y.parse().unwrap();
    let z_c: Coord = z.parse().unwrap();

    return ThreeCoord {
        x: x_c,
        y: y_c,
        z: z_c,
    };
}

fn parse_bricks(snapshot: &Vec<String>) -> Vec<Brick> {
    let mut result = vec![];
    for line in snapshot {
        let (front, back) = line.split_once("~").unwrap();
        // Optimization that could be done here, define a "front" to "back" order,
        // and switch inputs to make it so.
        let f = partial_to_coord(front);
        let b = partial_to_coord(back);
        assert!(f.z <= b.z);
        let brick = Brick::new(f, b, result.len());
        result.push(brick);
    }
    return result;
}

fn simulate_fall(bricks: &Vec<Brick>) -> Vec<Brick> {
    // sort bricks by z, the lowest bricks will fall first
    let mut sorted_order = bricks.clone();
    sorted_order.sort_by(|a, b| {
        let a_z;
        let b_z;
        if a.front.z < a.back.z {
            a_z = a.front.z;
        } else {
            a_z = a.back.z;
        }

        if b.front.z < b.back.z {
            b_z = b.front.z;
        } else {
            b_z = b.back.z;
        }

        return a_z.cmp(&b_z);
    });
    let mut end: Vec<Brick> = vec![];
    let mut queue = VecDeque::from(sorted_order);

    while let Some(brick) = queue.pop_front() {
        // Try moving down one. If we cannot, check what we collide with
        // if that one can't move (its in the end queue, or is on the ground), we are done
        // otherwise, requeue. (At front!)
        // if we can move down, do so and requeue (at front!).
        if brick.front.z == GROUND_Z + 1 || brick.back.z == GROUND_Z + 1 {
            end.push(brick);
            continue;
        }
        let next_brick = brick.fall();
        let mut re_push = true;
        for done_brick in end.clone() {
            if done_brick.does_intersect(&next_brick) {
                // Also done before this fall.
                end.push(brick);
                re_push = false;
                break;
            }
        }
        if re_push {
            queue.push_front(next_brick);
        }
    }

    return end;
}

type BrickGraph = Graph<Brick, u32, Undirected>;

fn build_brick_graph(bricks: &Vec<Brick>) -> BrickGraph {
    let mut graph = Graph::new_undirected();
    let mut brick_map: HashMap<Brick, NodeIndex> = HashMap::new();
    let mut seen_edges: HashSet<(Brick, Brick)> = HashSet::new();

    for brick in bricks.clone() {
        let index = graph.add_node(brick);
        brick_map.insert(brick, index);
    }
    for brick in bricks.clone() {
        if brick.front.z == GROUND_Z + 1 || brick.back.z == GROUND_Z + 1 {
            // Treat nodes on the ground as root nodes.
            continue;
        }
        let index = brick_map.get(&brick).unwrap();

        for other_brick in bricks.clone() {
            if seen_edges.contains(&(other_brick, brick)) {
                continue;
            }
            if brick == other_brick {
                continue;
            }
            if other_brick.does_support(&brick) {
                let other_index = brick_map.get(&other_brick).unwrap();
                graph.add_edge(*index, *other_index, 1);
                seen_edges.insert((brick, other_brick));
            }
        }
    }

    /*
    println!("Copy this into https://viz-js.com/");
    println!("{:?}", Dot::new(&graph));
    */

    return graph;
}

fn find_safe_to_disintegrate(graph: &BrickGraph) -> u32 {
    // Check every node in the graph to those with one neighbor.
    // if that node doesn't have a brick on Z = 1, count it
    let end_nodes: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|n| {
            if graph[*n].front.z == GROUND_Z + 1 || graph[*n].back.z == GROUND_Z + 1 {
                //println!("ground {}", graph[*n].debug_id);
                return false;
            }
            if graph.neighbors(*n).count() != 1 {
                //println!("more than one connection {}", graph[*n].debug_id);
            }
            return graph.neighbors(*n).count() == 1;
        })
        .collect();
    let multiple_supports: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|n| {
            // for each neighbor, check if it has multiple supports. If it does, I can go.
            //println!("Checking neighbors of {}", graph[*n].debug_id);
            for m in graph.neighbors(*n) {
                if !graph[*n].does_support(&graph[m]) {
                    continue;
                }
                let mut support_count = 0;
                //println!("Checking neighbor {}", graph[m].debug_id);
                for l in graph.neighbors(m) {
                    if graph[l].does_support(&graph[m]) {
                        /*println!(
                            "Detected {} supports {}",
                            graph[l].debug_id, graph[m].debug_id
                        );*/
                        support_count += 1;
                    }
                }
                if support_count == 1 {
                    //println!("Multiple supports not found for neighbor of {}", graph[*n].debug_id);
                    return false;
                }
            }
            return true;
        })
        .collect();

    let mut set = HashSet::new();
    set.extend(end_nodes);
    set.extend(multiple_supports);
    return set.len().try_into().unwrap();
}

/// 3d bricks, how fun. Find which can be removed without causing a brick to fall.
/// ```
/// let vec1: Vec<String> = vec![
///     "1,0,1~1,2,1",
///     "0,0,2~2,0,2",
///     "0,2,3~2,2,3",
///     "0,0,4~0,2,4",
///     "2,0,5~2,2,5",
///     "0,1,6~2,1,6",
///     "1,1,8~1,1,9"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day22::puzzle_a(&vec1), 5);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let bricks = parse_bricks(string_list);
    //println!("Starting fall");
    let fallen = simulate_fall(&bricks);
    //println!("Starting graph");
    let graph = build_brick_graph(&fallen);
    //println!("Starting disintegrate");
    return find_safe_to_disintegrate(&graph);
}

/*
fn directed_graph(graph: &Graph) -> Graph {
    let g = Graph::new();

    return g;
}

type Cache = HashMap<NodeIndex, usize>;
fn get_support_structure(graph: &Graph, n: &NodeIndex, cache: Cache) -> usize {
    if cache.contains(n) {
        return cache.get(n).unwrap();
    }
    let mut num_supported = 0;

}
*/

/// Toplogical sort, and take the sum of everything but the last element.
/// ```
/// let vec1: Vec<String> = vec![
///     "1,0,1~1,2,1",
///     "0,0,2~2,0,2",
///     "0,2,3~2,2,3",
///     "0,0,4~0,2,4",
///     "2,0,5~2,2,5",
///     "0,1,6~2,1,6",
///     "1,1,8~1,1,9"
/// ].iter().map(|s| s.to_string()).collect();
/// //assert_eq!(day22::puzzle_b(&vec1), 7);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let bricks = parse_bricks(string_list);
    //println!("Starting fall");
    let fallen = simulate_fall(&bricks);
    //println!("Starting graph");
    let graph = build_brick_graph(&fallen);
    return 0;
    /*
    let support_graph = directed_graph(&graph);
    let mut cache = Cache::new();
    return graph.node_indices().map(|n| get_support_structure(&graph, n, &cache).len()).sum()
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_intersect() {
        let one_brick = Brick::new(
            ThreeCoord { x: 0, y: 0, z: 0 },
            ThreeCoord { x: 0, y: 0, z: 0 },
            0,
        );
        let far_away_brick = Brick::new(
            ThreeCoord {
                x: 77,
                y: 77,
                z: 77,
            },
            ThreeCoord {
                x: 99,
                y: 99,
                z: 99,
            },
            1,
        );
        assert_eq!(one_brick.does_intersect(&one_brick), true);
        assert_eq!(one_brick.does_intersect(&far_away_brick), false);
        assert_eq!(far_away_brick.does_intersect(&one_brick), false);
    }
}
