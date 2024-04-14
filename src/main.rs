use std::mem;
use std::thread;

mod euchre;

mod organism;

use organism::neural_network::NeuralNetwork;

use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
use crate::organism::evolution::POPULATION_SIZE;

fn main() {
    // TODO: figure this out, idk why 21 works, had to add 6 for saving/loading
    let stack_size: usize = mem::size_of::<Organism>() * (POPULATION_SIZE + 21 + 6);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| -> std::io::Result<()> {
            let organism = evolve(1);
            let mut nn1 = NeuralNetwork::new();
            organism.brain.unwrap().save_to_file("model.bin")?;
            nn1.load_from_file("model.bin")?;
            assert_eq!(organism.brain.unwrap(), nn1);
            println!("Saving/loading success");
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
