use crate::{NeuralNetwork, NeuralNetworkInput};
use strum::EnumCount;

use super::{
    enums::{Rank, StateIndex, Suit, Team},
    neural_network_helpers::set_score,
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
    let mut north_south_score: u8 = 0;
    let mut east_west_score: u8 = 0;

    let north_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let east_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let south_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];
    let west_input: &mut NeuralNetworkInput = &mut [0.0; StateIndex::COUNT];

    while north_south_score < 10 && east_west_score < 10 {
        set_score(north_input, north_south_score, east_west_score);
        set_score(east_input, east_west_score, north_south_score);
        set_score(south_input, north_south_score, east_west_score);
        set_score(west_input, east_west_score, north_south_score);
        let (north_south_delta, east_west_delta) = run_round(
            north_player,
            east_player,
            south_player,
            west_player,
            north_input,
            east_input,
            south_input,
            west_input,
        );
        north_south_score += north_south_delta;
        east_west_score += east_west_delta;
    }
    Team::EastWest
}

fn run_round(
    north_player: &NeuralNetwork,
    east_player: &NeuralNetwork,
    south_player: &NeuralNetwork,
    west_player: &NeuralNetwork,
    north_input: &mut NeuralNetworkInput,
    east_input: &mut NeuralNetworkInput,
    south_input: &mut NeuralNetworkInput,
    west_input: &mut NeuralNetworkInput,
) -> (u8, u8) {
    (0, 0)
}
