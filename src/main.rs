use std::mem;
use std::thread;

mod euchre;

mod organism;

use organism::neural_network::NeuralNetwork;

use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
use crate::organism::evolution::POPULATION_SIZE;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Euchre NEAT",
    about = "Neuroevolution of Augmenting Topologies for Euchre Tool"
)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Evolve a population of agents")]
    Evolve {
        #[structopt(short, long)]
        generations: Option<usize>,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.command {
        Command::Evolve { generations } => match generations {
            Some(x) => evolve_cli(x),
            None => println!("You need to specify a number of generations"),
        },
    };
}

fn evolve_cli(generations: usize) {
    // TODO: figure this out, i'm at a total loss
    let stack_size: usize = mem::size_of::<Organism>() * (POPULATION_SIZE + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let organism = evolve(generations).unwrap();
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
