use super::enums::{RelativePosition, StateIndex};
use crate::NeuralNetworkInput;

pub fn set_dealer(input: &mut NeuralNetworkInput, relative_position: &RelativePosition) {
    for dealer_index in [
        StateIndex::DealerMyself,
        StateIndex::DealerLeft,
        StateIndex::DealerAlly,
        StateIndex::DealerRight,
    ] {
        input[dealer_index as usize] = 0.0
    }
    match relative_position {
        RelativePosition::Myself => input[StateIndex::DealerMyself as usize] = 1.0,
        RelativePosition::Left => input[StateIndex::DealerLeft as usize] = 1.0,
        RelativePosition::Ally => input[StateIndex::DealerAlly as usize] = 1.0,
        RelativePosition::Right => input[StateIndex::DealerRight as usize] = 1.0,
        _ => panic!("invalid relative position"),
    }
}

pub fn set_score(input: &mut NeuralNetworkInput, ally_score: u8, enemy_score: u8) {
    for score_index in [
        StateIndex::AllyScore0,
        StateIndex::AllyScore1,
        StateIndex::AllyScore2,
        StateIndex::AllyScore3,
        StateIndex::AllyScore4,
        StateIndex::AllyScore5,
        StateIndex::AllyScore6,
        StateIndex::AllyScore7,
        StateIndex::AllyScore8,
        StateIndex::AllyScore9,
        StateIndex::EnemyScore0,
        StateIndex::EnemyScore1,
        StateIndex::EnemyScore2,
        StateIndex::EnemyScore3,
        StateIndex::EnemyScore4,
        StateIndex::EnemyScore5,
        StateIndex::EnemyScore6,
        StateIndex::EnemyScore7,
        StateIndex::EnemyScore8,
        StateIndex::EnemyScore9,
    ] {
        input[score_index as usize] = 0.0
    }
    match ally_score {
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
    match enemy_score {
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
