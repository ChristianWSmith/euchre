use std::mem;
use std::thread;

mod euchre;

mod organism;

use organism::neural_network::NeuralNetwork;

use crate::organism::evolution::evolve;
use crate::organism::evolution::Organism;
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
        population_size: Option<usize>,
        #[structopt(short, long)]
        generations: Option<usize>,
        #[structopt(short, long)]
        out_dir: Option<String>,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.command {
        Command::Evolve {
            population_size,
            generations,
            out_dir,
        } => {
            evolve_cli(
                population_size.unwrap_or_else(|| 4),
                generations.unwrap_or_else(|| 10),
                out_dir.unwrap_or_else(|| "out".to_string()),
            );
        }
    };
}

fn evolve_cli(population_size: usize, generations: usize, out_dir: String) {
    // TODO: figure this out, i'm at a total loss
    let stack_size: usize = mem::size_of::<Organism>() * (2048 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let organism = match population_size {
                2048 => evolve::<2048, 1024>(generations, out_dir.clone()).unwrap(),
                1024 => evolve::<1024, 512>(generations, out_dir.clone()).unwrap(),
                512 => evolve::<512, 256>(generations, out_dir.clone()).unwrap(),
                256 => evolve::<256, 128>(generations, out_dir.clone()).unwrap(),
                128 => evolve::<128, 64>(generations, out_dir.clone()).unwrap(),
                64 => evolve::<64, 32>(generations, out_dir.clone()).unwrap(),
                32 => evolve::<32, 16>(generations, out_dir.clone()).unwrap(),
                16 => evolve::<16, 8>(generations, out_dir.clone()).unwrap(),
                8 => evolve::<8, 4>(generations, out_dir.clone()).unwrap(),
                4 => evolve::<4, 2>(generations, out_dir.clone()).unwrap(),
                _ => panic!("Invalid population size.  Valid populations sizes: [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4]")
            };
            let mut nn1 = NeuralNetwork::new();
            nn1.load_from_file(format!("{}/champion.bin", out_dir.as_str()).as_str())?;
            assert_eq!(organism.brain.unwrap(), nn1);
            println!("Saving/loading success");
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}
