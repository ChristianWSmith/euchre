use rand::prelude::*;

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

const INPUT_NODES: usize = 4;
const HIDDEN_NODES: usize = 3;
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
                if rng.gen::<f64>() < 0.5 { // 50% chance of inheriting from self
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
        // Forward pass
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
    let inputs = [0.5, 0.2, 0.1, 0.3];

    let nn1 = NeuralNetwork::new();
    let nn2 = NeuralNetwork::new();

    let mut child = nn1.crossover(&nn2);
    child.mutate(0.1, 0.1);

    let result1 = nn1.query(&inputs);
    let result2 = nn2.query(&inputs);
    let result3 = child.query(&inputs);
    println!("{:?}\n{:?}\n{:?}", result1, result2, result3);
}

