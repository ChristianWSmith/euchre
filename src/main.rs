use std::mem;
use std::thread;

mod euchre;

mod organism;

use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
use crate::organism::evolution::POPULATION_SIZE;

fn main() {
    // TODO: figure this out
    let stack_size: usize = mem::size_of::<Organism>() * (POPULATION_SIZE + 21);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| -> std::io::Result<()> {
            evolve(1);
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
