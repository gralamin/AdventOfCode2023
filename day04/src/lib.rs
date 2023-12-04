extern crate filelib;

pub use filelib::load_no_blanks;

#[derive(PartialEq, Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

fn parse_cards(string_list: &Vec<String>) -> Vec<Card> {
    let mut result: Vec<Card> = vec![];

    for card_line in string_list {
        //println!("Parsing {}", card_line);
        let (card_lead, card_end) = card_line.split_once(":").unwrap();
        let (_, card_id_s) = card_lead.split_once("Card ").unwrap();
        let card_id = card_id_s.trim().parse().unwrap();
        //println!("Done card id");

        // Split ids
        let (winning_numbers_s, scratched_numbers_s) = card_end.split_once(" | ").unwrap();

        // winning numbers
        let mut winning_numbers: Vec<u32> = vec![];
        for number_s in winning_numbers_s.split(" ") {
            if number_s.len() == 0 {
                continue;
            }
            let num = number_s.trim().parse().unwrap();
            winning_numbers.push(num);
        }
        //println!("Done winning numbers");

        // scratched numbers
        let mut scratched_numbers: Vec<u32> = vec![];
        for number_s in scratched_numbers_s.split(" ") {
            if number_s.len() == 0 {
                continue;
            }
            let num = number_s.trim().parse().unwrap();
            scratched_numbers.push(num);
        }
        //println!("Done scratched numbers");

        let card = Card {
            id: card_id,
            winning_numbers: winning_numbers,
            scratched_numbers: scratched_numbers,
        };
        result.push(card);
    }

    return result;
}

fn get_points_for_card(card: &Card) -> u32 {
    let matched_numbers: Vec<&u32> = card
        .scratched_numbers
        .iter()
        .filter(|&scratched| card.winning_numbers.contains(scratched))
        .collect();
    //println!("Matched numbers for {} - {:#?}", card.id, matched_numbers);
    if matched_numbers.len() == 0 {
        return 0;
    }
    return 2_u32.pow((matched_numbers.len() - 1).try_into().unwrap());
}

/// Get card points, a card's points is equal to the square of the count of matches.
/// ```
/// let vec1: Vec<String> = vec![
///    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
///    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
///    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
///    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
///    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
///    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_a(&vec1), 13);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let parsed_cards = parse_cards(string_list);
    return parsed_cards
        .iter()
        .map(|card| get_points_for_card(&card))
        .sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}
