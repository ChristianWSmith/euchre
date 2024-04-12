use super::{constants::*, enums::*, types::*};
use rand::seq::SliceRandom;
use strum::EnumCount;

// TODO: unstub
pub fn card_wins(card: &Card, winning_card: &Option<Card>, trump_suit: &Suit) -> bool {
    return false;
}

pub fn play_from_hand(hand: &mut [Option<Card>; 6], action: &ActionIndex) -> Card {
    let selected_card = match &action {
        // Spade
        ActionIndex::PlaySpadeNine => CARD_SPADE_NINE,
        ActionIndex::PlaySpadeTen => CARD_SPADE_TEN,
        ActionIndex::PlaySpadeJack => CARD_SPADE_JACK,
        ActionIndex::PlaySpadeQueen => CARD_SPADE_QUEEN,
        ActionIndex::PlaySpadeKing => CARD_SPADE_KING,
        ActionIndex::PlaySpadeAce => CARD_SPADE_ACE,
        // Heart
        ActionIndex::PlayHeartNine => CARD_HEART_NINE,
        ActionIndex::PlayHeartTen => CARD_HEART_TEN,
        ActionIndex::PlayHeartJack => CARD_HEART_JACK,
        ActionIndex::PlayHeartQueen => CARD_HEART_QUEEN,
        ActionIndex::PlayHeartKing => CARD_HEART_KING,
        ActionIndex::PlayHeartAce => CARD_HEART_ACE,
        // Diamond
        ActionIndex::PlayDiamondNine => CARD_DIAMOND_NINE,
        ActionIndex::PlayDiamondTen => CARD_DIAMOND_TEN,
        ActionIndex::PlayDiamondJack => CARD_DIAMOND_JACK,
        ActionIndex::PlayDiamondQueen => CARD_DIAMOND_QUEEN,
        ActionIndex::PlayDiamondKing => CARD_DIAMOND_KING,
        ActionIndex::PlayDiamondAce => CARD_DIAMOND_ACE,
        // Club
        ActionIndex::PlayClubNine => CARD_CLUB_NINE,
        ActionIndex::PlayClubTen => CARD_CLUB_TEN,
        ActionIndex::PlayClubJack => CARD_CLUB_JACK,
        ActionIndex::PlayClubQueen => CARD_CLUB_QUEEN,
        ActionIndex::PlayClubKing => CARD_CLUB_KING,
        ActionIndex::PlayClubAce => CARD_CLUB_ACE,
        _ => panic!("invalid play action attempted"),
    };
    for i in 0..hand.len() {
        if hand[i].is_some() && hand[i].unwrap() == selected_card {
            hand[i] = None;
            return selected_card;
        }
    }
    panic!("tried to play a card that is not in hand")
}

pub fn discard_from_hand(hand: &mut [Option<Card>; 6], action: &ActionIndex) {
    let selected_card = match &action {
        // Spade
        ActionIndex::DiscardSpadeNine => CARD_SPADE_NINE,
        ActionIndex::DiscardSpadeTen => CARD_SPADE_TEN,
        ActionIndex::DiscardSpadeJack => CARD_SPADE_JACK,
        ActionIndex::DiscardSpadeQueen => CARD_SPADE_QUEEN,
        ActionIndex::DiscardSpadeKing => CARD_SPADE_KING,
        ActionIndex::DiscardSpadeAce => CARD_SPADE_ACE,
        // Heart
        ActionIndex::DiscardHeartNine => CARD_HEART_NINE,
        ActionIndex::DiscardHeartTen => CARD_HEART_TEN,
        ActionIndex::DiscardHeartJack => CARD_HEART_JACK,
        ActionIndex::DiscardHeartQueen => CARD_HEART_QUEEN,
        ActionIndex::DiscardHeartKing => CARD_HEART_KING,
        ActionIndex::DiscardHeartAce => CARD_HEART_ACE,
        // Diamond
        ActionIndex::DiscardDiamondNine => CARD_DIAMOND_NINE,
        ActionIndex::DiscardDiamondTen => CARD_DIAMOND_TEN,
        ActionIndex::DiscardDiamondJack => CARD_DIAMOND_JACK,
        ActionIndex::DiscardDiamondQueen => CARD_DIAMOND_QUEEN,
        ActionIndex::DiscardDiamondKing => CARD_DIAMOND_KING,
        ActionIndex::DiscardDiamondAce => CARD_DIAMOND_ACE,
        // Club
        ActionIndex::DiscardClubNine => CARD_CLUB_NINE,
        ActionIndex::DiscardClubTen => CARD_CLUB_TEN,
        ActionIndex::DiscardClubJack => CARD_CLUB_JACK,
        ActionIndex::DiscardClubQueen => CARD_CLUB_QUEEN,
        ActionIndex::DiscardClubKing => CARD_CLUB_KING,
        ActionIndex::DiscardClubAce => CARD_CLUB_ACE,
        _ => panic!("invalid discard action attempted"),
    };
    for i in 0..hand.len() {
        if hand[i].is_some() && hand[i].unwrap() == selected_card {
            hand[i] = None;
            return;
        }
    }
    panic!("tried to discard a card that is not in hand")
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

pub fn deal() -> (
    [Option<Card>; 6],
    [Option<Card>; 6],
    [Option<Card>; 6],
    [Option<Card>; 6],
    Card,
) {
    let mut deck: [Card; Rank::COUNT * Suit::COUNT] = DECK.clone();
    deck.shuffle(&mut rand::thread_rng());
    return (
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
        deck[20],
    );
}
