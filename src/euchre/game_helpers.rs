use super::{constants::*, enums::*, types::*};
use rand::seq::SliceRandom;
use strum::EnumCount;

pub fn get_trick_winner() -> RelativePosition {
    RelativePosition::Myself
}

pub fn left_player(player: &Position) -> &Position {
    match player {
        Position::North => return &Position::East,
        Position::East => return &Position::South,
        Position::South => return &Position::West,
        Position::West => return &Position::North,
        _ => panic!("invalid position"),
    }
}

pub fn deal() -> ([[Option<Card>; 6]; 4], Card) {
    let mut deck: [Card; Rank::COUNT * Suit::COUNT] = DECK.clone();
    deck.shuffle(&mut rand::thread_rng());
    return (
        [
            [
                Some(deck[0]),
                Some(deck[1]),
                Some(deck[2]),
                Some(deck[3]),
                Some(deck[4]),
                None,
            ],
            [
                Some(deck[5]),
                Some(deck[6]),
                Some(deck[7]),
                Some(deck[8]),
                Some(deck[9]),
                None,
            ],
            [
                Some(deck[10]),
                Some(deck[11]),
                Some(deck[12]),
                Some(deck[13]),
                Some(deck[14]),
                None,
            ],
            [
                Some(deck[15]),
                Some(deck[16]),
                Some(deck[17]),
                Some(deck[18]),
                Some(deck[19]),
                None,
            ],
        ],
        deck[20],
    );
}
