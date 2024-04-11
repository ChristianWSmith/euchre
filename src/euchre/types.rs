use super::enums::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
