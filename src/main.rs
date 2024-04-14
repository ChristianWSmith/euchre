use std::mem;
use std::thread;

mod euchre;

mod organism;

use crate::euchre::enums::Team;
use crate::euchre::game::play_euchre;
use crate::organism::neural_network::NeuralNetwork;

fn main() {
    const NUM_NETWORKS: usize = 3;

    // enough memory for the number of networks plus 3:
    // - 1 chunk for each network
    // - 1 chunk for saving/loading a network
    // - 2 chunks for running a game
    let stack_size: usize = mem::size_of::<NeuralNetwork>() * (NUM_NETWORKS + 3);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| -> std::io::Result<()> {
            let mut nn1 = NeuralNetwork::new();
            let mut nn2 = NeuralNetwork::new();
            nn1.init();
            nn2.init();
            let (mut nn1_counter, mut nn2_counter) = (0, 0);
            for _ in 0..100 {
                let winning_team = play_euchre(&nn1, &nn2, &nn1, &nn2);
                match winning_team {
                    Team::NorthSouth => nn1_counter += 1,
                    Team::EastWest => nn2_counter += 1,
                }
                println!("nn1: {}, nn2: {}", nn1_counter, nn2_counter);
            }

            let nn3 = nn1.crossover(&nn2, 0.01, 0.1);
            nn3.save_to_file("model.bin")?;
            nn2.load_from_file("model.bin")?;
            assert_eq!(nn3, nn2);
            println!("Saving/loading successful");
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
