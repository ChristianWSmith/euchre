use crate::euchre::{enums::Team, game::play_euchre};

use super::neural_network::NeuralNetwork;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

lazy_static! {
    static ref POPULATION_INDICES: [usize; POPULATION_SIZE] = {
        let mut indices = [0; POPULATION_SIZE];
        for i in 0..POPULATION_SIZE {
            indices[i] = i;
        }
        indices
    };
    static ref BREEDER_INDICES: [usize; POPULATION_SIZE / 2] = {
        let mut indices = [0; POPULATION_SIZE / 2];
        for i in 0..POPULATION_SIZE / 2 {
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
const POPULATION_SIZE: usize = 4;

fn play_match(organism1: Organism, organism2: Organism) -> bool {
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
    let mut breeders: [Organism; POPULATION_SIZE / 2] = [Organism {
        brain: None,
        lifetime: 0,
        generation: 0,
    }; POPULATION_SIZE / 2];
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
        for i in 0..POPULATION_SIZE / 2 {
            match play_match(
                organisms[population_indices[i * 2]],
                organisms[population_indices[i * 2 + 1]],
            ) {
                true => breeders[i] = organisms[population_indices[i * 2]],
                false => breeders[i] = organisms[population_indices[i * 2 + 1]],
            }
            breeders[i].lifetime += 1;
        }

        let mut breeder_indices: [usize; POPULATION_SIZE] = POPULATION_INDICES.clone();
        breeder_indices.shuffle(&mut rand::thread_rng());
        for i in 0..POPULATION_SIZE / 4 {
            organisms[i] = breeders[i];
            organisms[i * 2] = Organism {
                brain: Some(breeders[i * 2].brain.unwrap().crossover(
                    &breeders[i * 2 + 1].brain.unwrap(),
                    0.01,
                    0.1,
                )),
                lifetime: 0,
                generation: generation,
            };
        }
    }
}
