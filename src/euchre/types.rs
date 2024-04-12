use super::enums::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
