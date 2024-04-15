use std::mem;
use std::thread;

mod euchre;

mod organism;

use organism::neural_network::NeuralNetwork;

use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
use crate::organism::evolution::POPULATION_SIZE;

fn main() {
    // TODO: figure this out, i'm at a total loss
    let stack_size: usize = mem::size_of::<Organism>() * (POPULATION_SIZE + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| -> std::io::Result<()> {
            let organism = evolve(100).unwrap();
            let mut nn1 = NeuralNetwork::new();
            organism.brain.unwrap().save_to_file("out/champion.bin")?;
            nn1.load_from_file("out/champion.bin")?;
            assert_eq!(organism.brain.unwrap(), nn1);
            println!("Saving/loading success");
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
