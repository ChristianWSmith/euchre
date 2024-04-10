use rand::prelude::*;
use std::mem;
use std::thread;

mod euchre;

mod neural_network;
use neural_network::*;

fn main() {
    // number of max simultaneously extant networks times 2
    const NUM_NETWORKS: usize = 4;
    let stack_size: usize = mem::size_of::<NeuralNetwork>() * NUM_NETWORKS * 2;

    // Spawn a thread with custom stack size
    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            let mut inputs: [f64; INPUT_NODES] = [0.0; INPUT_NODES];
            let mut rng = rand::thread_rng();

            for i in 0..INPUT_NODES {
                inputs[i] = rng.gen::<f64>();
            }

            let mut nn1 = NeuralNetwork::new();
            nn1.init();
            let mut nn2 = NeuralNetwork::new();
            nn2.init();

            let child = nn1.crossover(&nn2, 0.01, 0.1);

            let result1 = nn1.query(&inputs);
            let result2 = nn2.query(&inputs);
            let result3 = child.query(&inputs);
            println!("{:?}\n{:?}\n{:?}", result1, result2, result3);
        })
        .unwrap(); // Handle the Result to check for errors

    // Wait for the thread to finish
    handle.join().unwrap();
}
