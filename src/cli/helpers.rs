use std::{io::Write, mem, thread};

use crate::{
    euchre::{enums::Team, game::play_euchre},
    organism::{
        evolution::{evolve, Organism},
        neural_network::NeuralNetwork,
    },
};

const VALID_POPULATION_SIZES: [usize; 10] = [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4];

// TODO: better error handling here
pub fn evolve_cli(
    population_size: usize,
    generations: usize,
    thread_count: usize,
    out_dir: String,
    starting_population_dir: Option<String>,
    no_gen_save: bool,
) {
    if !VALID_POPULATION_SIZES.contains(&population_size) {
        println!(
            "Invalid population size, valid sizes are: {:?}",
            VALID_POPULATION_SIZES
        );
        std::process::exit(1);
    }
    println!(
        "Population Size: {}, Generations: {}",
        population_size, generations
    );

    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (2048 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            match population_size {
                2048 => evolve::<2048, 1024>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                1024 => evolve::<1024, 512>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                512 => evolve::<512, 256>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                256 => evolve::<256, 128>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                128 => evolve::<128, 64>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                64 => evolve::<64, 32>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                32 => evolve::<32, 16>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                16 => evolve::<16, 8>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                8 => evolve::<8, 4>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                4 => evolve::<4, 2>(generations, out_dir.clone(), thread_count, stack_size, starting_population_dir, no_gen_save).unwrap(),
                _ => panic!("Invalid population size.  Valid populations sizes: [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4]")
            };
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}

pub fn compete_cli(
    north_player: String,
    east_player: String,
    south_player: String,
    west_player: String,
    num_games: usize,
) {
    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (4 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let mut north = NeuralNetwork::new();
            let mut east = NeuralNetwork::new();
            let mut south = NeuralNetwork::new();
            let mut west = NeuralNetwork::new();
            north.load_from_file(north_player.as_str())?;
            east.load_from_file(east_player.as_str())?;
            south.load_from_file(south_player.as_str())?;
            west.load_from_file(west_player.as_str())?;
            let mut north_south_score = 0;
            let mut east_west_score = 0;
            let games_to_win: usize = num_games / 2;
            for _ in 0..num_games {
                match play_euchre(&north, &east, &south, &west) {
                    Team::NorthSouth => north_south_score += 1,
                    Team::EastWest => east_west_score += 1,
                }
                println!(
                    "North/South Score: {}, East/West Score: {}",
                    north_south_score, east_west_score
                );
                if north_south_score > games_to_win {
                    println!("North/South team wins!");
                } else if east_west_score > games_to_win {
                    println!("East/West team wins!");
                }
            }
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}

pub fn stats_cli(file: String) {
    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (1 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let mut nn = NeuralNetwork::new();
            nn.load_from_file(file.as_str())?;
            nn.stats();
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}

pub fn breed_cli(parent1_file: String, parent2_file: String, child_file: String) {
    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (1 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let mut parent1 = NeuralNetwork::new();
            let mut parent2 = NeuralNetwork::new();
            parent1.load_from_file(parent1_file.as_str())?;
            parent2.load_from_file(parent2_file.as_str())?;
            // TODO: mutation rate and magnitude as arguments
            let child = parent1.crossover(&parent2, 0.01, 0.1);
            child.save_to_file(child_file.as_str())?;
            Ok(())
        })
        .unwrap();

    handle.join().unwrap().ok();
}

pub fn tutor_cli(tutor_file: String, left_file: String, right_file: String, ally_file: String) {
    // max supported population size + 31, don't ask why
    let stack_size: usize = mem::size_of::<Organism>() * (4 + 31);

    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || -> std::io::Result<()> {
            let mut tutor = NeuralNetwork::new();
            let mut left: NeuralNetwork = NeuralNetwork::new();
            let mut right = NeuralNetwork::new();
            let mut ally = NeuralNetwork::new();
            tutor.load_from_file(tutor_file.as_str())?;
            left.load_from_file(left_file.as_str())?;
            right.load_from_file(right_file.as_str())?;
            ally.load_from_file(ally_file.as_str())?;
            tutor.tutor_mode = true;
            loop {
                play_euchre(&tutor, &left, &ally, &right);
                loop {
                    print!("Do you want to continue? [Y/n]: ");
                    std::io::stdout().flush().expect("Failed to flush stdout");

                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let input = input.trim().to_lowercase();
                    if input == "y" || input == "Y" || input == "" {
                        println!("Continuing...");
                        break;
                    } else if input == "n" || input == "N" {
                        println!("Exiting...");
                        return Ok(());
                    } else {
                        println!("Invalid input, please enter 'Y' or 'N'");
                    }
                }
            }
        })
        .unwrap();

    handle.join().unwrap().ok();
}
