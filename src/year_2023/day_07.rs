#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 7 };

mod utils {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    pub fn hand_type_no_jokers<'a, I>(
        mut sorted_card_counts: I,
    ) -> HandType
    where 
        I: Iterator<Item = &'a u8> 
    {
        match sorted_card_counts.next().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                match sorted_card_counts.next().unwrap() {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAKind,
                    _ => panic!("Unrecognized hand type."),
                }
            },
            2 => {
                match sorted_card_counts.next().unwrap() {
                    2 => HandType::TwoPair,
                    1 => HandType::OnePair,
                    _ => panic!("Unrecognized hand type."),
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Unrecognized hand type.")
        }
    }
}

pub mod part_one {
    use std::collections::HashMap;
    use itertools::Itertools;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{self, HandType};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl Card {
        fn from_char(card: char) -> Self {
            match card {
                '2' => Self::Two,
                '3' => Self::Three,
                '4' => Self::Four,
                '5' => Self::Five,
                '6' => Self::Six,
                '7' => Self::Seven,
                '8' => Self::Eight,
                '9' => Self::Nine,
                'T' => Self::Ten,
                'J' => Self::Jack,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => panic!("Unrecognized card.")
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand {
        hand_type: HandType,        
        cards: Vec<Card>,
    }

    impl Hand {
        fn from_str(hand: &str) -> Self {
            assert!(hand.len() == 5);
            let cards: Vec<Card> = hand.chars()
                .map(|card| Card::from_char(card))
                .collect();
            let mut card_counts: HashMap<Card, u8> = HashMap::new();
            for card in cards.iter() {
                card_counts.entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            let hand_type = utils::hand_type_no_jokers(
                card_counts.values().sorted().rev()
            );
            Self {
                cards,
                hand_type,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Deal {
        hand: Hand,
        bid: u32,
    }

    impl Deal {
        fn from_str(deal: &str) -> Self {
            let mut split = deal.split(" ");
            Deal {
                hand: Hand::from_str(split.next().unwrap()),
                bid: split.next().unwrap().parse().unwrap(),
            }
        }

        fn bid(&self) -> u32 {
            self.bid
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        deals: Vec<Deal>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.total_winnings())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| self.deals.push(Deal::from_str(&line)));
        }

        fn total_winnings(&mut self) -> u32 {
            self.deals.sort();
            self.deals.iter().enumerate()
                .map(|(rank, deal)| (rank as u32 + 1) * deal.bid())
                .sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(6_440); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::collections::HashMap;
    use itertools::Itertools;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{self, HandType};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl Card {
        fn from_char(card: char) -> Self {
            match card {
                '2' => Self::Two,
                '3' => Self::Three,
                '4' => Self::Four,
                '5' => Self::Five,
                '6' => Self::Six,
                '7' => Self::Seven,
                '8' => Self::Eight,
                '9' => Self::Nine,
                'T' => Self::Ten,
                'J' => Self::Joker,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => panic!("Unrecognized card.")
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand {
        hand_type: HandType,        
        cards: Vec<Card>,
    }

    impl Hand {
        fn from_str(hand: &str) -> Self {
            assert!(hand.len() == 5);
            let cards: Vec<Card> = hand.chars()
                .map(|card| Card::from_char(card))
                .collect();
            let mut card_counts: HashMap<Card, u8> = HashMap::new();
            for card in cards.iter() {
                card_counts.entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            let jokers = card_counts.remove(&Card::Joker);
            let mut sorted_card_counts = card_counts.values().sorted().rev();
            let hand_type = match jokers {
                None => utils::hand_type_no_jokers(sorted_card_counts),
                Some(count) => {
                    match count {
                        5 | 4 => HandType::FiveOfAKind,
                        3 => {
                            match sorted_card_counts.next().unwrap() {
                                2 => HandType::FiveOfAKind,
                                1 => HandType::FourOfAKind,
                                _ => panic!("Unrecognized hand type."),
                            }
                        },
                        2 => {
                            match sorted_card_counts.next().unwrap() {
                                3 => HandType::FiveOfAKind,
                                2 => HandType::FourOfAKind,
                                1 => HandType::ThreeOfAKind,
                                _ => panic!("Unrecognized hand type."),
                            }
                        },
                        1 => {
                            match sorted_card_counts.next().unwrap() {
                                4 => HandType::FiveOfAKind,
                                3 => HandType::FourOfAKind,
                                2 => {
                                    match sorted_card_counts.next().unwrap() {
                                        2 => HandType::FullHouse,
                                        1 => HandType::ThreeOfAKind,
                                        _ => panic!("Unrecognized hand type."),
                                    }
                                },
                                1 => HandType::OnePair,
                                _ => panic!("Unrecognized hand type."),
                            }
                        },
                        _ => panic!("Impossible number of jokers.")
                    }
                }
            };
            Self {
                cards,
                hand_type,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Deal {
        hand: Hand,
        bid: u32,
    }

    impl Deal {
        fn from_str(deal: &str) -> Self {
            let mut split = deal.split(" ");
            Deal {
                hand: Hand::from_str(split.next().unwrap()),
                bid: split.next().unwrap().parse().unwrap(),
            }
        }

        fn bid(&self) -> u32 {
            self.bid
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        deals: Vec<Deal>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.total_winnings())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| self.deals.push(Deal::from_str(&line)));
        }

        fn total_winnings(&mut self) -> u32 {
            self.deals.sort();
            self.deals.iter().enumerate()
                .map(|(rank, deal)| (rank as u32 + 1) * deal.bid())
                .sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(5_905); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}