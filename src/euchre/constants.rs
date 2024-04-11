use super::{enums::*, types::*};
use lazy_static::lazy_static;
use strum::EnumCount;

lazy_static! {
    pub static ref BID_UPCARD_AVAILABLE_ACTIONS: [bool; ActionIndex::COUNT] = {
        let mut bid_upcard_available_actions: [bool; ActionIndex::COUNT] =
            [false; ActionIndex::COUNT];
        bid_upcard_available_actions[ActionIndex::MakeUpcard as usize] = true;
        bid_upcard_available_actions[ActionIndex::MakeUpcardAlone as usize] = true;
        bid_upcard_available_actions[ActionIndex::PassUpcard as usize] = true;
        bid_upcard_available_actions
    };
}

// Cards
// Spades
pub const CARD_SPADE_NINE: Card = Card {
    suit: Suit::Spade,
    rank: Rank::Nine,
};
pub const CARD_SPADE_TEN: Card = Card {
    suit: Suit::Spade,
    rank: Rank::Ten,
};
pub const CARD_SPADE_JACK: Card = Card {
    suit: Suit::Spade,
    rank: Rank::Jack,
};
pub const CARD_SPADE_QUEEN: Card = Card {
    suit: Suit::Spade,
    rank: Rank::Queen,
};
pub const CARD_SPADE_KING: Card = Card {
    suit: Suit::Spade,
    rank: Rank::King,
};
pub const CARD_SPADE_ACE: Card = Card {
    suit: Suit::Spade,
    rank: Rank::Ace,
};
// Hearts
pub const CARD_HEART_NINE: Card = Card {
    suit: Suit::Heart,
    rank: Rank::Nine,
};
pub const CARD_HEART_TEN: Card = Card {
    suit: Suit::Heart,
    rank: Rank::Ten,
};
pub const CARD_HEART_JACK: Card = Card {
    suit: Suit::Heart,
    rank: Rank::Jack,
};
pub const CARD_HEART_QUEEN: Card = Card {
    suit: Suit::Heart,
    rank: Rank::Queen,
};
pub const CARD_HEART_KING: Card = Card {
    suit: Suit::Heart,
    rank: Rank::King,
};
pub const CARD_HEART_ACE: Card = Card {
    suit: Suit::Heart,
    rank: Rank::Ace,
};
// Diamonds
pub const CARD_DIAMOND_NINE: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::Nine,
};
pub const CARD_DIAMOND_TEN: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::Ten,
};
pub const CARD_DIAMOND_JACK: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::Jack,
};
pub const CARD_DIAMOND_QUEEN: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::Queen,
};
pub const CARD_DIAMOND_KING: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::King,
};
pub const CARD_DIAMOND_ACE: Card = Card {
    suit: Suit::Diamond,
    rank: Rank::Ace,
};
// Clubs
pub const CARD_CLUB_NINE: Card = Card {
    suit: Suit::Club,
    rank: Rank::Nine,
};
pub const CARD_CLUB_TEN: Card = Card {
    suit: Suit::Club,
    rank: Rank::Ten,
};
pub const CARD_CLUB_JACK: Card = Card {
    suit: Suit::Club,
    rank: Rank::Jack,
};
pub const CARD_CLUB_QUEEN: Card = Card {
    suit: Suit::Club,
    rank: Rank::Queen,
};
pub const CARD_CLUB_KING: Card = Card {
    suit: Suit::Club,
    rank: Rank::King,
};
pub const CARD_CLUB_ACE: Card = Card {
    suit: Suit::Club,
    rank: Rank::Ace,
};

pub const DECK: [Card; Rank::COUNT * Suit::COUNT] = [
    CARD_SPADE_NINE,
    CARD_SPADE_TEN,
    CARD_SPADE_JACK,
    CARD_SPADE_QUEEN,
    CARD_SPADE_KING,
    CARD_SPADE_ACE,
    CARD_HEART_NINE,
    CARD_HEART_TEN,
    CARD_HEART_JACK,
    CARD_HEART_QUEEN,
    CARD_HEART_KING,
    CARD_HEART_ACE,
    CARD_DIAMOND_NINE,
    CARD_DIAMOND_TEN,
    CARD_DIAMOND_JACK,
    CARD_DIAMOND_QUEEN,
    CARD_DIAMOND_KING,
    CARD_DIAMOND_ACE,
    CARD_CLUB_NINE,
    CARD_CLUB_TEN,
    CARD_CLUB_JACK,
    CARD_CLUB_QUEEN,
    CARD_CLUB_KING,
    CARD_CLUB_ACE,
];

pub const POSITIONS: [Position; 4] = [
    Position::North,
    Position::East,
    Position::South,
    Position::West,
];
