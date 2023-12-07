extern crate filelib;

pub use filelib::load_no_blanks;
use std::cmp::Ordering;
use std::collections::HashMap;

// Sorts such that Ace comes first.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum CamelCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

fn card_from_char(c: char) -> CamelCard {
    match c {
        'A' => CamelCard::Ace,
        'K' => CamelCard::King,
        'Q' => CamelCard::Queen,
        'J' => CamelCard::Jack,
        'T' => CamelCard::Ten,
        '9' => CamelCard::Nine,
        '8' => CamelCard::Eight,
        '7' => CamelCard::Seven,
        '6' => CamelCard::Six,
        '5' => CamelCard::Five,
        '4' => CamelCard::Four,
        '3' => CamelCard::Three,
        '2' => CamelCard::Two,
        _ => panic!("Bad character passed in"),
    }
}

// Sorts such that Five of a kind comes first
#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn find_hand_type(cards: &Vec<CamelCard>) -> HandType {
    let mut seen: HashMap<CamelCard, u8> = HashMap::new();
    for card in cards {
        *seen.entry(*card).or_insert(0_u8) += 1_u8;
    }
    if seen.len() == 1 {
        // Must be five of a kind
        return HandType::FiveOfKind;
    }
    if seen.len() == 2 {
        // Either four of a kind, or full house.
        let first_value = seen.values().nth(0).unwrap();
        if *first_value == 1_u8 || *first_value == 4_u8 {
            return HandType::FourOfKind;
        }
        return HandType::FullHouse;
    }
    if seen.len() == 3 {
        // Two pair, three of a kind
        // we can tell by looking for a value of three
        if seen
            .values()
            .filter(|&x| *x == 3_u8)
            .collect::<Vec<_>>()
            .len()
            == 1
        {
            return HandType::ThreeOfKind;
        }
        return HandType::TwoPair;
    }
    if seen.len() == 4 {
        // One pair
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    bid: u32,
    cards: Vec<CamelCard>,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hand_type, self.cards.clone()).cmp(&(other.hand_type, other.cards.clone()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn parse_hands(string_list: &Vec<String>) -> Vec<Hand> {
    let mut result: Vec<Hand> = vec![];
    for line in string_list {
        let (cards_to_parse, value) = line.split_once(" ").unwrap();
        let bid: u32 = value.trim().parse().unwrap();
        let cards: Vec<CamelCard> = cards_to_parse.chars().map(|c| card_from_char(c)).collect();
        let hand_type = find_hand_type(&cards);
        let hand = Hand {
            bid: bid,
            cards: cards,
            hand_type: hand_type,
        };
        result.push(hand);
    }
    return result;
}

fn usize_to_u32(i: usize) -> u32 {
    return i.try_into().unwrap();
}

/// Order the hands by their rank, then multiply with their bid. Rank is determined by strength.
/// ```
/// let vec1: Vec<String> = vec![
///     "32T3K 765",
///     "T55J5 684",
///     "KK677 28",
///     "KTJJT 220",
///     "QQQJA 483"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_a(&vec1), 6440);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u32 {
    let mut hands = parse_hands(string_list);
    hands.sort();
    return hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| usize_to_u32(index + 1) * hand.bid)
        .sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_parse() {
        let vec1: Vec<String> = vec!["QQQJA 483"].iter().map(|s| s.to_string()).collect();
        let hands = parse_hands(&vec1);
        let hand = hands.first().unwrap();
        assert_eq!(hand.hand_type, HandType::ThreeOfKind);
        assert_eq!(hand.bid, 483);
    }

    #[test]
    fn tst_sort() {
        let hand_one = Hand {
            bid: 765,
            cards: vec![
                CamelCard::Three,
                CamelCard::Two,
                CamelCard::Ten,
                CamelCard::Three,
                CamelCard::King,
            ],
            hand_type: HandType::OnePair,
        };

        let hand_two = Hand {
            bid: 684,
            cards: vec![
                CamelCard::Ten,
                CamelCard::Five,
                CamelCard::Five,
                CamelCard::Jack,
                CamelCard::Five,
            ],
            hand_type: HandType::ThreeOfKind,
        };

        let hand_three = Hand {
            bid: 28,
            cards: vec![
                CamelCard::King,
                CamelCard::King,
                CamelCard::Six,
                CamelCard::Seven,
                CamelCard::Seven,
            ],
            hand_type: HandType::TwoPair,
        };

        let hand_four = Hand {
            bid: 220,
            cards: vec![
                CamelCard::King,
                CamelCard::Ten,
                CamelCard::Jack,
                CamelCard::Jack,
                CamelCard::Ten,
            ],
            hand_type: HandType::TwoPair,
        };

        let hand_five = Hand {
            bid: 483,
            cards: vec![
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Jack,
                CamelCard::Ace,
            ],
            hand_type: HandType::ThreeOfKind,
        };

        let mut hands = vec![
            hand_one.clone(),
            hand_two.clone(),
            hand_three.clone(),
            hand_four.clone(),
            hand_five.clone(),
        ];
        hands.sort();
        assert_eq!(
            hands,
            vec![hand_five, hand_two, hand_three, hand_four, hand_one]
        )
    }
}
