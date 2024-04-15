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
        thread_count: Option<usize>,
        #[structopt(short, long)]
        out_dir: Option<String>,
    },
    #[structopt(about = "Run a game between agents")]
    Compete {
        #[structopt(short, long)]
        north_player: String,
        #[structopt(short, long)]
        east_player: String,
        #[structopt(short, long)]
        south_player: String,
        #[structopt(short, long)]
        west_player: String,
        #[structopt(short, long)]
        games: Option<usize>,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.command {
        Command::Evolve {
            population_size,
            generations,
            thread_count,
            out_dir,
        } => {
            evolve_cli(
                population_size.unwrap_or_else(|| 4),
                generations.unwrap_or_else(|| 10),
                thread_count.unwrap_or_else(|| 1),
                out_dir.unwrap_or_else(|| "out".to_string()),
            );
        }
        Command::Compete {
            north_player,
            east_player,
            south_player,
            west_player,
            games,
        } => {
            compete_cli(
                north_player,
                east_player,
                south_player,
                west_player,
                games.unwrap_or_else(|| 3),
            );
        }
    };
}
