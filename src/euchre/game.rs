use super::{constants::*, enums::*, game_helpers::*, neural_network_helpers::*, types::*};
use crate::{NeuralNetwork, NeuralNetworkInput};
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
    Team::EastWest
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
    let (hands, upcard): ([[Option<Card>; 6]; 4], Card) = deal();
    let mut dealer_hand = hands[0];
    let mut position_1_hand = hands[1];
    let mut position_2_hand = hands[2];
    let mut position_3_hand = hands[3];
    set_hand(dealer_input, &dealer_hand);
    set_hand(position_1_input, &position_1_hand);
    set_hand(position_2_input, &position_2_hand);
    set_hand(position_3_input, &position_3_hand);
    set_upcard(dealer_input, &upcard);
    set_upcard(position_1_input, &upcard);
    set_upcard(position_2_input, &upcard);
    set_upcard(position_3_input, &upcard);
    let (made, alone) = run_bid_upcard(
        dealer_player,
        position_1_player,
        position_2_player,
        position_3_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
    );
    (0, 0)
}

// TODO: unstub
fn run_bid_upcard(
    dealer_player: &NeuralNetwork,
    position_1_player: &NeuralNetwork,
    position_2_player: &NeuralNetwork,
    position_3_player: &NeuralNetwork,
    dealer_input: &mut NeuralNetworkInput,
    position_1_input: &mut NeuralNetworkInput,
    position_2_input: &mut NeuralNetworkInput,
    position_3_input: &mut NeuralNetworkInput,
) -> (bool, bool) {
    match get_bid_upcard_action(
        position_1_player,
        position_2_player,
        position_3_player,
        dealer_player,
        position_1_input,
        position_2_input,
        position_3_input,
        dealer_input,
    ) {
        Some((true, true)) => return (true, true),
        Some((true, false)) => return (true, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        position_2_player,
        position_3_player,
        dealer_player,
        position_1_player,
        position_2_input,
        position_3_input,
        dealer_input,
        position_1_input,
    ) {
        Some((true, true)) => return (true, true),
        Some((true, false)) => return (true, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        position_3_player,
        dealer_player,
        position_1_player,
        position_2_player,
        position_3_input,
        dealer_input,
        position_1_input,
        position_2_input,
    ) {
        Some((true, true)) => return (true, true),
        Some((true, false)) => return (true, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    match get_bid_upcard_action(
        dealer_player,
        position_1_player,
        position_2_player,
        position_3_player,
        dealer_input,
        position_1_input,
        position_2_input,
        position_3_input,
    ) {
        Some((true, true)) => return (true, true),
        Some((true, false)) => return (true, false),
        None => {}
        _ => panic!("invalid bid upcard action result"),
    }
    (false, false)
}

fn get_bid_upcard_action(
    myself: &NeuralNetwork,
    left: &NeuralNetwork,
    ally: &NeuralNetwork,
    right: &NeuralNetwork,
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

// TODO: unstub
fn run_discard() {}

// TODO: unstub
fn run_bid_suit() {}

// TODO: unstub
fn run_trick() {}
