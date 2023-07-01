/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
///
use std::collections::HashMap;
use std::error::Error;

const RADIX: u32 = 10;
const SIMPLE_STRAIGHT: &[i8; 4] = &[1, 1, 1, 1];
const LOW_STRAIGHT: &[i8; 4] = &[1, 1, 1, 9];
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum HandRank {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
}

impl Suit {
    pub fn match_suit(symbol: char) -> Suit {
        match symbol {
            'H' => Self::Hearts,
            'C' => Self::Clubs,
            'S' => Self::Spades,
            'D' => Self::Diamonds,
            _ => panic!("Invalid symbol!"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Card<'a> {
    card_name: &'a str,
    rank: u8,
    suit: Suit,
}

impl<'a> Card<'a> {
    pub fn map_alphabetic_types(value: char) -> u8 {
        match value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            _ => panic!("Invalid alphabetic type"),
        }
    }

    pub fn new(card_name: &'a str) -> Self {
        let mut chars = card_name.chars();
        let value = chars.next().unwrap();

        let card_name_length = card_name.len();
        let (rank, suit) = card_name.split_at(card_name_length - 1);
        let suit = Suit::match_suit(suit.chars().next().unwrap());

        let result = u8::from_str_radix(rank, RADIX);

        let number = match result {
            Ok(number) => number,
            Err(_) => Self::map_alphabetic_types(value),
        };

        Card {
            card_name: card_name,
            rank: number,
            suit: suit,
        }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    hand_rank: HandRank,
    score: u8,
    composite_score: u8,
}

impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        let mut cards = hand
            .split_whitespace()
            .map(|card| Card::new(card))
            .collect::<Vec<Card>>();
        cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        let (hand_rank, composite_score) = find_hand_rank(&mut cards);
        let scores = cards.iter().map(|card| card.rank).collect::<Vec<u8>>();
        let score = scores.iter().sum();
        Hand {
            hand: hand,
            hand_rank,
            score: score,
            composite_score: composite_score,
        }
    }
}

fn first_difference(cards: &Vec<Card>) -> Vec<i8> {
    let cards = cards;
    let mut ranks = cards.iter().map(|card| card.rank).collect::<Vec<u8>>();
    ranks.sort();
    ranks
        .iter()
        .take(4) // hand-coded knowing that each hand is 5 cards
        .enumerate()
        .map(|(index, val)| cards[index + 1].rank.clone() as i8 - val.clone() as i8)
        .collect::<Vec<i8>>()
}

fn find_hand_rank(cards: &mut Vec<Card>) -> (HandRank, u8) {
    let mut suit_count = HashMap::new();
    let mut rank_count = HashMap::new();

    cards.sort_by(|a, b| a.rank.cmp(&b.rank));

    cards.iter().for_each(|card| {
        *rank_count.entry(card.rank).or_insert(0_u8) += 1_u8;
        *suit_count.entry(&card.suit).or_insert(0_u8) += 1_u8
    });

    let mut rank_count_clone = rank_count.clone();
    rank_count_clone.retain(|_, value| *value >= 2_u8);

    let pairs_exist = !rank_count_clone.is_empty();

    let first_difference = first_difference(cards);
    let (possible_straight, straight_score) = if first_difference == SIMPLE_STRAIGHT.to_vec() {
        (true, 2_u8)
    } else if first_difference == LOW_STRAIGHT.to_vec() {
        (true, 1_u8)
    } else {
        (false, 0_u8)
    };

    let length_of_suit_count = suit_count.len();

    let (hand_rank, composite_score) = match (pairs_exist, possible_straight) {
        (true, false) => {
            let result = rank_of_duplicates(&rank_count_clone);
            match result {
                Ok(_) => result.ok().unwrap(),
                Err(_) => panic!(
                    "Pairs with no straights, hand rank unidentified! {:?}",
                    cards
                ),
            }
        }
        (true, true) => panic!("Impossible hand for a standard 52-card deck! {:?}", cards),
        (false, true) => {
            if length_of_suit_count == 1 {
                (HandRank::StraightFlush, straight_score)
            } else {
                (HandRank::Straight, straight_score)
            }
        }
        (false, false) => {
            if length_of_suit_count == 1 {
                (HandRank::Flush, 0)
            } else {
                (HandRank::HighCard, cards.last().unwrap().rank)
            }
        }
    };

    (hand_rank, composite_score)
}

fn rank_of_duplicates(
    duplicates_count: &HashMap<u8, u8>,
) -> Result<(HandRank, u8), Box<dyn Error>> {
    let count_of_duplicates = duplicates_count.len();

    match count_of_duplicates {
        1 => {
            let only_option = duplicates_count.iter().next();
            match only_option {
                Some((key, value)) => match value {
                    2 => Ok((HandRank::OnePair, *key)),
                    3 => Ok((HandRank::ThreeOfAKind, *key)),
                    4 => Ok((HandRank::FourOfAKind, *key)),
                    _ => panic!(
                        "Rank of duplicates cannot find any pairs of one duplicated rank! {:?}",
                        duplicates_count
                    ),
                },
                None => panic!(
                    "Rank of duplicates found None pairs of one duplicated rank! {:?}",
                    duplicates_count
                ),
            }
        }

        2 => {
            let mut iterator = duplicates_count.iter();
            let first_option = iterator.next();
            let second_option = iterator.next();
            match (first_option, second_option) {
                (Some((first_key, first_value)), Some((second_key, second_value))) => {
                    match (first_value, second_value) {
                        (2, 2) => Ok((HandRank::TwoPair, *(first_key.max(second_key)))),
                        (3, 2) => Ok((HandRank::FullHouse, *first_key)),
                        (2, 3) => Ok((HandRank::FullHouse, *second_key)),
                        _ => panic!(
                            "Rank of duplicates could not find a flush or a two pair! {:?}",
                            duplicates_count
                        ),
                    }
                }

                _ => panic!(
                    "Rank of duplicates found 2 duplicates with at least 1 None! {:?}",
                    duplicates_count
                ),
            }
        }
        _ => panic!("Invalid number of duplicates! {:?}", duplicates_count),
    }
}

pub fn winning_hands<'a>(hand_strings: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hand_strings
        .iter()
        .map(|h| Hand::new(h))
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.hand_rank.cmp(&b.hand_rank));

    match hands.last() {
        Some(hand) => {
            let hand_rank = hand.hand_rank;
            let mut highest_rank_hands = hands
                .iter()
                .filter(|hand| hand.hand_rank.eq(&hand_rank))
                .collect::<Vec<&Hand>>();
            highest_rank_hands.sort_by(|a, b| a.composite_score.cmp(&b.composite_score));
            let highest_composite_score = highest_rank_hands.last().unwrap().composite_score;
            let mut highest_composite_score_hands = highest_rank_hands
                .iter()
                .filter(|hand| hand.composite_score == highest_composite_score)
                .map(|hand| *hand)
                .collect::<Vec<&Hand>>();
            highest_composite_score_hands.sort_by(|a, b| a.score.cmp(&b.score));
            let highest_score = highest_composite_score_hands.last().unwrap().score;
            highest_composite_score_hands
                .iter()
                .filter(|hand| hand.score == highest_score)
                .map(|hand| hand.hand)
                .collect::<Vec<&str>>()
        }
        None => hand_strings.to_vec(),
    }
}
