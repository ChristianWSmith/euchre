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
        #[structopt(short, long)]
        starting_population_dir: Option<String>,
        #[structopt(short, long)]
        no_gen_save: bool,
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
    #[structopt(about = "Query an agent for its stats")]
    Stats {
        #[structopt(short, long)]
        file: String,
    },
    #[structopt(about = "Run a game between agents")]
    Breed {
        #[structopt(short, long)]
        first_parent: String,
        #[structopt(short, long)]
        second_parent: String,
        #[structopt(short, long)]
        child: String,
    },
    #[structopt(about = "Play with a tutor")]
    Tutor {
        #[structopt(short, long)]
        tutor: String,
        #[structopt(short, long)]
        left: String,
        #[structopt(short, long)]
        right: String,
        #[structopt(short, long)]
        ally: String,
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
            starting_population_dir,
            no_gen_save,
        } => {
            evolve_cli(
                population_size.unwrap_or_else(|| 4),
                generations.unwrap_or_else(|| 10),
                thread_count.unwrap_or_else(|| 1),
                out_dir.unwrap_or_else(|| "out".to_string()),
                starting_population_dir,
                no_gen_save,
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
        Command::Stats { file } => {
            stats_cli(file);
        }
        Command::Breed {
            first_parent,
            second_parent,
            child,
        } => breed_cli(first_parent, second_parent, child),
        Command::Tutor {
            tutor,
            left,
            right,
            ally,
        } => {
            tutor_cli(tutor, left, right, ally);
        }
    };
}
