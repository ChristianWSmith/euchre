use std::mem;
use std::thread;

mod euchre;

mod organism;

use crate::euchre::enums::Team;
use crate::euchre::game::play_euchre;
use crate::organism::evolution;
use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
use crate::organism::neural_network::NeuralNetwork;

fn main() {
    const NUM_ORGANISMS: usize = 4;

    // enough memory for the number of networks plus 3:
    // - 1 chunk for each network
    // - 1 chunk for saving/loading a network
    // - 2 chunks for running a game
    let stack_size: usize = mem::size_of::<Organism>() * (NUM_ORGANISMS + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| -> std::io::Result<()> {
            evolve(1);
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
