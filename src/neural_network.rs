use lazy_static::lazy_static;
use rand::prelude::*;

lazy_static! {
    static ref INITIAL_INDICES: [usize; OUTPUT_NODES] = {
        let mut indices = [0; OUTPUT_NODES];
        for i in 0..OUTPUT_NODES {
            indices[i] = i;
        }
        indices
    };
}

pub const INPUT_NODES: usize = 554;
pub const OUTPUT_NODES: usize = 60;
const HIDDEN_NODES: usize = (INPUT_NODES + OUTPUT_NODES) * 2 / 3;

pub struct NeuralNetwork {
    weights_input_hidden: [[f64; HIDDEN_NODES]; INPUT_NODES],
    weights_hidden_output: [[f64; OUTPUT_NODES]; HIDDEN_NODES],
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let weights_input_hidden = [[0.0; HIDDEN_NODES]; INPUT_NODES];
        let weights_hidden_output = [[0.0; OUTPUT_NODES]; HIDDEN_NODES];

        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..INPUT_NODES {
            for j in 0..HIDDEN_NODES {
                self.weights_input_hidden[i][j] = rng.gen_range(-0.5..0.5);
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..OUTPUT_NODES {
                self.weights_hidden_output[i][j] = rng.gen_range(-0.5..0.5);
            }
        }
    }

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn crossover(
        &self,
        partner: &NeuralNetwork,
        mutation_rate: f64,
        mutation_magnitude: f64,
    ) -> NeuralNetwork {
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

        let mut rng = rand::thread_rng();
        for i in 0..INPUT_NODES {
            for j in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < mutation_rate {
                    child.weights_input_hidden[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..OUTPUT_NODES {
                if rng.gen::<f64>() < mutation_rate {
                    child.weights_hidden_output[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }

        child
    }

    pub fn query(&self, inputs: &[f64; INPUT_NODES]) -> [usize; OUTPUT_NODES] {
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

        let mut indices: [usize; OUTPUT_NODES] = INITIAL_INDICES.clone();
        indices.sort_by(|&a, &b| final_outputs[b].partial_cmp(&final_outputs[a]).unwrap());
        indices
    }
}
