use crate::{NeuralNetwork, NeuralNetworkInput};
use rand::{seq::IteratorRandom, Rng};
use strum::EnumCount;

use super::{
    enums::{Position, Rank, RelativePosition, StateIndex, Suit, Team},
    game_helpers::left_player,
    neural_network_helpers::{set_dealer, set_score},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref DECK: [Card; Rank::COUNT * Suit::COUNT] = {
        let mut deck = [Card {
            suit: Suit::Spade,
            rank: Rank::Nine,
        }; Rank::COUNT * Suit::COUNT];
        let mut k = 0;
        for i in 0..Suit::COUNT {
            for j in 0..Rank::COUNT {
                deck[k] = Card {
                    suit: Suit::from_usize(i),
                    rank: Rank::from_usize(j),
                };
                k += 1
            }
        }
        deck
    };
    static ref POSITIONS: [Position; 4] = {
        let positions = [
            Position::North,
            Position::East,
            Position::South,
            Position::West,
        ];
        positions
    };
}

#[derive(Copy, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

pub fn play_euchre(
    north_player: &NeuralNetwork,
    east_player: &NeuralNetwork,
    south_player: &NeuralNetwork,
    west_player: &NeuralNetwork,
) -> Team {
    let mut rng = rand::thread_rng();

    let mut north_south_score: u8 = 0;
    let mut east_west_score: u8 = 0;

    let north_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let east_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let south_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let west_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];

    let mut dealer: &Position = &POSITIONS[rng.gen_range(0..POSITIONS.len())];

    while north_south_score < 10 && east_west_score < 10 {
        set_score(north_input, north_south_score, east_west_score);
        set_score(east_input, east_west_score, north_south_score);
        set_score(south_input, north_south_score, east_west_score);
        set_score(west_input, east_west_score, north_south_score);

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
    (0, 0)
}
