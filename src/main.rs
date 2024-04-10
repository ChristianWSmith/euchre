use rand::prelude::*;
use std::thread;
use std::mem;

const ACTION_COUNT: isize = 60;
enum Action {
    PlaySpadeNine,
    PlaySpadeTen,
    PlaySpadeJack,
    PlaySpadeQueen,
    PlaySpadeKing,
    PlaySpadeAce,
    PlayHeartNine,
    PlayHeartTen,
    PlayHeartJack,
    PlayHeartQueen,
    PlayHeartKing,
    PlayHeartAce,
    PlayDiamondNine,
    PlayDiamondTen,
    PlayDiamondJack,
    PlayDiamondQueen,
    PlayDiamondKing,
    PlayDiamondAce,
    PlayClubNine,
    PlayClubTen,
    PlayClubJack,
    PlayClubQueen,
    PlayClubKing,
    PlayClubAce,
    DiscardSpadeNine,
    DiscardSpadeTen,
    DiscardSpadeJack,
    DiscardSpadeQueen,
    DiscardSpadeKing,
    DiscardSpadeAce,
    DiscardHeartNine,
    DiscardHeartTen,
    DiscardHeartJack,
    DiscardHeartQueen,
    DiscardHeartKing,
    DiscardHeartAce,
    DiscardDiamondNine,
    DiscardDiamondTen,
    DiscardDiamondJack,
    DiscardDiamondQueen,
    DiscardDiamondKing,
    DiscardDiamondAce,
    DiscardClubNine,
    DiscardClubTen,
    DiscardClubJack,
    DiscardClubQueen,
    DiscardClubKing,
    DiscardClubAce,
    MakeUpcard,
    MakeUpcardAlone,
    PassUpcard,
    MakeSuitSpade,
    MakeSuitHeart,
    MakeSuitDiamond,
    MakeSuitClub,
    MakeSuitSpadeAlone,
    MakeSuitHeartAlone,
    MakeSuitDiamondAlone,
    MakeSuitClubAlone,
    PassSuit,
}

const INPUT_NODES: usize = 626;
const HIDDEN_NODES: usize = 1252;
const OUTPUT_NODES: usize = 60;

struct NeuralNetwork {
    weights_input_hidden: [[f64; HIDDEN_NODES]; INPUT_NODES],
    weights_hidden_output: [[f64; OUTPUT_NODES]; HIDDEN_NODES],
}

impl NeuralNetwork {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut weights_input_hidden = [[0.0; HIDDEN_NODES]; INPUT_NODES];
        let mut weights_hidden_output = [[0.0; OUTPUT_NODES]; HIDDEN_NODES];

        for i in 0..INPUT_NODES {
            for j in 0..HIDDEN_NODES {
                weights_input_hidden[i][j] = rng.gen_range(-0.5..0.5);
            }
        }

        for i in 0..HIDDEN_NODES {
            for j in 0..OUTPUT_NODES {
                weights_hidden_output[i][j] = rng.gen_range(-0.5..0.5);
            }
        }

        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn mutate(&mut self, rate: f64, magnitude: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..INPUT_NODES {
            for j in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < rate {
                    self.weights_input_hidden[i][j] += rng.gen_range(-magnitude..magnitude);
                }
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..OUTPUT_NODES {
                if rng.gen::<f64>() < rate {
                    self.weights_hidden_output[i][j] += rng.gen_range(-magnitude..magnitude);
                }
            }
        }
    }

    fn crossover(&self, partner: &NeuralNetwork) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        let mut child = NeuralNetwork::new();

        for i in 0..INPUT_NODES {
            for j in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_input_hidden[i][j] = self.weights_input_hidden[i][j];
                } else {
                    child.weights_input_hidden[i][j] = partner.weights_input_hidden[i][j];
                }
            }
        }

        for i in 0..HIDDEN_NODES {
            for j in 0..OUTPUT_NODES {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_hidden_output[i][j] = self.weights_hidden_output[i][j];
                } else {
                    child.weights_hidden_output[i][j] = partner.weights_hidden_output[i][j];
                }
            }
        }

        child
    }

    fn query(&self, inputs: &[f64; INPUT_NODES]) -> [f64; OUTPUT_NODES] {
        let mut hidden_outputs = [0.0; HIDDEN_NODES];
        let mut final_outputs = [0.0; OUTPUT_NODES];

        for i in 0..HIDDEN_NODES {
            let mut sum = 0.0;
            for j in 0..INPUT_NODES {
                sum += inputs[j] * self.weights_input_hidden[j][i];
            }
            hidden_outputs[i] = NeuralNetwork::sigmoid(sum);
        }

        for i in 0..OUTPUT_NODES {
            let mut sum = 0.0;
            for j in 0..HIDDEN_NODES {
                sum += hidden_outputs[j] * self.weights_hidden_output[j][i];
            }
            final_outputs[i] = NeuralNetwork::sigmoid(sum);
        }

        final_outputs
    }
}

fn main() {
    // Game (20)
    // self team score           - 10 indices (0-9 points)
    // opponent team score       - 10 indices (0-9 points)

    // Round (70)
    // dealer                    - 4 indices  (1 for each player, left/ally/right)
    // self team trick count     - 5 indices  (0-4 tricks)
    // opponent team trick count - 5 indices  (0-4 tricks)
    // upcard                    - 28 indices (1 for each card)
    // hand                      - 28 indices (1 for each card)

    // Bid Upcard (12)
    // self action               - 3 indices  (make, make alone, pass)
    // left opponent action      - 3 indices  (make, make alone, pass)
    // ally action               - 3 indices  (make, make alone, pass)
    // right opponent action     - 3 indices  (make, make alone, pass)

    // Bid Suit (36)
    // self action               - 9 indices  (4 makes, 4 make alones, pass)
    // left opponent action      - 9 indices  (4 makes, 4 make alones, pass)
    // ally action               - 9 indices  (4 makes, 4 make alones, pass)
    // right opponent action     - 9 indices  (4 makes, 4 make alones, pass)

    // Play (4)
    // trump suit                - 4 indices  (4 suits)

    // Trick (121 * 4 = 484)
    // lead player               - 4 indices (self/left/ally/right)
    // lead suit                 - 5 indices (4 suits, 1 "not set")
    // first card                - 28 indices (1 for each card)
    // second card               - 28 indices (1 for each card)
    // third card                - 28 indices (1 for each card)
    // fourth card               - 28 indices (1 for each card)

    // Set custom stack size (in bytes)
    const STACK_SIZE: usize = 8 * 1024 * 1024 * 1024; // 8GB

    // Spawn a thread with custom stack size
    let handle = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let size = mem::size_of::<NeuralNetwork>();
            println!("Size of NeuralNetwork struct: {} bytes", size);
            let mut inputs: [f64; INPUT_NODES] = [0.0; INPUT_NODES];
            let mut rng = rand::thread_rng();

            for i in 0..INPUT_NODES {
                inputs[i] = rng.gen::<f64>();
            }

            let nn1 = NeuralNetwork::new();
            let nn2 = NeuralNetwork::new();
            let nn3 = NeuralNetwork::new();

            let mut child = nn1.crossover(&nn2);
            child.mutate(0.1, 0.1);

            let result1 = nn1.query(&inputs);
            let result2 = nn2.query(&inputs);
            let result3 = child.query(&inputs);
            println!("{:?}\n{:?}\n{:?}", result1, result2, result3);
        })
        .unwrap(); // Handle the Result to check for errors

    // Wait for the thread to finish
    handle.join().unwrap();
}
