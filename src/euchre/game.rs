use super::{constants::*, enums::*, game_helpers::*, neural_network_helpers::*, types::*};
use crate::{AvailableActions, NeuralNetwork, NeuralNetworkInput};
use rand::Rng;
use strum::EnumCount;

pub fn play_euchre(
    north_player: &NeuralNetwork,
    east_player: &NeuralNetwork,
    south_player: &NeuralNetwork,
    west_player: &NeuralNetwork,
) -> Team {
    let mut rng = rand::thread_rng();

    let mut north_south_score: u8 = 0;
    let mut east_west_score: u8 = 0;

    let mut dealer: &Position = &POSITIONS[rng.gen_range(0..POSITIONS.len())];

    while north_south_score < 10 && east_west_score < 10 {
        let north_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
        let east_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
        let south_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
        let west_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];

        set_score(north_input, &north_south_score, &east_west_score);
        set_score(east_input, &east_west_score, &north_south_score);
        set_score(south_input, &north_south_score, &east_west_score);
        set_score(west_input, &east_west_score, &north_south_score);

        let (dealer_score_delta, other_score_delta) = match dealer {
            &Position::North => {
                set_dealer(north_input, &RelativePosition::Myself);
                set_dealer(east_input, &RelativePosition::Right);
                set_dealer(south_input, &RelativePosition::Ally);
                set_dealer(west_input, &RelativePosition::Left);
                run_round(
                    north_player,
                    east_player,
                    south_player,
                    west_player,
                    north_input,
                    east_input,
                    south_input,
                    west_input,
                )
            }
            &Position::East => {
                set_dealer(north_input, &RelativePosition::Left);
                set_dealer(east_input, &RelativePosition::Myself);
                set_dealer(south_input, &RelativePosition::Right);
                set_dealer(west_input, &RelativePosition::Ally);
                run_round(
                    east_player,
                    south_player,
                    west_player,
                    north_player,
                    east_input,
                    south_input,
                    west_input,
                    north_input,
                )
            }
            &Position::South => {
                set_dealer(north_input, &RelativePosition::Ally);
                set_dealer(east_input, &RelativePosition::Left);
                set_dealer(south_input, &RelativePosition::Myself);
                set_dealer(west_input, &RelativePosition::Right);
                run_round(
                    south_player,
                    west_player,
                    north_player,
                    east_player,
                    south_input,
                    west_input,
                    north_input,
                    east_input,
                )
            }
            &Position::West => {
                set_dealer(north_input, &RelativePosition::Right);
                set_dealer(east_input, &RelativePosition::Ally);
                set_dealer(south_input, &RelativePosition::Left);
                set_dealer(west_input, &RelativePosition::Myself);
                run_round(
                    west_player,
                    north_player,
                    east_player,
                    south_player,
                    west_input,
                    north_input,
                    east_input,
                    south_input,
                )
            }
            _ => panic!("invalid dealer"),
        };

        match dealer {
            &Position::North | &Position::South => {
                north_south_score += dealer_score_delta;
                east_west_score += other_score_delta;
            }
            &Position::East | &Position::West => {
                north_south_score += other_score_delta;
                east_west_score += dealer_score_delta;
            }
            _ => panic!("invalid dealer"),
        };

        dealer = left_player(dealer);
    }
    if north_south_score > east_west_score {
        Team::NorthSouth
    } else {
        Team::EastWest
    }
}

fn run_round(
    dealer_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    dealer_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
) -> (u8, u8) {
    let (mut dealer_hand, mut position_1_hand, mut position_2_hand, mut position_3_hand, upcard) =
        deal();
    set_hand(dealer_input, &dealer_hand);
    set_hand(position_1_input, &position_1_hand);
    set_hand(position_2_input, &position_2_hand);
    set_hand(position_3_input, &position_3_hand);
    set_upcard(dealer_input, &upcard);
    set_upcard(position_1_input, &upcard);
    set_upcard(position_2_input, &upcard);
    set_upcard(position_3_input, &upcard);
    let (
        mut making_team,
        mut skip_dealer,
        mut skip_position_1,
        mut skip_position_2,
        mut skip_position_3,
    ) = run_bid_upcard(
        dealer_player,
        position_1_player,
        position_2_player,
        position_3_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
    );
    let mut trump_suit: Option<Suit> = None;
    if making_team.is_some() {
        trump_suit = Some(upcard.suit);
        dealer_hand[5] = Some(upcard);
        run_discard(dealer_player, dealer_input, &mut dealer_hand);
    } else {
        (
            making_team,
            trump_suit,
            skip_dealer,
            skip_position_1,
            skip_position_2,
            skip_position_3,
        ) = run_bid_suit(
            dealer_player,
            position_1_player,
            position_2_player,
            position_3_player,
            dealer_input,
            position_1_input,
            position_2_input,
            position_3_input,
            &upcard.suit,
        );
    }
    if making_team.is_none() || trump_suit.is_none() {
        return (0, 0);
    }

    let (dealer_team_tricks, other_team_tricks): (u8, u8) = run_tricks(
        dealer_player,
        position_1_player,
        position_2_player,
        position_3_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
        &mut dealer_hand,
        &mut position_1_hand,
        &mut position_2_hand,
        &mut position_3_hand,
        &skip_dealer,
        &skip_position_1,
        &skip_position_2,
        &skip_position_3,
        &trump_suit.unwrap(),
    );

    match (
        making_team,
        dealer_team_tricks,
        other_team_tricks,
        skip_dealer,
        skip_position_1,
        skip_position_2,
        skip_position_3,
    ) {
        (Some(RelativeTeam::Dealer), 5, _, true, _, _, _)
        | (Some(RelativeTeam::Dealer), 5, _, _, _, true, _) => return (4, 0),
        (Some(RelativeTeam::Other), _, 5, _, true, _, _)
        | (Some(RelativeTeam::Other), _, 5, _, _, _, true) => return (0, 4),

        (Some(RelativeTeam::Dealer), 5, _, false, _, false, _)
        | (Some(RelativeTeam::Other), _, 2, _, _, _, _)
        | (Some(RelativeTeam::Other), _, 1, _, _, _, _)
        | (Some(RelativeTeam::Other), _, 0, _, _, _, _) => return (2, 0),

        (Some(RelativeTeam::Other), _, 5, _, false, _, false)
        | (Some(RelativeTeam::Dealer), 2, _, _, _, _, _)
        | (Some(RelativeTeam::Dealer), 1, _, _, _, _, _)
        | (Some(RelativeTeam::Dealer), 0, _, _, _, _, _) => return (0, 2),

        (Some(RelativeTeam::Dealer), 4, _, _, _, _, _)
        | (Some(RelativeTeam::Dealer), 3, _, _, _, _, _) => return (1, 0),
        (Some(RelativeTeam::Other), _, 4, _, _, _, _)
        | (Some(RelativeTeam::Other), _, 3, _, _, _, _) => return (0, 1),
        _ => panic!("failed to score round"),
    }
}

fn run_bid_upcard(
    dealer_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    dealer_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
) -> (Option<RelativeTeam>, bool, bool, bool, bool) {
    match get_bid_upcard_action(
        position_1_player,
        position_1_input,
        position_2_input,
        position_3_input,
        dealer_input,
    ) {
        Some((true, true)) => return (Some(RelativeTeam::Other), false, false, false, true),
        Some((true, false)) => return (Some(RelativeTeam::Other), false, false, false, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        position_2_player,
        position_2_input,
        position_3_input,
        dealer_input,
        position_1_input,
    ) {
        Some((true, true)) => return (Some(RelativeTeam::Dealer), true, false, false, false),
        Some((true, false)) => return (Some(RelativeTeam::Dealer), false, false, false, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        position_3_player,
        position_3_input,
        dealer_input,
        position_1_input,
        position_2_input,
    ) {
        Some((true, true)) => return (Some(RelativeTeam::Other), false, true, false, false),
        Some((true, false)) => return (Some(RelativeTeam::Other), false, false, false, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        dealer_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
    ) {
        Some((true, true)) => return (Some(RelativeTeam::Dealer), false, false, true, false),
        Some((true, false)) => return (Some(RelativeTeam::Dealer), false, false, false, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    (None, false, false, false, false)
}

fn get_bid_upcard_action(
    myself: &NeuralNetwork,
    myself_input: &mut NeuralNetworkInput,
    left_input: &mut NeuralNetworkInput,
    ally_input: &mut NeuralNetworkInput,
    right_input: &mut NeuralNetworkInput,
) -> Option<(bool, bool)> {
    match myself.get_action(&myself_input, &BID_UPCARD_AVAILABLE_ACTIONS) {
        ActionIndex::MakeUpcard => {
            set_bid_upcard(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeUpcard,
            );
            set_bid_upcard(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeUpcard,
            );
            set_bid_upcard(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeUpcard,
            );
            set_bid_upcard(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeUpcard,
            );
            return Some((true, false));
        }
        ActionIndex::MakeUpcardAlone => {
            set_bid_upcard(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeUpcardAlone,
            );
            set_bid_upcard(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeUpcardAlone,
            );
            set_bid_upcard(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeUpcardAlone,
            );
            set_bid_upcard(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeUpcardAlone,
            );
            return Some((true, true));
        }
        ActionIndex::PassUpcard => {
            set_bid_upcard(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::PassUpcard,
            );
            set_bid_upcard(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::PassUpcard,
            );
            set_bid_upcard(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::PassUpcard,
            );
            set_bid_upcard(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::PassUpcard,
            );
            return None;
        }
        _ => panic!("invalid bid upcard action"),
    }
}

fn run_discard(
    player: &NeuralNetwork,
    input: &mut NeuralNetworkInput,
    hand: &mut [Option<Card>; 6],
) {
    let action = player.get_action(input, &get_discard_available_actions(hand));
    set_discarded(input, &action);
    discard_from_hand(hand, &action);
}

fn run_bid_suit(
    dealer_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    dealer_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
    upcard_suit: &Suit,
) -> (Option<RelativeTeam>, Option<Suit>, bool, bool, bool, bool) {
    let available_actions = get_bid_suit_available_actions(upcard_suit);
    match get_bid_suit_action(
        position_1_player,
        position_1_input,
        position_2_input,
        position_3_input,
        dealer_input,
        &available_actions,
    ) {
        Some((true, Suit::Spade, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Spade),
                false,
                false,
                false,
                true,
            )
        }
        Some((true, Suit::Heart, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Heart),
                false,
                false,
                false,
                true,
            )
        }
        Some((true, Suit::Diamond, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Diamond),
                false,
                false,
                false,
                true,
            )
        }
        Some((true, Suit::Club, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Club),
                false,
                false,
                false,
                true,
            )
        }
        Some((true, Suit::Spade, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Spade),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Diamond),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Heart),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Club, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Club),
                false,
                false,
                false,
                false,
            )
        }
        None => {}
        _ => panic!("invalid bid suit action result"),
    }
    match get_bid_suit_action(
        position_2_player,
        position_2_input,
        position_3_input,
        dealer_input,
        position_1_input,
        &available_actions,
    ) {
        Some((true, Suit::Spade, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Spade),
                true,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Heart),
                true,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Diamond),
                true,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Club, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Club),
                true,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Spade, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Spade),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Diamond),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Heart),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Club, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Club),
                false,
                false,
                false,
                false,
            )
        }
        None => {}
        _ => panic!("invalid bid suit action result"),
    }
    match get_bid_suit_action(
        position_3_player,
        position_3_input,
        dealer_input,
        position_1_input,
        position_2_input,
        &available_actions,
    ) {
        Some((true, Suit::Spade, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Spade),
                false,
                true,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Heart),
                false,
                true,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Diamond),
                false,
                true,
                false,
                false,
            )
        }
        Some((true, Suit::Club, true)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Club),
                false,
                true,
                false,
                false,
            )
        }
        Some((true, Suit::Spade, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Spade),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Diamond),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Heart),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Club, false)) => {
            return (
                Some(RelativeTeam::Other),
                Some(Suit::Club),
                false,
                false,
                false,
                false,
            )
        }
        None => {}
        _ => panic!("invalid bid suit action result"),
    }
    match get_bid_suit_action(
        dealer_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
        &available_actions,
    ) {
        Some((true, Suit::Spade, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Spade),
                false,
                false,
                true,
                false,
            )
        }
        Some((true, Suit::Heart, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Heart),
                false,
                false,
                true,
                false,
            )
        }
        Some((true, Suit::Diamond, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Diamond),
                false,
                false,
                true,
                false,
            )
        }
        Some((true, Suit::Club, true)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Club),
                false,
                false,
                true,
                false,
            )
        }
        Some((true, Suit::Spade, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Spade),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Diamond, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Diamond),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Heart, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Heart),
                false,
                false,
                false,
                false,
            )
        }
        Some((true, Suit::Club, false)) => {
            return (
                Some(RelativeTeam::Dealer),
                Some(Suit::Club),
                false,
                false,
                false,
                false,
            )
        }
        None => {}
        _ => panic!("invalid bid suit action result"),
    }
    (None, None, false, false, false, false)
}

fn get_bid_suit_action(
    myself: &NeuralNetwork,
    myself_input: &mut NeuralNetworkInput,
    left_input: &mut NeuralNetworkInput,
    ally_input: &mut NeuralNetworkInput,
    right_input: &mut NeuralNetworkInput,
    available_actions: &AvailableActions,
) -> Option<(bool, Suit, bool)> {
    match myself.get_action(&myself_input, available_actions) {
        ActionIndex::MakeSuitSpade => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitSpade,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitSpade,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitSpade,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitSpade,
            );
            return Some((true, Suit::Spade, false));
        }
        ActionIndex::MakeSuitSpadeAlone => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitSpadeAlone,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitSpadeAlone,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitSpadeAlone,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitSpadeAlone,
            );
            return Some((true, Suit::Spade, true));
        }
        ActionIndex::MakeSuitHeart => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitHeart,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitHeart,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitHeart,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitHeart,
            );
            return Some((true, Suit::Heart, false));
        }
        ActionIndex::MakeSuitHeartAlone => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitHeartAlone,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitHeartAlone,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitHeartAlone,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitHeartAlone,
            );
            return Some((true, Suit::Heart, true));
        }
        ActionIndex::MakeSuitDiamond => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitDiamond,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitDiamond,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitDiamond,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitDiamond,
            );
            return Some((true, Suit::Diamond, false));
        }
        ActionIndex::MakeSuitDiamondAlone => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitDiamondAlone,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitDiamondAlone,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitDiamondAlone,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitDiamondAlone,
            );
            return Some((true, Suit::Diamond, true));
        }
        ActionIndex::MakeSuitClub => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitClub,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitClub,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitClub,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitClub,
            );
            return Some((true, Suit::Club, false));
        }
        ActionIndex::MakeSuitClubAlone => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::MakeSuitClubAlone,
            );
            set_bid_suit(
                left_input,
                &RelativePosition::Right,
                &ActionIndex::MakeSuitClubAlone,
            );
            set_bid_suit(
                ally_input,
                &RelativePosition::Ally,
                &ActionIndex::MakeSuitClubAlone,
            );
            set_bid_suit(
                right_input,
                &RelativePosition::Left,
                &ActionIndex::MakeSuitClubAlone,
            );
            return Some((true, Suit::Club, true));
        }
        ActionIndex::PassSuit => {
            set_bid_suit(
                myself_input,
                &RelativePosition::Myself,
                &ActionIndex::PassSuit,
            );
            set_bid_suit(left_input, &RelativePosition::Right, &ActionIndex::PassSuit);
            set_bid_suit(ally_input, &RelativePosition::Ally, &ActionIndex::PassSuit);
            set_bid_suit(right_input, &RelativePosition::Left, &ActionIndex::PassSuit);
            return None;
        }
        _ => panic!("invalid bid upcard action"),
    }
}

fn run_tricks(
    dealer_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    dealer_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
    dealer_hand: &mut [Option<Card>; 6],
    position_1_hand: &mut [Option<Card>; 6],
    position_2_hand: &mut [Option<Card>; 6],
    position_3_hand: &mut [Option<Card>; 6],
    skip_dealer: &bool,
    skip_position_1: &bool,
    skip_position_2: &bool,
    skip_position_3: &bool,
    trump_suit: &Suit,
) -> (u8, u8) {
    let (mut dealer_team_trick_count, mut other_team_trick_count): (u8, u8) = (0, 0);
    let mut lead = DealerRelativePosition::Left;
    for trick_index in 0..5 {
        set_trick_count(
            dealer_input,
            dealer_team_trick_count,
            other_team_trick_count,
            1.0,
        );
        set_trick_count(
            position_1_input,
            other_team_trick_count,
            dealer_team_trick_count,
            1.0,
        );
        set_trick_count(
            position_2_input,
            dealer_team_trick_count,
            other_team_trick_count,
            1.0,
        );
        set_trick_count(
            position_3_input,
            other_team_trick_count,
            dealer_team_trick_count,
            1.0,
        );
        lead = match lead {
            DealerRelativePosition::Dealer => run_trick(
                &DealerRelativePosition::Dealer,
                dealer_player,
                position_1_player,
                position_2_player,
                position_3_player,
                dealer_input,
                position_1_input,
                position_2_input,
                position_3_input,
                dealer_hand,
                position_1_hand,
                position_2_hand,
                position_3_hand,
                &skip_dealer,
                &skip_position_1,
                &skip_position_2,
                &skip_position_3,
                trump_suit,
                &trick_index,
            ),
            DealerRelativePosition::Left => run_trick(
                &DealerRelativePosition::Left,
                position_1_player,
                position_2_player,
                position_3_player,
                dealer_player,
                position_1_input,
                position_2_input,
                position_3_input,
                dealer_input,
                position_1_hand,
                position_2_hand,
                position_3_hand,
                dealer_hand,
                &skip_position_1,
                &skip_position_2,
                &skip_position_3,
                &skip_dealer,
                trump_suit,
                &trick_index,
            ),
            DealerRelativePosition::Ally => run_trick(
                &DealerRelativePosition::Ally,
                position_2_player,
                position_3_player,
                dealer_player,
                position_1_player,
                position_2_input,
                position_3_input,
                dealer_input,
                position_1_input,
                position_2_hand,
                position_3_hand,
                dealer_hand,
                position_1_hand,
                &skip_position_2,
                &skip_position_3,
                &skip_dealer,
                &skip_position_1,
                trump_suit,
                &trick_index,
            ),
            DealerRelativePosition::Right => run_trick(
                &DealerRelativePosition::Right,
                position_3_player,
                dealer_player,
                position_1_player,
                position_2_player,
                position_3_input,
                dealer_input,
                position_1_input,
                position_2_input,
                position_3_hand,
                dealer_hand,
                position_1_hand,
                position_2_hand,
                &skip_position_3,
                &skip_dealer,
                &skip_position_1,
                &skip_position_2,
                trump_suit,
                &trick_index,
            ),
            _ => panic!("invalid trick winner"),
        };

        set_trick_count(
            dealer_input,
            dealer_team_trick_count,
            other_team_trick_count,
            0.0,
        );
        set_trick_count(
            position_1_input,
            other_team_trick_count,
            dealer_team_trick_count,
            0.0,
        );
        set_trick_count(
            position_2_input,
            dealer_team_trick_count,
            other_team_trick_count,
            0.0,
        );
        set_trick_count(
            position_3_input,
            other_team_trick_count,
            dealer_team_trick_count,
            0.0,
        );
        match lead {
            DealerRelativePosition::Dealer | DealerRelativePosition::Ally => {
                dealer_team_trick_count += 1;
            }
            DealerRelativePosition::Left | DealerRelativePosition::Right => {
                other_team_trick_count += 1;
            }
            _ => panic!("invalid trick winner"),
        }
    }
    (dealer_team_trick_count, other_team_trick_count)
}

fn run_trick(
    lead_position_relative_to_dealer: &DealerRelativePosition,
    lead_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    lead_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
    lead_hand: &mut [Option<Card>; 6],
    position_1_hand: &mut [Option<Card>; 6],
    position_2_hand: &mut [Option<Card>; 6],
    position_3_hand: &mut [Option<Card>; 6],
    skip_lead: &bool,
    skip_position_1: &bool,
    skip_position_2: &bool,
    skip_position_3: &bool,
    trump_suit: &Suit,
    trick_index: &u8,
) -> DealerRelativePosition {
    set_trick_lead(lead_input, &RelativePosition::Myself, trick_index);
    set_trick_lead(position_1_input, &RelativePosition::Right, trick_index);
    set_trick_lead(position_2_input, &RelativePosition::Ally, trick_index);
    set_trick_lead(position_3_input, &RelativePosition::Left, trick_index);

    let mut winning_player_lead_relative_position: Option<LeadRelativePosition> = None;
    let mut lead_suit: Option<Suit> = None;
    let mut winning_card: Option<Card> = None;

    get_trick_action(
        lead_player,
        lead_input,
        position_1_input,
        position_2_input,
        position_3_input,
        lead_hand,
        &skip_lead,
        &mut winning_player_lead_relative_position,
        &LeadRelativePosition::Lead,
        &mut winning_card,
        &mut lead_suit,
        trump_suit,
        trick_index,
        &0,
    );
    get_trick_action(
        position_1_player,
        position_1_input,
        position_2_input,
        position_3_input,
        lead_input,
        position_1_hand,
        &skip_position_1,
        &mut winning_player_lead_relative_position,
        &LeadRelativePosition::Left,
        &mut winning_card,
        &mut lead_suit,
        trump_suit,
        trick_index,
        &1,
    );
    get_trick_action(
        position_2_player,
        position_2_input,
        position_3_input,
        lead_input,
        position_1_input,
        position_2_hand,
        &skip_position_2,
        &mut winning_player_lead_relative_position,
        &LeadRelativePosition::Ally,
        &mut winning_card,
        &mut lead_suit,
        trump_suit,
        trick_index,
        &2,
    );
    get_trick_action(
        position_3_player,
        position_3_input,
        lead_input,
        position_1_input,
        position_2_input,
        position_3_hand,
        &skip_position_3,
        &mut winning_player_lead_relative_position,
        &LeadRelativePosition::Right,
        &mut winning_card,
        &mut lead_suit,
        trump_suit,
        trick_index,
        &3,
    );

    match (
        winning_player_lead_relative_position,
        lead_position_relative_to_dealer,
    ) {
        (Some(LeadRelativePosition::Lead), DealerRelativePosition::Dealer)
        | (Some(LeadRelativePosition::Left), DealerRelativePosition::Right)
        | (Some(LeadRelativePosition::Ally), DealerRelativePosition::Ally)
        | (Some(LeadRelativePosition::Right), DealerRelativePosition::Left) => {
            return DealerRelativePosition::Dealer
        }
        (Some(LeadRelativePosition::Lead), DealerRelativePosition::Left)
        | (Some(LeadRelativePosition::Left), DealerRelativePosition::Dealer)
        | (Some(LeadRelativePosition::Ally), DealerRelativePosition::Right)
        | (Some(LeadRelativePosition::Right), DealerRelativePosition::Ally) => {
            return DealerRelativePosition::Left
        }
        (Some(LeadRelativePosition::Lead), DealerRelativePosition::Ally)
        | (Some(LeadRelativePosition::Left), DealerRelativePosition::Left)
        | (Some(LeadRelativePosition::Ally), DealerRelativePosition::Dealer)
        | (Some(LeadRelativePosition::Right), DealerRelativePosition::Right) => {
            return DealerRelativePosition::Ally
        }
        (Some(LeadRelativePosition::Lead), DealerRelativePosition::Right)
        | (Some(LeadRelativePosition::Left), DealerRelativePosition::Ally)
        | (Some(LeadRelativePosition::Ally), DealerRelativePosition::Left)
        | (Some(LeadRelativePosition::Right), DealerRelativePosition::Dealer) => {
            return DealerRelativePosition::Right
        }
        _ => panic!("impossible lead/dealer relative position pairing"),
    }
}

fn get_trick_action(
    player: &NeuralNetwork,
    input: &mut NeuralNetworkInput,
    other_input_1: &mut NeuralNetworkInput,
    other_input_2: &mut NeuralNetworkInput,
    other_input_3: &mut NeuralNetworkInput,
    hand: &mut [Option<Card>; 6],
    skip: &bool,
    winning_player_lead_relative_position: &mut Option<LeadRelativePosition>,
    lead_relative_position: &LeadRelativePosition,
    winning_card: &mut Option<Card>,
    lead_suit: &mut Option<Suit>,
    trump_suit: &Suit,
    trick_index: &u8,
    trick_card_index: &u8,
) {
    if *skip {
        return;
    }
    let available_actions = get_play_available_actions(hand, &lead_suit);
    let action = player.get_action(input, &available_actions);
    let card = play_from_hand(hand, &action);
    if card_wins(&card, &winning_card, &lead_suit, &trump_suit) {
        *winning_player_lead_relative_position = Some(*lead_relative_position);
        *winning_card = Some(card);
    }
    if lead_suit.is_none() {
        *lead_suit = Some(card.suit);
        set_trick_lead_suit(input, &lead_suit, trick_index);
        set_trick_lead_suit(other_input_1, &lead_suit, trick_index);
        set_trick_lead_suit(other_input_2, &lead_suit, trick_index);
        set_trick_lead_suit(other_input_3, &lead_suit, trick_index);
    }
    set_trick_card_played(input, trick_index, trick_card_index);
    set_trick_card_played(other_input_1, trick_index, trick_card_index);
    set_trick_card_played(other_input_2, trick_index, trick_card_index);
    set_trick_card_played(other_input_3, trick_index, trick_card_index);
}
