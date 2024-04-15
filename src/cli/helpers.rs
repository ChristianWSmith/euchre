use std::{mem, thread};

use crate::{
    euchre::{enums::Team, game::play_euchre},
    organism::{
        evolution::{evolve, Organism},
        neural_network::NeuralNetwork,
    },
};

// TODO: better error handling here
pub fn evolve_cli(population_size: usize, generations: usize, out_dir: String) {
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
