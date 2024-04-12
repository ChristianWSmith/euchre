use super::{constants::*, enums::*, types::*};
use crate::{AvailableActions, NeuralNetworkInput};
use strum::EnumCount;

pub fn get_play_available_actions(
    hand: &[Option<Card>; 6],
    lead_suit: &Option<Suit>,
) -> AvailableActions {
    let mut available_actions: [bool; ActionIndex::COUNT] = [false; ActionIndex::COUNT];

    let mut must_follow = false;

    match lead_suit {
        Some(lead_suit) => {
            for card in hand {
                if card.is_some() && card.unwrap().suit == *lead_suit {
                    must_follow = true;
                    break;
                }
            }
        }
        _ => {}
    }

    for card in hand {
        match card {
            Some(card) => {
                if must_follow && card.suit != lead_suit.unwrap() {
                    continue;
                }
                match *card {
                    // Spade
                    CARD_SPADE_NINE => {
                        available_actions[ActionIndex::PlaySpadeNine as usize] = true
                    }
                    CARD_SPADE_TEN => available_actions[ActionIndex::PlaySpadeTen as usize] = true,
                    CARD_SPADE_JACK => {
                        available_actions[ActionIndex::PlaySpadeJack as usize] = true
                    }
                    CARD_SPADE_QUEEN => {
                        available_actions[ActionIndex::PlaySpadeQueen as usize] = true
                    }
                    CARD_SPADE_KING => {
                        available_actions[ActionIndex::PlaySpadeKing as usize] = true
                    }
                    CARD_SPADE_ACE => available_actions[ActionIndex::PlaySpadeAce as usize] = true,
                    // Heart
                    CARD_HEART_NINE => {
                        available_actions[ActionIndex::PlayHeartNine as usize] = true
                    }
                    CARD_HEART_TEN => available_actions[ActionIndex::PlayHeartTen as usize] = true,
                    CARD_HEART_JACK => {
                        available_actions[ActionIndex::PlayHeartJack as usize] = true
                    }
                    CARD_HEART_QUEEN => {
                        available_actions[ActionIndex::PlayHeartQueen as usize] = true
                    }
                    CARD_HEART_KING => {
                        available_actions[ActionIndex::PlayHeartKing as usize] = true
                    }
                    CARD_HEART_ACE => available_actions[ActionIndex::PlayHeartAce as usize] = true,
                    // Diamond
                    CARD_DIAMOND_NINE => {
                        available_actions[ActionIndex::PlayDiamondNine as usize] = true
                    }
                    CARD_DIAMOND_TEN => {
                        available_actions[ActionIndex::PlayDiamondTen as usize] = true
                    }
                    CARD_DIAMOND_JACK => {
                        available_actions[ActionIndex::PlayDiamondJack as usize] = true
                    }
                    CARD_DIAMOND_QUEEN => {
                        available_actions[ActionIndex::PlayDiamondQueen as usize] = true
                    }
                    CARD_DIAMOND_KING => {
                        available_actions[ActionIndex::PlayDiamondKing as usize] = true
                    }
                    CARD_DIAMOND_ACE => {
                        available_actions[ActionIndex::PlayDiamondAce as usize] = true
                    }
                    // Club
                    CARD_CLUB_NINE => available_actions[ActionIndex::PlayClubNine as usize] = true,
                    CARD_CLUB_TEN => available_actions[ActionIndex::PlayClubTen as usize] = true,
                    CARD_CLUB_JACK => available_actions[ActionIndex::PlayClubJack as usize] = true,
                    CARD_CLUB_QUEEN => {
                        available_actions[ActionIndex::PlayClubQueen as usize] = true
                    }
                    CARD_CLUB_KING => available_actions[ActionIndex::PlayClubKing as usize] = true,
                    CARD_CLUB_ACE => available_actions[ActionIndex::PlayClubAce as usize] = true,
                    _ => panic!("invalid card"),
                }
            }
            _ => {}
        }
    }

    return available_actions;
}

// TODO: unstub
pub fn set_trick_lead(
    input: &mut NeuralNetworkInput,
    relative_position: &RelativePosition,
    trick_index: &TrickIndex,
) {
}

pub fn set_trick_lead_suit(
    input: &mut NeuralNetworkInput,
    lead_suit: &Option<Suit>,
    trick_index: &TrickIndex,
) {
    match lead_suit {
        Some(lead_suit) => match (lead_suit, trick_index) {
            (Suit::Spade, TrickIndex::First) => {
                input[StateIndex::Trick1LeadSuitSpade as usize] = 1.0
            }
            (Suit::Heart, TrickIndex::First) => {
                input[StateIndex::Trick1LeadSuitHeart as usize] = 1.0
            }
            (Suit::Diamond, TrickIndex::First) => {
                input[StateIndex::Trick1LeadSuitDiamond as usize] = 1.0
            }
            (Suit::Club, TrickIndex::First) => input[StateIndex::Trick1LeadSuitClub as usize] = 1.0,
            (Suit::Spade, TrickIndex::Second) => {
                input[StateIndex::Trick2LeadSuitSpade as usize] = 1.0
            }
            (Suit::Heart, TrickIndex::Second) => {
                input[StateIndex::Trick2LeadSuitHeart as usize] = 1.0
            }
            (Suit::Diamond, TrickIndex::Second) => {
                input[StateIndex::Trick2LeadSuitDiamond as usize] = 1.0
            }
            (Suit::Club, TrickIndex::Second) => {
                input[StateIndex::Trick2LeadSuitClub as usize] = 1.0
            }
            (Suit::Spade, TrickIndex::Third) => {
                input[StateIndex::Trick3LeadSuitSpade as usize] = 1.0
            }
            (Suit::Heart, TrickIndex::Third) => {
                input[StateIndex::Trick3LeadSuitHeart as usize] = 1.0
            }
            (Suit::Diamond, TrickIndex::Third) => {
                input[StateIndex::Trick3LeadSuitDiamond as usize] = 1.0
            }
            (Suit::Club, TrickIndex::Third) => input[StateIndex::Trick3LeadSuitClub as usize] = 1.0,
            (Suit::Spade, TrickIndex::Fourth) => {
                input[StateIndex::Trick4LeadSuitSpade as usize] = 1.0
            }
            (Suit::Heart, TrickIndex::Fourth) => {
                input[StateIndex::Trick4LeadSuitHeart as usize] = 1.0
            }
            (Suit::Diamond, TrickIndex::Fourth) => {
                input[StateIndex::Trick4LeadSuitDiamond as usize] = 1.0
            }
            (Suit::Club, TrickIndex::Fourth) => {
                input[StateIndex::Trick4LeadSuitClub as usize] = 1.0
            }
            _ => panic!("invalid lead suit and trick index combination"),
        },
        _ => return,
    }
}

// TODO: unstub
pub fn set_trick_card_played(
    input: &mut NeuralNetworkInput,
    trick_index: &TrickIndex,
    trick_card_index: &TrickCardIndex,
) {
}

pub fn set_trick_count(
    input: &mut NeuralNetworkInput,
    ally_trick_count: &u8,
    enemy_trick_count: &u8,
    set_to_value: &f64,
) {
    match ally_trick_count {
        0 => input[StateIndex::AllyTrickCount0 as usize] = *set_to_value,
        1 => input[StateIndex::AllyTrickCount1 as usize] = *set_to_value,
        2 => input[StateIndex::AllyTrickCount2 as usize] = *set_to_value,
        3 => input[StateIndex::AllyTrickCount3 as usize] = *set_to_value,
        4 => input[StateIndex::AllyTrickCount4 as usize] = *set_to_value,
        _ => panic!("invalid ally trick count"),
    }
    match enemy_trick_count {
        0 => input[StateIndex::EnemyTrickCount0 as usize] = *set_to_value,
        1 => input[StateIndex::EnemyTrickCount1 as usize] = *set_to_value,
        2 => input[StateIndex::EnemyTrickCount2 as usize] = *set_to_value,
        3 => input[StateIndex::EnemyTrickCount3 as usize] = *set_to_value,
        4 => input[StateIndex::EnemyTrickCount4 as usize] = *set_to_value,
        _ => panic!("invalid enemy trick count"),
    }
}

pub fn get_bid_suit_available_actions(suit: &Suit) -> AvailableActions {
    let mut available_actions: [bool; ActionIndex::COUNT] = [false; ActionIndex::COUNT];
    available_actions[ActionIndex::PassSuit as usize] = true;
    if *suit != Suit::Spade {
        available_actions[ActionIndex::MakeSuitSpade as usize] = true;
        available_actions[ActionIndex::MakeSuitSpadeAlone as usize] = true;
    }
    if *suit != Suit::Heart {
        available_actions[ActionIndex::MakeSuitHeart as usize] = true;
        available_actions[ActionIndex::MakeSuitHeartAlone as usize] = true;
    }
    if *suit != Suit::Diamond {
        available_actions[ActionIndex::MakeSuitDiamond as usize] = true;
        available_actions[ActionIndex::MakeSuitDiamondAlone as usize] = true;
    }
    if *suit != Suit::Club {
        available_actions[ActionIndex::MakeSuitClub as usize] = true;
        available_actions[ActionIndex::MakeSuitClubAlone as usize] = true;
    }
    available_actions
}

pub fn get_discard_available_actions(hand: &[Option<Card>; 6]) -> AvailableActions {
    let mut available_actions: [bool; ActionIndex::COUNT] = [false; ActionIndex::COUNT];
    for card in *hand {
        match card {
            // Spade
            Some(CARD_SPADE_NINE) => {
                available_actions[ActionIndex::DiscardSpadeNine as usize] = true
            }
            Some(CARD_SPADE_TEN) => available_actions[ActionIndex::DiscardSpadeTen as usize] = true,
            Some(CARD_SPADE_JACK) => {
                available_actions[ActionIndex::DiscardSpadeJack as usize] = true
            }
            Some(CARD_SPADE_QUEEN) => {
                available_actions[ActionIndex::DiscardSpadeQueen as usize] = true
            }
            Some(CARD_SPADE_KING) => {
                available_actions[ActionIndex::DiscardSpadeKing as usize] = true
            }
            Some(CARD_SPADE_ACE) => available_actions[ActionIndex::DiscardSpadeAce as usize] = true,
            // Heart
            Some(CARD_HEART_NINE) => {
                available_actions[ActionIndex::DiscardHeartNine as usize] = true
            }
            Some(CARD_HEART_TEN) => available_actions[ActionIndex::DiscardHeartTen as usize] = true,
            Some(CARD_HEART_JACK) => {
                available_actions[ActionIndex::DiscardHeartJack as usize] = true
            }
            Some(CARD_HEART_QUEEN) => {
                available_actions[ActionIndex::DiscardHeartQueen as usize] = true
            }
            Some(CARD_HEART_KING) => {
                available_actions[ActionIndex::DiscardHeartKing as usize] = true
            }
            Some(CARD_HEART_ACE) => available_actions[ActionIndex::DiscardHeartAce as usize] = true,
            // Diamond
            Some(CARD_DIAMOND_NINE) => {
                available_actions[ActionIndex::DiscardDiamondNine as usize] = true
            }
            Some(CARD_DIAMOND_TEN) => {
                available_actions[ActionIndex::DiscardDiamondTen as usize] = true
            }
            Some(CARD_DIAMOND_JACK) => {
                available_actions[ActionIndex::DiscardDiamondJack as usize] = true
            }
            Some(CARD_DIAMOND_QUEEN) => {
                available_actions[ActionIndex::DiscardDiamondQueen as usize] = true
            }
            Some(CARD_DIAMOND_KING) => {
                available_actions[ActionIndex::DiscardDiamondKing as usize] = true
            }
            Some(CARD_DIAMOND_ACE) => {
                available_actions[ActionIndex::DiscardDiamondAce as usize] = true
            }
            // Club
            Some(CARD_CLUB_NINE) => available_actions[ActionIndex::DiscardClubNine as usize] = true,
            Some(CARD_CLUB_TEN) => available_actions[ActionIndex::DiscardClubTen as usize] = true,
            Some(CARD_CLUB_JACK) => available_actions[ActionIndex::DiscardClubJack as usize] = true,
            Some(CARD_CLUB_QUEEN) => {
                available_actions[ActionIndex::DiscardClubQueen as usize] = true
            }
            Some(CARD_CLUB_KING) => available_actions[ActionIndex::DiscardClubKing as usize] = true,
            Some(CARD_CLUB_ACE) => available_actions[ActionIndex::DiscardClubAce as usize] = true,
            None => {}
            _ => panic!("invalid card"),
        }
    }
    available_actions
}

pub fn set_bid_suit(
    input: &mut NeuralNetworkInput,
    relative_position: &RelativePosition,
    action: &ActionIndex,
) {
    match (relative_position, action) {
        // Myself
        (&RelativePosition::Myself, &ActionIndex::MakeSuitSpade) => {
            input[StateIndex::BidSuitMyselfMakeSpade as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitSpadeAlone) => {
            input[StateIndex::BidSuitMyselfMakeAloneSpade as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitHeart) => {
            input[StateIndex::BidSuitMyselfMakeHeart as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitHeartAlone) => {
            input[StateIndex::BidSuitMyselfMakeAloneHeart as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitDiamond) => {
            input[StateIndex::BidSuitMyselfMakeDiamond as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitDiamondAlone) => {
            input[StateIndex::BidSuitMyselfMakeAloneDiamond as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitClub) => {
            input[StateIndex::BidSuitMyselfMakeClub as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::MakeSuitClubAlone) => {
            input[StateIndex::BidSuitMyselfMakeAloneClub as usize] = 1.0;
        }
        (&RelativePosition::Myself, &ActionIndex::PassSuit) => {
            input[StateIndex::BidSuitMyselfPass as usize] = 1.0;
        }
        // Left
        (&RelativePosition::Left, &ActionIndex::MakeSuitSpade) => {
            input[StateIndex::BidSuitLeftMakeSpade as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitSpadeAlone) => {
            input[StateIndex::BidSuitLeftMakeAloneSpade as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitHeart) => {
            input[StateIndex::BidSuitLeftMakeHeart as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitHeartAlone) => {
            input[StateIndex::BidSuitLeftMakeAloneHeart as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitDiamond) => {
            input[StateIndex::BidSuitLeftMakeDiamond as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitDiamondAlone) => {
            input[StateIndex::BidSuitLeftMakeAloneDiamond as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitClub) => {
            input[StateIndex::BidSuitLeftMakeClub as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::MakeSuitClubAlone) => {
            input[StateIndex::BidSuitLeftMakeAloneClub as usize] = 1.0;
        }
        (&RelativePosition::Left, &ActionIndex::PassSuit) => {
            input[StateIndex::BidSuitLeftPass as usize] = 1.0;
        }
        // Ally
        (&RelativePosition::Ally, &ActionIndex::MakeSuitSpade) => {
            input[StateIndex::BidSuitAllyMakeSpade as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitSpadeAlone) => {
            input[StateIndex::BidSuitAllyMakeAloneSpade as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitHeart) => {
            input[StateIndex::BidSuitAllyMakeHeart as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitHeartAlone) => {
            input[StateIndex::BidSuitAllyMakeAloneHeart as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitDiamond) => {
            input[StateIndex::BidSuitAllyMakeDiamond as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitDiamondAlone) => {
            input[StateIndex::BidSuitAllyMakeAloneDiamond as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitClub) => {
            input[StateIndex::BidSuitAllyMakeClub as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::MakeSuitClubAlone) => {
            input[StateIndex::BidSuitAllyMakeAloneClub as usize] = 1.0;
        }
        (&RelativePosition::Ally, &ActionIndex::PassSuit) => {
            input[StateIndex::BidSuitAllyPass as usize] = 1.0;
        }
        // Right
        (&RelativePosition::Right, &ActionIndex::MakeSuitSpade) => {
            input[StateIndex::BidSuitRightMakeSpade as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitSpadeAlone) => {
            input[StateIndex::BidSuitRightMakeAloneSpade as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitHeart) => {
            input[StateIndex::BidSuitRightMakeHeart as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitHeartAlone) => {
            input[StateIndex::BidSuitRightMakeAloneHeart as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitDiamond) => {
            input[StateIndex::BidSuitRightMakeDiamond as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitDiamondAlone) => {
            input[StateIndex::BidSuitRightMakeAloneDiamond as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitClub) => {
            input[StateIndex::BidSuitRightMakeClub as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::MakeSuitClubAlone) => {
            input[StateIndex::BidSuitRightMakeAloneClub as usize] = 1.0;
        }
        (&RelativePosition::Right, &ActionIndex::PassSuit) => {
            input[StateIndex::BidSuitRightPass as usize] = 1.0;
        }
        _ => panic!("invalid relative position or suit bid action"),
    }
}

pub fn set_bid_upcard(
    input: &mut NeuralNetworkInput,
    relative_position: &RelativePosition,
    action: &ActionIndex,
) {
    match (relative_position, action) {
        // Myself
        (&RelativePosition::Myself, &ActionIndex::MakeUpcard) => {
            input[StateIndex::BidUpcardMyselfMake as usize] = 1.0
        }
        (&RelativePosition::Myself, &ActionIndex::MakeUpcardAlone) => {
            input[StateIndex::BidUpcardMyselfMakeAlone as usize] = 1.0
        }
        (&RelativePosition::Myself, &ActionIndex::PassUpcard) => {
            input[StateIndex::BidUpcardMyselfPass as usize] = 1.0
        }
        // Left
        (&RelativePosition::Left, &ActionIndex::MakeUpcard) => {
            input[StateIndex::BidUpcardLeftMake as usize] = 1.0
        }
        (&RelativePosition::Left, &ActionIndex::MakeUpcardAlone) => {
            input[StateIndex::BidUpcardLeftMakeAlone as usize] = 1.0
        }
        (&RelativePosition::Left, &ActionIndex::PassUpcard) => {
            input[StateIndex::BidUpcardLeftPass as usize] = 1.0
        }
        // Ally
        (&RelativePosition::Ally, &ActionIndex::MakeUpcard) => {
            input[StateIndex::BidUpcardAllyMake as usize] = 1.0
        }
        (&RelativePosition::Ally, &ActionIndex::MakeUpcardAlone) => {
            input[StateIndex::BidUpcardAllyMakeAlone as usize] = 1.0
        }
        (&RelativePosition::Ally, &ActionIndex::PassUpcard) => {
            input[StateIndex::BidUpcardAllyPass as usize] = 1.0
        }
        // Right
        (&RelativePosition::Right, &ActionIndex::MakeUpcard) => {
            input[StateIndex::BidUpcardRightMake as usize] = 1.0
        }
        (&RelativePosition::Right, &ActionIndex::MakeUpcardAlone) => {
            input[StateIndex::BidUpcardRightMakeAlone as usize] = 1.0
        }
        (&RelativePosition::Right, &ActionIndex::PassUpcard) => {
            input[StateIndex::BidUpcardRightPass as usize] = 1.0
        }
        _ => panic!("invalid relative position or upcard bid action"),
    }
}

pub fn set_upcard(input: &mut NeuralNetworkInput, card: &Card) {
    match *card {
        // Spade
        CARD_SPADE_NINE => input[StateIndex::UpcardSpadeNine as usize] = 1.0,
        CARD_SPADE_TEN => input[StateIndex::UpcardSpadeTen as usize] = 1.0,
        CARD_SPADE_JACK => input[StateIndex::UpcardSpadeJack as usize] = 1.0,
        CARD_SPADE_QUEEN => input[StateIndex::UpcardSpadeQueen as usize] = 1.0,
        CARD_SPADE_KING => input[StateIndex::UpcardSpadeKing as usize] = 1.0,
        CARD_SPADE_ACE => input[StateIndex::UpcardSpadeAce as usize] = 1.0,
        // Heart
        CARD_HEART_NINE => input[StateIndex::UpcardHeartNine as usize] = 1.0,
        CARD_HEART_TEN => input[StateIndex::UpcardHeartTen as usize] = 1.0,
        CARD_HEART_JACK => input[StateIndex::UpcardHeartJack as usize] = 1.0,
        CARD_HEART_QUEEN => input[StateIndex::UpcardHeartQueen as usize] = 1.0,
        CARD_HEART_KING => input[StateIndex::UpcardHeartKing as usize] = 1.0,
        CARD_HEART_ACE => input[StateIndex::UpcardHeartAce as usize] = 1.0,
        // Diamond
        CARD_DIAMOND_NINE => input[StateIndex::UpcardDiamondNine as usize] = 1.0,
        CARD_DIAMOND_TEN => input[StateIndex::UpcardDiamondTen as usize] = 1.0,
        CARD_DIAMOND_JACK => input[StateIndex::UpcardDiamondJack as usize] = 1.0,
        CARD_DIAMOND_QUEEN => input[StateIndex::UpcardDiamondQueen as usize] = 1.0,
        CARD_DIAMOND_KING => input[StateIndex::UpcardDiamondKing as usize] = 1.0,
        CARD_DIAMOND_ACE => input[StateIndex::UpcardDiamondAce as usize] = 1.0,
        // Club
        CARD_CLUB_NINE => input[StateIndex::UpcardClubNine as usize] = 1.0,
        CARD_CLUB_TEN => input[StateIndex::UpcardClubTen as usize] = 1.0,
        CARD_CLUB_JACK => input[StateIndex::UpcardClubJack as usize] = 1.0,
        CARD_CLUB_QUEEN => input[StateIndex::UpcardClubQueen as usize] = 1.0,
        CARD_CLUB_KING => input[StateIndex::UpcardClubKing as usize] = 1.0,
        CARD_CLUB_ACE => input[StateIndex::UpcardClubAce as usize] = 1.0,
        _ => panic!("invalid card"),
    }
}

pub fn set_hand(input: &mut NeuralNetworkInput, hand: &[Option<Card>; 6]) {
    for card in *hand {
        match card {
            // Spade
            Some(CARD_SPADE_NINE) => input[StateIndex::HandSpadeNine as usize] = 1.0,
            Some(CARD_SPADE_TEN) => input[StateIndex::HandSpadeTen as usize] = 1.0,
            Some(CARD_SPADE_JACK) => input[StateIndex::HandSpadeJack as usize] = 1.0,
            Some(CARD_SPADE_QUEEN) => input[StateIndex::HandSpadeQueen as usize] = 1.0,
            Some(CARD_SPADE_KING) => input[StateIndex::HandSpadeKing as usize] = 1.0,
            Some(CARD_SPADE_ACE) => input[StateIndex::HandSpadeAce as usize] = 1.0,
            // Heart
            Some(CARD_HEART_NINE) => input[StateIndex::HandHeartNine as usize] = 1.0,
            Some(CARD_HEART_TEN) => input[StateIndex::HandHeartTen as usize] = 1.0,
            Some(CARD_HEART_JACK) => input[StateIndex::HandHeartJack as usize] = 1.0,
            Some(CARD_HEART_QUEEN) => input[StateIndex::HandHeartQueen as usize] = 1.0,
            Some(CARD_HEART_KING) => input[StateIndex::HandHeartKing as usize] = 1.0,
            Some(CARD_HEART_ACE) => input[StateIndex::HandHeartAce as usize] = 1.0,
            // Diamond
            Some(CARD_DIAMOND_NINE) => input[StateIndex::HandDiamondNine as usize] = 1.0,
            Some(CARD_DIAMOND_TEN) => input[StateIndex::HandDiamondTen as usize] = 1.0,
            Some(CARD_DIAMOND_JACK) => input[StateIndex::HandDiamondJack as usize] = 1.0,
            Some(CARD_DIAMOND_QUEEN) => input[StateIndex::HandDiamondQueen as usize] = 1.0,
            Some(CARD_DIAMOND_KING) => input[StateIndex::HandDiamondKing as usize] = 1.0,
            Some(CARD_DIAMOND_ACE) => input[StateIndex::HandDiamondAce as usize] = 1.0,
            // Club
            Some(CARD_CLUB_NINE) => input[StateIndex::HandClubNine as usize] = 1.0,
            Some(CARD_CLUB_TEN) => input[StateIndex::HandClubTen as usize] = 1.0,
            Some(CARD_CLUB_JACK) => input[StateIndex::HandClubJack as usize] = 1.0,
            Some(CARD_CLUB_QUEEN) => input[StateIndex::HandClubQueen as usize] = 1.0,
            Some(CARD_CLUB_KING) => input[StateIndex::HandClubKing as usize] = 1.0,
            Some(CARD_CLUB_ACE) => input[StateIndex::HandClubAce as usize] = 1.0,
            None => {}
            _ => panic!("invalid card"),
        }
    }
}

pub fn set_discarded(input: &mut NeuralNetworkInput, action: &ActionIndex) {
    match *action {
        // Spade
        ActionIndex::DiscardSpadeNine => input[StateIndex::HandSpadeNine as usize] = 0.0,
        ActionIndex::DiscardSpadeTen => input[StateIndex::HandSpadeTen as usize] = 0.0,
        ActionIndex::DiscardSpadeJack => input[StateIndex::HandSpadeJack as usize] = 0.0,
        ActionIndex::DiscardSpadeQueen => input[StateIndex::HandSpadeQueen as usize] = 0.0,
        ActionIndex::DiscardSpadeKing => input[StateIndex::HandSpadeKing as usize] = 0.0,
        ActionIndex::DiscardSpadeAce => input[StateIndex::HandSpadeAce as usize] = 0.0,
        // Heart
        ActionIndex::DiscardHeartNine => input[StateIndex::HandHeartNine as usize] = 0.0,
        ActionIndex::DiscardHeartTen => input[StateIndex::HandHeartTen as usize] = 0.0,
        ActionIndex::DiscardHeartJack => input[StateIndex::HandHeartJack as usize] = 0.0,
        ActionIndex::DiscardHeartQueen => input[StateIndex::HandHeartQueen as usize] = 0.0,
        ActionIndex::DiscardHeartKing => input[StateIndex::HandHeartKing as usize] = 0.0,
        ActionIndex::DiscardHeartAce => input[StateIndex::HandHeartAce as usize] = 0.0,
        // Diamond
        ActionIndex::DiscardDiamondNine => input[StateIndex::HandDiamondNine as usize] = 0.0,
        ActionIndex::DiscardDiamondTen => input[StateIndex::HandDiamondTen as usize] = 0.0,
        ActionIndex::DiscardDiamondJack => input[StateIndex::HandDiamondJack as usize] = 0.0,
        ActionIndex::DiscardDiamondQueen => input[StateIndex::HandDiamondQueen as usize] = 0.0,
        ActionIndex::DiscardDiamondKing => input[StateIndex::HandDiamondKing as usize] = 0.0,
        ActionIndex::DiscardDiamondAce => input[StateIndex::HandDiamondAce as usize] = 0.0,
        // Club
        ActionIndex::DiscardClubNine => input[StateIndex::HandClubNine as usize] = 0.0,
        ActionIndex::DiscardClubTen => input[StateIndex::HandClubTen as usize] = 0.0,
        ActionIndex::DiscardClubJack => input[StateIndex::HandClubJack as usize] = 0.0,
        ActionIndex::DiscardClubQueen => input[StateIndex::HandClubQueen as usize] = 0.0,
        ActionIndex::DiscardClubKing => input[StateIndex::HandClubKing as usize] = 0.0,
        ActionIndex::DiscardClubAce => input[StateIndex::HandClubAce as usize] = 0.0,
        _ => panic!("invalid card"),
    }
}

pub fn set_dealer(input: &mut NeuralNetworkInput, relative_position: &RelativePosition) {
    match *relative_position {
        RelativePosition::Myself => input[StateIndex::DealerMyself as usize] = 1.0,
        RelativePosition::Left => input[StateIndex::DealerLeft as usize] = 1.0,
        RelativePosition::Ally => input[StateIndex::DealerAlly as usize] = 1.0,
        RelativePosition::Right => input[StateIndex::DealerRight as usize] = 1.0,
        _ => panic!("invalid relative position"),
    }
}

pub fn set_score(input: &mut NeuralNetworkInput, ally_score: &u8, enemy_score: &u8) {
    match *ally_score {
        0 => input[StateIndex::AllyScore0 as usize] = 1.0,
        1 => input[StateIndex::AllyScore1 as usize] = 1.0,
        2 => input[StateIndex::AllyScore2 as usize] = 1.0,
        3 => input[StateIndex::AllyScore3 as usize] = 1.0,
        4 => input[StateIndex::AllyScore4 as usize] = 1.0,
        5 => input[StateIndex::AllyScore5 as usize] = 1.0,
        6 => input[StateIndex::AllyScore6 as usize] = 1.0,
        7 => input[StateIndex::AllyScore7 as usize] = 1.0,
        8 => input[StateIndex::AllyScore8 as usize] = 1.0,
        9 => input[StateIndex::AllyScore9 as usize] = 1.0,
        _ => panic!("invalid ally score"),
    }
    match *enemy_score {
        0 => input[StateIndex::EnemyScore0 as usize] = 1.0,
        1 => input[StateIndex::EnemyScore1 as usize] = 1.0,
        2 => input[StateIndex::EnemyScore2 as usize] = 1.0,
        3 => input[StateIndex::EnemyScore3 as usize] = 1.0,
        4 => input[StateIndex::EnemyScore4 as usize] = 1.0,
        5 => input[StateIndex::EnemyScore5 as usize] = 1.0,
        6 => input[StateIndex::EnemyScore6 as usize] = 1.0,
        7 => input[StateIndex::EnemyScore7 as usize] = 1.0,
        8 => input[StateIndex::EnemyScore8 as usize] = 1.0,
        9 => input[StateIndex::EnemyScore9 as usize] = 1.0,
        _ => panic!("invalid enemy score"),
    }
}
