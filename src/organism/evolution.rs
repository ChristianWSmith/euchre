use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
};

use crate::euchre::{enums::Team, game::play_euchre};

use super::neural_network::NeuralNetwork;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub struct Organism {
    pub brain: Option<NeuralNetwork>,
    pub lifetime: usize,
    pub generation: usize,
}

// must be a multiple of 4

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

pub fn evolve<const POPULATION_SIZE: usize, const BREEDING_POOL_SIZE: usize>(
    generations: usize,
    out_dir: String,
    thread_count: usize,
    stack_size: usize,
    starting_population_dir: Option<String>,
    no_gen_save: bool,
) -> Result<Organism, Box<dyn Error>> {
    // Initialize
    println!("Initializing");
    let mut organisms: [Organism; POPULATION_SIZE] = [Organism {
        brain: None,
        lifetime: 0,
        generation: 0,
    }; POPULATION_SIZE];
    let mut children: [Organism; BREEDING_POOL_SIZE] = [Organism {
        brain: None,
        lifetime: 0,
        generation: 0,
    }; BREEDING_POOL_SIZE];
    let mut breeder_indices: [usize; BREEDING_POOL_SIZE] = [0; BREEDING_POOL_SIZE];
    let mut rng = thread_rng();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .stack_size(stack_size)
        .build()
        .unwrap();

    let mut file_names: Vec<String> = Vec::new();
    let mut loading = false;
    let mut verbage = "Spawning";
    match starting_population_dir {
        Some(dir) => {
            let entries = fs::read_dir(dir);
            file_names = entries?
                .filter_map(|entry| {
                    entry
                        .ok()
                        .and_then(|e| e.path().to_str().map(|s| s.to_owned()))
                })
                .collect::<Vec<String>>();
            loading = true;
            verbage = "Loading";
        }
        None => (),
    }
    if loading && file_names.len() != POPULATION_SIZE {
        panic!("Invalid generation size per starting pool");
    }

    println!("{} Organisms", verbage);

    let organism_count = Arc::new(Mutex::new(0));
    pool.install(|| {
        organisms
            .par_iter_mut()
            .enumerate()
            .for_each(|(_, organism)| {
                let organism_count_val;
                {
                    let mut organism_count_guard = organism_count.lock().unwrap();
                    *organism_count_guard += 1;
                    organism_count_val = *organism_count_guard;
                }
                println!(
                    "{} Organisms - {}/{}",
                    verbage, organism_count_val, POPULATION_SIZE
                );
                let mut nn = NeuralNetwork::new();
                if loading {
                    nn.load_from_file(file_names[organism_count_val - 1].as_str())
                        .unwrap();
                } else {
                    nn.init();
                }
                (*organism).brain = Some(nn);
            });
    });

    let mut population_indices = [0; POPULATION_SIZE];
    for i in 0..POPULATION_SIZE {
        population_indices[i] = i;
    }

    // Run Generations
    println!("Generations");
    let mut generation = 0;
    while generation < generations {
        generation += 1;
        population_indices.shuffle(&mut rand::thread_rng());

        println!("Generation {} - Playing Games", generation);
        let match_count = Arc::new(Mutex::new(0));
        pool.install(|| {
            breeder_indices
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, breed_index)| {
                    let match_count_val;
                    {
                        let mut match_count_guard = match_count.lock().unwrap();
                        *match_count_guard += 1;
                        match_count_val = *match_count_guard;
                    }
                    println!(
                        "Generation {} - Match {}/{}",
                        generation, match_count_val, BREEDING_POOL_SIZE
                    );
                    let index = i * 2;
                    *breed_index = if play_match(
                        &organisms[population_indices[index]],
                        &organisms[population_indices[index + 1]],
                    ) {
                        population_indices[index]
                    } else {
                        population_indices[index + 1]
                    };
                });
        });

        println!("Generation {} - Breeding Children", generation);
        // Select parents
        let mut parent_matchings: [(usize, usize); BREEDING_POOL_SIZE] =
            [(0, 0); BREEDING_POOL_SIZE];
        for i in 0..BREEDING_POOL_SIZE {
            let parent_indexes =
                rand::seq::index::sample(&mut rng, BREEDING_POOL_SIZE, 2).into_vec();
            parent_matchings[i] = (parent_indexes[0], parent_indexes[1]);
        }
        breeder_indices.sort();
        let child_count = Arc::new(Mutex::new(0));

        // Do breeding
        pool.install(|| {
            children.par_iter_mut().enumerate().for_each(|(i, child)| {
                let child_count_val;
                {
                    let mut child_count_guard = child_count.lock().unwrap();
                    *child_count_guard += 1;
                    child_count_val = *child_count_guard;
                }
                println!(
                    "Generation {} - Breeding Child {}/{}",
                    generation, child_count_val, BREEDING_POOL_SIZE
                );
                let (j, k) = parent_matchings[i];
                *child = Organism {
                    brain: Some(
                        organisms[breeder_indices[j]]
                            .brain
                            .as_ref()
                            .unwrap()
                            .crossover(
                                organisms[breeder_indices[k]].brain.as_ref().unwrap(),
                                0.01,
                                0.1,
                            ),
                    ),
                    lifetime: 0,
                    generation: generation,
                };
            });
        });

        // Write children back to organisms
        let mut check_cursor: usize = 0;
        let mut child_cursor = 0;
        for i in 0..POPULATION_SIZE {
            if check_cursor < BREEDING_POOL_SIZE && breeder_indices[check_cursor] == i {
                organisms[i].lifetime += 1;
                check_cursor += 1;
            } else {
                organisms[i] = children[child_cursor];
                child_cursor += 1;
            }
        }

        if !no_gen_save {
            println!("Generation {} - Saving Generation", generation);
            fs::create_dir_all(format!("{}/gen_{}", out_dir, generation))?;
            pool.install(|| {
                organisms.par_iter().enumerate().for_each(|(i, organism)| {
                    let filename = format!(
                        "{}/gen_{}/index({})-lifetime({})-generation({}).bin",
                        out_dir, generation, i, organism.lifetime, organism.generation
                    );
                    organism
                        .brain
                        .as_ref()
                        .unwrap()
                        .save_to_file(&filename)
                        .unwrap();
                });
            });
        }
    }

    // Round Robin for Champ
    println!("Tournament");
    let total_rounds = f64::log2(POPULATION_SIZE as f64) as usize;

    let mut alive = Vec::with_capacity(POPULATION_SIZE);
    for _ in 0..POPULATION_SIZE {
        alive.push(AtomicBool::new(true));
    }
    let alive = Arc::new(Mutex::new(alive));

    for round_number in 1..total_rounds + 1 {
        println!("Tournament Round {}/{}", round_number, total_rounds);
        let mut matchups: Vec<(usize, usize)> = Vec::with_capacity(POPULATION_SIZE / 2);
        let mut next_matchup: Vec<usize> = Vec::with_capacity(2);
        let alive_guard = alive.lock().unwrap();
        for (i, alive) in alive_guard.iter().enumerate() {
            if alive.load(Ordering::SeqCst) {
                next_matchup.push(i);
                if next_matchup.len() == 2 {
                    matchups.push((next_matchup[0], next_matchup[1]));
                    next_matchup.clear();
                }
            }
        }
        drop(alive_guard);
        let match_count = Arc::new(Mutex::new(0));

        pool.install(|| {
            matchups.par_iter().for_each(|(i, j)| {
                let match_count_val;
                {
                    let mut match_count_guard = match_count.lock().unwrap();
                    *match_count_guard += 1;
                    match_count_val = *match_count_guard;
                }
                println!(
                    "Trounament Round {}/{} - Match {}/{}",
                    round_number,
                    total_rounds,
                    match_count_val,
                    matchups.len()
                );
                let loser_index = match play_match(&organisms[*i], &organisms[*j]) {
                    true => *j,
                    false => *i,
                };
                let alive_guard = alive.lock().unwrap();
                alive_guard[loser_index].store(false, Ordering::SeqCst);
            });
        });
    }

    let alive_guard = alive.lock().unwrap();
    for (i, alive) in alive_guard.iter().enumerate() {
        if alive.load(Ordering::SeqCst) {
            organisms[i]
                .brain
                .unwrap()
                .save_to_file(format!("{}/champion.bin", out_dir).as_str())?;
            return Ok(organisms[i]);
        }
    }
    drop(alive_guard);
    panic!("no champion found");
}
