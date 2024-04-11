use rand::prelude::*;
use std::mem;
use std::thread;

mod euchre;

mod neural_network;
use neural_network::*;
use strum::EnumCount;

use crate::euchre::enums::ActionIndex;
use crate::euchre::enums::StateIndex;

fn main() {
    // number of max simultaneously extant networks times 2
    const NUM_NETWORKS: usize = 4;
    let stack_size: usize = mem::size_of::<NeuralNetwork>() * NUM_NETWORKS * 2;

    // Spawn a thread with custom stack size
    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            let mut input: [f64; StateIndex::COUNT] = [0.0; StateIndex::COUNT];
            let mut rng = rand::thread_rng();

            for i in 0..StateIndex::COUNT {
                input[i] = rng.gen::<f64>();
            }

            let mut available_actions: AvailableActions = [false; ActionIndex::COUNT];
            for i in 0..6 {
                available_actions[i] = true;
            }

            let mut nn1 = NeuralNetwork::new();
            nn1.init();
            let mut nn2 = NeuralNetwork::new();
            nn2.init();

            let child = nn1.crossover(&nn2, 0.01, 0.1);

            let result1 = nn1.get_action(&input, &available_actions);
            let result2 = nn2.get_action(&input, &available_actions);
            let result3 = child.get_action(&input, &available_actions);
            println!("{:?}\n{:?}\n{:?}", result1, result2, result3);
        })
        .unwrap(); // Handle the Result to check for errors

    // Wait for the thread to finish
    handle.join().unwrap();
}
