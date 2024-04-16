use crate::euchre::enums::ActionIndex;

use super::neural_network::{AvailableActions, NeuralNetworkInput};

// TODO: unstub this
pub fn get_player_action(
    inputs: &NeuralNetworkInput,
    available_actions: &AvailableActions,
    tutor_action: &ActionIndex,
) -> ActionIndex {
    ActionIndex::DiscardClubAce
}
