mod cli;
mod euchre;
mod organism;

use crate::cli::helpers::*;
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
