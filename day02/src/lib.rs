extern crate filelib;

pub use filelib::load_no_blanks;

#[derive(PartialEq, Debug, Copy, Clone)]
struct BallCount {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(PartialEq, Debug, Clone)]
struct Game {
    id: usize,
    shown: Vec<BallCount>,
}

fn parse_games(string_list: &Vec<String>) -> Vec<Game> {
    let mut result: Vec<Game> = vec![];
    for s in string_list {
        //print!("{}", s);
        let (game_prefix, counts) = s.split_once(":").unwrap();
        let (_, game_id_s) = game_prefix.split_once("Game ").unwrap();
        let game_id: usize = game_id_s.parse().unwrap();
        let mut ball_counts: Vec<BallCount> = vec![];
        //print!("... Parsed game_id");
        for c in counts.split("; ") {
            // order is variable, and can be partial, need to split by comma and figure it out by ending.
            let mut cur_ball_count = BallCount {
                red: 0,
                blue: 0,
                green: 0,
            };
            if c.contains(", ") {
                for partial in c.split(", ") {
                    let trimed = partial.trim();
                    //print!("... parsing \"{}\"", partial);
                    if trimed.ends_with("blue") {
                        let (v_s, _) = trimed.split_once(" blue").unwrap();
                        let v: u32 = v_s.parse().unwrap();
                        cur_ball_count.blue = v;
                        //print!("... Parsed blue {}", v);
                    } else if trimed.ends_with("green") {
                        let (v_s, _) = trimed.split_once(" green").unwrap();
                        let v: u32 = v_s.parse().unwrap();
                        cur_ball_count.green = v;
                        //print!("... Parsed green {}", v);
                    } else if trimed.ends_with("red") {
                        let (v_s, _) = trimed.split_once(" red").unwrap();
                        let v: u32 = v_s.parse().unwrap();
                        cur_ball_count.red = v;
                        //print!("... Parsed red {}", v);
                    } else {
                        panic!("Could not parse");
                    }
                }
            } else {
                let trimed = c.trim();
                if trimed.ends_with("blue") {
                    let (v_s, _) = trimed.split_once(" blue").unwrap();
                    let v: u32 = v_s.parse().unwrap();
                    cur_ball_count.blue = v;
                    //print!("... Parsed single blue {}", v);
                } else if trimed.ends_with("green") {
                    let (v_s, _) = trimed.split_once(" green").unwrap();
                    let v: u32 = v_s.parse().unwrap();
                    cur_ball_count.green = v;
                    //print!("... Parsed single green {}", v);
                } else if trimed.ends_with("red") {
                    let (v_s, _) = trimed.split_once(" red").unwrap();
                    let v: u32 = v_s.parse().unwrap();
                    cur_ball_count.red = v;
                    //print!("... Parsed single red {}", v);
                } else {
                    panic!("Could not parse");
                }
            }
            ball_counts.push(cur_ball_count);
        }
        let game = Game {
            id: game_id,
            shown: ball_counts,
        };
        result.push(game);
    }
    return result;
}

/// Get sum of game ids that are possible with 12 red cubes, 13 green cubes, 14 blue cubes
/// ```
/// let vec1: Vec<String> = vec![
///     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
///     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
///     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
///     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
///     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_a(&vec1), 8);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let games: Vec<Game> = parse_games(string_list);
    return games
        .iter()
        .filter(|g| game_possible_max_cubes(red_cubes, green_cubes, blue_cubes, g))
        .map(|g| g.id)
        .sum();
}

fn game_possible_max_cubes(red: u32, green: u32, blue: u32, game: &Game) -> bool {
    return game
        .shown
        .iter()
        .all(|b| b.red <= red && b.green <= green && b.blue <= blue);
}

pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    return 1;
}
