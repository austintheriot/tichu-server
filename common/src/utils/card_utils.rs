use crate::{
    BombOf4, Card, FullHouse, Pair, Sequence, SequenceBomb, SequenceOfPairs, Single, Trio,
    ValidCardCombos,
};

// TODO: account for single special cards and Phoenix wild card
pub fn get_card_combination(cards: &Vec<Card>) -> Option<ValidCardCombos> {
    let mut cards = cards.clone();
    sort_cards_for_hand(&mut cards);

    // length 0: no cards
    if cards.is_empty() {
        return None;
    }

    // length 1: a single card
    if cards.len() == 1 {
        return Some(ValidCardCombos::Single(Single(
            cards.get(0).unwrap().clone(),
        )));
    }

    // length 2: a pair of cards of equal rank
    // OR 1 standard card and 1 Phoenix
    if cards.len() == 2 {
        if let [card_0, card_1] = &cards[..cards.len()] {
            return if card_0.value == card_1.value {
                Some(ValidCardCombos::Pair(Pair {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
                }))
            } else {
                None
            };
        }

        return None;
    }

    // length 3: a trio of cards of equal rank
    if cards.len() == 3 {
        if let [card_0, card_1, card_2] = &cards[..cards.len()] {
            return if card_0.value == card_1.value && card_1.value == card_2.value {
                Some(ValidCardCombos::Trio(Trio {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
                }))
            } else {
                None
            };
        }

        return None;
    }

    // length 4:
    if cards.len() == 4 {
        // a bomb (4 of the same)
        if let [card_0, card_1, card_2, card_3] = &cards[..cards.len()] {
            return if card_0.value == card_1.value
                && card_1.value == card_2.value
                && card_2.value == card_3.value
            {
                Some(ValidCardCombos::BombOf4(BombOf4 {
                    cards: cards.clone(),
                    value: card_0.value.clone(),
                }))
            }
            // sequence of 2 pairs
            else if (card_0.value == card_1.value && card_2.value == card_3.value)
                || (card_0.value == card_2.value && card_1.value == card_3.value)
                || (card_0.value == card_3.value && card_1.value == card_2.value)
            {
                let mut least_value = &cards.get(0).unwrap().value;
                cards.iter().for_each(|card| {
                    if card.value < *least_value {
                        least_value = &card.value;
                    }
                });
                Some(ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    cards: cards.clone(),
                    number_of_pairs: 2,
                    starting_value: least_value.clone(),
                }))
            } else {
                None
            };
        }

        return None;
    }

    // length 5:
    // a full house (trio + pair)
    // a sequence of length at least 5
    // a bomb (sequence of 5, all same suit)
    if cards.len() == 5 {
        // full house (first 3 are equal)
        return if let [card_0, card_1, card_2, card_3, card_4] = &cards[..cards.len()] {
            if (card_0.value == card_1.value && card_0.value == card_2.value)
                && (card_3.value == card_4.value)
            {
                Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: cards.clone(),
                    trio_value: card_0.value.clone(),
                }))
            }
            // full house (last 3 are equal)
            else if (card_2.value == card_3.value && card_2.value == card_4.value)
                && (card_0.value == card_1.value)
            {
                Some(ValidCardCombos::FullHouse(FullHouse {
                    cards: cards.clone(),
                    trio_value: card_2.value.clone(),
                }))
            }
            // sequences
            else if card_0.value.add_one() == card_1.value
                && card_1.value.add_one() == card_2.value
                && card_2.value.add_one() == card_3.value
                && card_3.value.add_one() == card_4.value
            {
                if card_0.suit == card_1.suit
                    && card_0.suit == card_2.suit
                    && card_0.suit == card_3.suit
                    && card_0.suit == card_4.suit
                {
                    // bomb sequence of length 5
                    Some(ValidCardCombos::SequenceBomb(SequenceBomb {
                        cards: cards.clone(),
                        number_of_cards: 5,
                        starting_value: card_0.value.clone(),
                        suit: card_0.suit.clone(),
                    }))
                } else {
                    // plain sequence of length at least 5
                    Some(ValidCardCombos::Sequence(Sequence {
                        cards: cards.clone(),
                        number_of_cards: 5,
                        starting_value: card_0.value.clone(),
                    }))
                }
            } else {
                None
            }
        } else {
            None
        };
    }

    // any length greater than 5:
    // a sequence
    // a sequence of pairs of adjacent value

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{get_card_combination, Card, CardSuit, CardValue};

    mod test_get_random_string_of_len {
        use super::super::get_card_combination;
        use crate::{
            BombOf4, Card, CardSuit, CardValue, FullHouse, Pair, Sequence, SequenceBomb,
            SequenceOfPairs, Single, Trio, ValidCardCombos,
        };

        #[test]
        fn it_should_return_some_for_correct_combos() {
            // a single card
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Sword,
                        value: CardValue(2),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                })))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Pagoda,
                        value: CardValue(14),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14),
                })))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![Card {
                        suit: CardSuit::Dragon,
                        value: CardValue::noop(),
                    }])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Single(Single(Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                })))
            );

            // a pair of cards of equal rank
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(7),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(13),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(13),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(13),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Pair(Pair {
                    value: CardValue(14),
                    cards: vec![ /* omitted */],
                }))
            );

            // a trio of cards of equal rank
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(3),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(3),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(11),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(11),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(11),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Trio(Trio {
                    value: CardValue(14),
                    cards: vec![ /* omitted */],
                }))
            );

            // a sequence of pairs of adjacent value
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(15),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(15),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(14),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(3),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(2),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(8),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(8),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceOfPairs(SequenceOfPairs {
                    starting_value: CardValue(7),
                    number_of_pairs: 2,
                    cards: vec![ /* omitted */],
                }))
            );

            // a bomb (4 of the same)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(7),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::BombOf4(BombOf4 {
                    value: CardValue(7),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(2),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(2),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::BombOf4(BombOf4 {
                    value: CardValue(2),
                    cards: vec![ /* omitted */],
                }))
            );
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(14),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::BombOf4(BombOf4 {
                    value: CardValue(14),
                    cards: vec![ /* omitted */],
                }))
            );

            // a full house (trio + pair)
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(14),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(15),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(15),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::FullHouse(FullHouse {
                    cards: vec![ /* omitted */],
                    trio_value: CardValue(14),
                }))
            );

            // a plain sequence of length 5
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Star,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Jade,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Sword,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::Sequence(Sequence {
                    cards: vec![ /* omitted */],
                    number_of_cards: 5,
                    starting_value: CardValue(3),
                }))
            );

            // sequence bomb
            assert_eq!(
                std::mem::discriminant(
                    &get_card_combination(&vec![
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(3),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(4),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(5),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(6),
                        },
                        Card {
                            suit: CardSuit::Pagoda,
                            value: CardValue(7),
                        },
                    ])
                    .unwrap()
                ),
                std::mem::discriminant(&ValidCardCombos::SequenceBomb(SequenceBomb {
                    cards: vec![ /* omitted */],
                    number_of_cards: 5,
                    starting_value: CardValue(3),
                    suit: CardSuit::Pagoda,
                }))
            );

            // sequence of pairs of adjacent value (any length)
            // length 4
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 6
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 8
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // length 10
            assert_eq!(get_card_combination(&vec![]).is_some(), false);

            // TODO: every combo as possible with a Phoenix:
            // length 2: a pair of cards of equal rank
            // OR 1 standard card and 1 Phoenix

            // length 3: a trio of cards of equal rank

            // length 4:
            // a bomb (4 of the same)
            // a sequence of pairs of adjacent value

            // length 5:
            // a full house (trio + pair)
            // a sequence of length at least 5
            // a bomb (sequence of 5, all same suit)

            // any length greater than 5:
            // a sequence
            // a sequence of pairs of adjacent value
        }
    }

    #[test]
    fn it_should_return_none_for_bogus_combos() {
        assert_eq!(get_card_combination(&vec![]), None);

        // two different cards
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(3),
                }
            ]),
            None
        );

        // non-Phoenix special card and standard card:
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Dog,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Star,
                    value: CardValue(11),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::MahJong,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(6),
                }
            ]),
            None
        );
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Dragon,
                    value: CardValue::noop(),
                },
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(14),
                }
            ]),
            None
        );

        // 3: 2 same value and 3rd non-matching
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(5),
                }
            ]),
            None
        );
        // 3: 3 different values
        assert_eq!(
            get_card_combination(&vec![
                Card {
                    suit: CardSuit::Pagoda,
                    value: CardValue(2),
                },
                Card {
                    suit: CardSuit::Sword,
                    value: CardValue(3),
                },
                Card {
                    suit: CardSuit::Jade,
                    value: CardValue(4),
                }
            ]),
            None
        );
    }
}

pub fn sort_cards_for_hand(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| {
        if a.value == b.value {
            [&a.suit].cmp(&[&b.suit])
        } else {
            [&a.value].cmp(&[&b.value])
        }
    });
}

#[cfg(test)]
mod test_sort_cards_for_hand {
    use crate::{sort_cards_for_hand, Card, CardSuit, CardValue, Deck};

    #[test]
    fn it_should_sort_for_hand_correctly() {
        let mut deck = Deck::new();
        deck.shuffle();
        sort_cards_for_hand(&mut deck.0);

        assert_eq!(
            deck.0.get(0),
            Some(&Card {
                suit: CardSuit::MahJong,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(1),
            Some(&Card {
                suit: CardSuit::Dog,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(2),
            Some(&Card {
                suit: CardSuit::Phoenix,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(3),
            Some(&Card {
                suit: CardSuit::Dragon,
                value: CardValue::noop(),
            })
        );

        assert_eq!(
            deck.0.get(4),
            Some(&Card {
                suit: CardSuit::Sword,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(5),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(6),
            Some(&Card {
                suit: CardSuit::Pagoda,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(7),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(2),
            })
        );

        assert_eq!(
            deck.0.get(8),
            Some(&Card {
                suit: CardSuit::Sword,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(9),
            Some(&Card {
                suit: CardSuit::Jade,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(10),
            Some(&Card {
                suit: CardSuit::Pagoda,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(11),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(3),
            })
        );

        assert_eq!(
            deck.0.get(55),
            Some(&Card {
                suit: CardSuit::Star,
                value: CardValue(14),
            })
        );
    }
}
