use std::{error::Error, fs};

use crate::euchre::{enums::Team, game::play_euchre};

use super::neural_network::NeuralNetwork;
use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng};

lazy_static! {
    static ref POPULATION_INDICES: [usize; POPULATION_SIZE] = {
        let mut indices = [0; POPULATION_SIZE];
        for i in 0..POPULATION_SIZE {
            indices[i] = i;
        }
        indices
    };
    static ref BREEDER_INDICES: [usize; BREEDING_POOL_SIZE] = {
        let mut indices = [0; BREEDING_POOL_SIZE];
        for i in 0..BREEDING_POOL_SIZE {
            indices[i] = i;
        }
        indices
    };
}

#[derive(Clone, Copy)]
pub struct Organism {
    pub brain: Option<NeuralNetwork>,
    pub lifetime: usize,
    pub generation: usize,
}

// must be a multiple of 4
pub const POPULATION_SIZE: usize = 64;
const BREEDING_POOL_SIZE: usize = POPULATION_SIZE / 2;

fn play_match(organism1: &Organism, organism2: &Organism) -> bool {
    let (mut organism1_wins, mut orgaism2_wins) = (0, 0);
    for _ in 0..3 {
        match play_euchre(
            &organism1.brain.unwrap(),
            &organism2.brain.unwrap(),
            &organism1.brain.unwrap(),
            &organism2.brain.unwrap(),
        ) {
            Team::NorthSouth => organism1_wins += 1,
            Team::EastWest => orgaism2_wins += 1,
        }
        match (organism1_wins, orgaism2_wins) {
            (2, _) => return true,
            (_, 2) => return false,
            _ => (),
        }
    }
    panic!("couldn't finish match")
}

pub fn evolve(generations: usize) -> Result<Organism, Box<dyn Error>> {
    // Initialize
    let mut organisms: [Organism; POPULATION_SIZE] = [Organism {
        brain: None,
        lifetime: 0,
        generation: 0,
    }; POPULATION_SIZE];
    let mut breeder_indices: [usize; BREEDING_POOL_SIZE] = [0; BREEDING_POOL_SIZE];
    for i in 0..POPULATION_SIZE {
        let mut nn = NeuralNetwork::new();
        nn.init();
        organisms[i].brain = Some(nn);
    }

    // Run Generations
    let mut generation = 0;
    while generation < generations {
        generation += 1;

        let mut population_indices: [usize; POPULATION_SIZE] = POPULATION_INDICES.clone();
        population_indices.shuffle(&mut rand::thread_rng());
        for i in 0..BREEDING_POOL_SIZE {
            match play_match(
                &organisms[population_indices[i * 2]],
                &organisms[population_indices[i * 2 + 1]],
            ) {
                true => breeder_indices[i] = population_indices[i * 2],
                false => breeder_indices[i] = population_indices[i * 2 + 1],
            }
        }

        let mut rng = thread_rng();
        breeder_indices.sort();
        let mut check_cursor: usize = 0;
        for i in 0..POPULATION_SIZE {
            if check_cursor < BREEDING_POOL_SIZE && breeder_indices[check_cursor] == i {
                organisms[i].lifetime += 1;
                check_cursor += 1;
            } else {
                let parent_indexes =
                    rand::seq::index::sample(&mut rng, BREEDING_POOL_SIZE, 2).into_vec();
                organisms[i] = Organism {
                    brain: Some(
                        organisms[breeder_indices[parent_indexes[0]]]
                            .brain
                            .unwrap()
                            .crossover(
                                &organisms[breeder_indices[parent_indexes[1]]].brain.unwrap(),
                                0.01,
                                0.1,
                            ),
                    ),
                    lifetime: 0,
                    generation: generation,
                };
            }
        }

        println!("Generation {}", generation);
        fs::create_dir_all(format!("out/gen_{}", generation))?;
        for i in 0..POPULATION_SIZE {
            println!(
                "index: {}, lifetime: {}, generation: {}",
                i, organisms[i].lifetime, organisms[i].generation
            );
            organisms[i].brain.unwrap().save_to_file(
                format!(
                    "out/gen_{}/index({})-lifetime({})-generation({}).bin",
                    generation, i, organisms[i].lifetime, organisms[i].generation
                )
                .as_str(),
            )?;
        }
    }

    // Round Robin for Champ
    println!("Round Robin");
    let mut wins: [usize; POPULATION_SIZE] = [0; POPULATION_SIZE];
    let total_matches = (POPULATION_SIZE * (POPULATION_SIZE - 1)) / 2;
    let mut match_count = 0;
    for i in 0..POPULATION_SIZE {
        for j in i + 1..POPULATION_SIZE {
            match_count += 1;
            match play_match(&organisms[i], &organisms[j]) {
                true => wins[i] += 1,
                false => wins[j] += 1,
            }
            println!("champ match {}/{}, {:?}", match_count, total_matches, wins);
        }
    }
    let max_index = wins
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .map(|(idx, _)| idx)
        .unwrap();
    println!("{:?}", wins);
    return Ok(organisms[max_index]);
}
