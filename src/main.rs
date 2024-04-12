use std::mem;
use std::thread;

mod euchre;

mod organism;

use crate::euchre::game::play_euchre;
use crate::organism::neural_network::NeuralNetwork;

fn main() {
    const NUM_NETWORKS: usize = 2;

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
            let winning_team = play_euchre(&nn2, &nn1, &nn2, &nn1);
            println!("{:?}", winning_team);
            let winning_team = play_euchre(&nn1, &nn2, &nn1, &nn2);
            println!("{:?}", winning_team);

            nn1.save_to_file("model.bin")?;
            nn2.load_from_file("model.bin")?;
            assert_eq!(nn1, nn2);
            println!("Saving/loading successful");
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
