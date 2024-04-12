use rand::prelude::*;
use std::mem;
use std::thread;

mod euchre;

mod neural_network;
use neural_network::*;
use strum::EnumCount;

use crate::euchre::enums::ActionIndex;
use crate::euchre::enums::StateIndex;
use crate::euchre::game::play_euchre;

fn main() {
    // number of max simultaneously extant networks times 2
    const NUM_NETWORKS: usize = 4;
    let stack_size: usize = mem::size_of::<NeuralNetwork>() * NUM_NETWORKS * 2;

    // Spawn a thread with custom stack size
    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            let mut nn1 = NeuralNetwork::new();
            let mut nn2 = NeuralNetwork::new();
            let mut nn3 = NeuralNetwork::new();
            let mut nn4 = NeuralNetwork::new();
            nn1.init();
            nn2.init();
            nn3.init();
            nn4.init();
            let winning_team = play_euchre(&nn2, &nn1, &nn4, &nn3);
            println!("{:?}", winning_team);
            let winning_team = play_euchre(&nn1, &nn2, &nn3, &nn4);
            println!("{:?}", winning_team);
        })
        .unwrap(); // Handle the Result to check for errors

    // Wait for the thread to finish
    handle.join().unwrap();
}
