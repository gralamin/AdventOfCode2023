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
    Joker,
}

fn card_from_char(c: char, with_joker: bool) -> CamelCard {
    let mut j_card = CamelCard::Jack;
    if with_joker {
        j_card = CamelCard::Joker;
    }
    return match c {
        'A' => CamelCard::Ace,
        'K' => CamelCard::King,
        'Q' => CamelCard::Queen,
        'J' => j_card,
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
    };
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
    let mut jokers = 0;
    for card in cards {
        if *card == CamelCard::Joker {
            jokers += 1;
            continue;
        }
        *seen.entry(*card).or_insert(0_u8) += 1_u8;
    }
    if jokers == 0 {
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

    // With jokers logic
    if seen.len() == 1 {
        // all jokers just add to five of a kind.
        return HandType::FiveOfKind;
    }
    if seen.len() == 2 {
        // Four scenarios:
        // JJJAB -> Four of kind
        // JJAAB -> Four of kind
        // JAABB -> FullHouse
        // JAAAB -> Four of kind
        // This becomes four of a kind unless both values equal and equal to 2.
        let first_value = seen.values().nth(0).unwrap();
        let second_value = seen.values().nth(1).unwrap();
        if *first_value == *second_value && *first_value == 2_u8 {
            return HandType::FullHouse;
        }
        return HandType::FourOfKind;
    }
    if seen.len() == 3 {
        // JJABC -> Three of a kind, since its better than two pair.
        // JAABC -> Three of a kind
        // Just get highest count.
        return HandType::ThreeOfKind;
    }
    if seen.len() == 4 {
        // JABCD -> One pair
        return HandType::OnePair;
    }
    // Must be five jokers
    return HandType::FiveOfKind;
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

fn parse_hands(string_list: &Vec<String>, joker: bool) -> Vec<Hand> {
    let mut result: Vec<Hand> = vec![];
    for line in string_list {
        let (cards_to_parse, value) = line.split_once(" ").unwrap();
        let bid: u32 = value.trim().parse().unwrap();
        let cards: Vec<CamelCard> = cards_to_parse
            .chars()
            .map(|c| card_from_char(c, joker))
            .collect();
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
    let mut hands = parse_hands(string_list, false);
    hands.sort();
    return hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| usize_to_u32(index + 1) * hand.bid)
        .sum();
}

/// As 1, but parse as jokers instead.
/// ```
/// let vec1: Vec<String> = vec![
///     "32T3K 765",
///     "T55J5 684",
///     "KK677 28",
///     "KTJJT 220",
///     "QQQJA 483"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_b(&vec1), 5905);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    let mut hands = parse_hands(string_list, true);
    hands.sort();
    println!("{:#?}", hands);
    return hands
        .iter()
        .rev()
        .enumerate()
        .map(|(index, hand)| usize_to_u32(index + 1) * hand.bid)
        .sum();
    // Answer should be 243101568
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_parse() {
        let vec1: Vec<String> = vec!["QQQJA 483"].iter().map(|s| s.to_string()).collect();
        let hands = parse_hands(&vec1, false);
        let hand = hands.first().unwrap();
        assert_eq!(hand.hand_type, HandType::ThreeOfKind);
        assert_eq!(hand.bid, 483);
    }

    #[test]
    fn tst_parse_joker() {
        let vec1: Vec<String> = vec!["QQQJA 483"].iter().map(|s| s.to_string()).collect();
        let hands = parse_hands(&vec1, true);
        let hand = hands.first().unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfKind);
        assert_eq!(hand.bid, 483);
    }

    #[test]
    fn tst_card_type_four_joker() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FiveOfKind);
    }

    #[test]
    fn tst_card_type_three_joker() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FourOfKind);
    }

    fn tst_card_type_three_joker_to_five() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FiveOfKind);
    }

    #[test]
    fn tst_card_type_two_joker_to_four() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Three,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FourOfKind);
    }

    #[test]
    fn tst_card_type_two_joker_to_three() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Four,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::ThreeOfKind);
    }

    #[test]
    fn tst_card_type_one_joker_to_full() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FullHouse);
    }

    #[test]
    fn tst_card_type_one_joker_to_pair() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Four,
            CamelCard::Five,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::OnePair);
    }

    #[test]
    fn tst_card_type_high_card() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Four,
            CamelCard::Five,
            CamelCard::Nine,
        ];
        assert_eq!(find_hand_type(&hand), HandType::HighCard);
    }

    #[test]
    fn tst_card_type_plain_four() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Nine,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FourOfKind);
    }

    #[test]
    fn tst_card_type_plain_five() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FiveOfKind);
    }

    #[test]
    fn tst_card_type_fullhouse() {
        let hand = vec![
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Three,
            CamelCard::Two,
            CamelCard::Two,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FullHouse);
    }

    #[test]
    fn tst_card_type_five_jokers() {
        let hand = vec![
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
            CamelCard::Joker,
        ];
        assert_eq!(find_hand_type(&hand), HandType::FiveOfKind);
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

    #[test]
    fn tst_sort_joker() {
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
                CamelCard::Joker,
                CamelCard::Five,
            ],
            hand_type: HandType::FourOfKind,
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
                CamelCard::Joker,
                CamelCard::Joker,
                CamelCard::Ten,
            ],
            hand_type: HandType::FourOfKind,
        };

        let hand_five = Hand {
            bid: 483,
            cards: vec![
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Joker,
                CamelCard::Ace,
            ],
            hand_type: HandType::FourOfKind,
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
            vec![hand_four, hand_five, hand_two, hand_three, hand_one]
        )
    }

    #[test]
    fn additional_char_scenarios() {
        assert_eq!(CamelCard::Nine, card_from_char('9', false));
        assert_eq!(CamelCard::Four, card_from_char('4', false));
        assert_eq!(CamelCard::Eight, card_from_char('8', false));
    }

    #[test]
    #[should_panic]
    fn invalid_character_in_parsing() {
        card_from_char('z', false);
    }
}
