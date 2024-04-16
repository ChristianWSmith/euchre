use super::{constants::*, enums::*, types::*};
use crate::organism::neural_network::*;
use strum::EnumCount;

pub fn set_trump_suit(input: &mut NeuralNetworkInput, trump_suit: &Suit) {
    match *trump_suit {
        Suit::Spade => input[StateIndex::TrumpSuitSpade as usize] = 1.0,
        Suit::Heart => input[StateIndex::TrumpSuitHeart as usize] = 1.0,
        Suit::Diamond => input[StateIndex::TrumpSuitDiamond as usize] = 1.0,
        Suit::Club => input[StateIndex::TrumpSuitClub as usize] = 1.0,
    }
}

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
                }
            }
            _ => {}
        }
    }

    return available_actions;
}

pub fn get_card_play_action(card: &Card) -> ActionIndex {
    match *card {
        // Spade
        CARD_SPADE_NINE => return ActionIndex::PlaySpadeNine,
        CARD_SPADE_TEN => return ActionIndex::PlaySpadeTen,
        CARD_SPADE_JACK => return ActionIndex::PlaySpadeJack,
        CARD_SPADE_QUEEN => return ActionIndex::PlaySpadeQueen,
        CARD_SPADE_KING => return ActionIndex::PlaySpadeKing,
        CARD_SPADE_ACE => return ActionIndex::PlaySpadeAce,
        // Heart
        CARD_HEART_NINE => return ActionIndex::PlayHeartNine,
        CARD_HEART_TEN => return ActionIndex::PlayHeartTen,
        CARD_HEART_JACK => return ActionIndex::PlayHeartJack,
        CARD_HEART_QUEEN => return ActionIndex::PlayHeartQueen,
        CARD_HEART_KING => return ActionIndex::PlayHeartKing,
        CARD_HEART_ACE => return ActionIndex::PlayHeartAce,
        // Diamond
        CARD_DIAMOND_NINE => return ActionIndex::PlayDiamondNine,
        CARD_DIAMOND_TEN => return ActionIndex::PlayDiamondTen,
        CARD_DIAMOND_JACK => return ActionIndex::PlayDiamondJack,
        CARD_DIAMOND_QUEEN => return ActionIndex::PlayDiamondQueen,
        CARD_DIAMOND_KING => return ActionIndex::PlayDiamondKing,
        CARD_DIAMOND_ACE => return ActionIndex::PlayDiamondAce,
        // Club
        CARD_CLUB_NINE => return ActionIndex::PlayClubNine,
        CARD_CLUB_TEN => return ActionIndex::PlayClubTen,
        CARD_CLUB_JACK => return ActionIndex::PlayClubJack,
        CARD_CLUB_QUEEN => return ActionIndex::PlayClubQueen,
        CARD_CLUB_KING => return ActionIndex::PlayClubKing,
        CARD_CLUB_ACE => return ActionIndex::PlayClubAce,
    }
}

pub fn set_trick_lead(
    input: &mut NeuralNetworkInput,
    relative_position: &RelativePosition,
    trick_index: &TrickIndex,
) {
    match (relative_position, trick_index) {
        (RelativePosition::Myself, TrickIndex::First) => {
            input[StateIndex::Trick1MyselfLead as usize] = 1.0
        }
        (RelativePosition::Myself, TrickIndex::Second) => {
            input[StateIndex::Trick2MyselfLead as usize] = 1.0
        }
        (RelativePosition::Myself, TrickIndex::Third) => {
            input[StateIndex::Trick3MyselfLead as usize] = 1.0
        }
        (RelativePosition::Myself, TrickIndex::Fourth) => {
            input[StateIndex::Trick4MyselfLead as usize] = 1.0
        }
        (RelativePosition::Left, TrickIndex::First) => {
            input[StateIndex::Trick1LeftLead as usize] = 1.0
        }
        (RelativePosition::Left, TrickIndex::Second) => {
            input[StateIndex::Trick2LeftLead as usize] = 1.0
        }
        (RelativePosition::Left, TrickIndex::Third) => {
            input[StateIndex::Trick3LeftLead as usize] = 1.0
        }
        (RelativePosition::Left, TrickIndex::Fourth) => {
            input[StateIndex::Trick4LeftLead as usize] = 1.0
        }
        (RelativePosition::Ally, TrickIndex::First) => {
            input[StateIndex::Trick1AllyLead as usize] = 1.0
        }
        (RelativePosition::Ally, TrickIndex::Second) => {
            input[StateIndex::Trick2AllyLead as usize] = 1.0
        }
        (RelativePosition::Ally, TrickIndex::Third) => {
            input[StateIndex::Trick3AllyLead as usize] = 1.0
        }
        (RelativePosition::Ally, TrickIndex::Fourth) => {
            input[StateIndex::Trick4AllyLead as usize] = 1.0
        }
        (RelativePosition::Right, TrickIndex::First) => {
            input[StateIndex::Trick1RightLead as usize] = 1.0
        }
        (RelativePosition::Right, TrickIndex::Second) => {
            input[StateIndex::Trick2RightLead as usize] = 1.0
        }
        (RelativePosition::Right, TrickIndex::Third) => {
            input[StateIndex::Trick3RightLead as usize] = 1.0
        }
        (RelativePosition::Right, TrickIndex::Fourth) => {
            input[StateIndex::Trick4RightLead as usize] = 1.0
        }
        _ => panic!("invalid relative position and trick index combination"),
    }
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

pub fn set_trick_card_played(
    input: &mut NeuralNetworkInput,
    card: &Card,
    trick_index: &TrickIndex,
    trick_card_index: &TrickCardIndex,
) {
    match (card.suit, card.rank, trick_index, trick_card_index) {
        (Suit::Spade, Rank::Nine, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeNine as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeTen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeJack as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeQueen as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::King, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeKing as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3SpadeAce as usize] = 1.0;
        }
        (Suit::Spade, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4SpadeAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartNine as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartTen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartJack as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartQueen as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::King, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartKing as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3HeartAce as usize] = 1.0;
        }
        (Suit::Heart, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4HeartAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondNine as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondTen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondJack as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondQueen as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::King, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondKing as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3DiamondAce as usize] = 1.0;
        }
        (Suit::Diamond, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4DiamondAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Nine, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubNine as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Ten, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubTen as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Jack, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubJack as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::Queen, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubQueen as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::King, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubKing as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::First, TrickCardIndex::First) => {
            input[StateIndex::Trick1Card1ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::First, TrickCardIndex::Second) => {
            input[StateIndex::Trick1Card2ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::First, TrickCardIndex::Third) => {
            input[StateIndex::Trick1Card3ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::First, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick1Card4ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Second, TrickCardIndex::First) => {
            input[StateIndex::Trick2Card1ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Second, TrickCardIndex::Second) => {
            input[StateIndex::Trick2Card2ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Second, TrickCardIndex::Third) => {
            input[StateIndex::Trick2Card3ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Second, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick2Card4ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Third, TrickCardIndex::First) => {
            input[StateIndex::Trick3Card1ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Third, TrickCardIndex::Second) => {
            input[StateIndex::Trick3Card2ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Third, TrickCardIndex::Third) => {
            input[StateIndex::Trick3Card3ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Third, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick3Card4ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::First) => {
            input[StateIndex::Trick4Card1ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Second) => {
            input[StateIndex::Trick4Card2ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Third) => {
            input[StateIndex::Trick4Card3ClubAce as usize] = 1.0;
        }
        (Suit::Club, Rank::Ace, TrickIndex::Fourth, TrickCardIndex::Fourth) => {
            input[StateIndex::Trick4Card4ClubAce as usize] = 1.0;
        }

        _ => panic!("invalid card / trick index / trick card index combination"),
    }
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
