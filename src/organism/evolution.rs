use crate::euchre::{enums::Team, game::play_euchre};

use super::neural_network::NeuralNetwork;
use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};

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
    brain: Option<NeuralNetwork>,
    lifetime: usize,
    generation: usize,
}

// must be a multiple of 4
pub const POPULATION_SIZE: usize = 4;
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

pub fn evolve(generations: usize) {
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
                // TODO: disallow self crossover
                organisms[i] = Organism {
                    brain: Some(
                        organisms[breeder_indices[rng.gen_range(0..BREEDING_POOL_SIZE - 1)]]
                            .brain
                            .unwrap()
                            .crossover(
                                &organisms
                                    [breeder_indices[rng.gen_range(0..BREEDING_POOL_SIZE - 1)]]
                                .brain
                                .unwrap(),
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
        for i in 0..POPULATION_SIZE {
            println!(
                "lifetime: {}, generation: {}",
                organisms[i].lifetime, organisms[i].generation
            );
        }
    }
}
