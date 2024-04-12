use lazy_static::lazy_static;
use rand::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use strum::EnumCount;

use crate::euchre::enums::{ActionIndex, StateIndex};

const HIDDEN_NODES: usize = (StateIndex::COUNT + ActionIndex::COUNT) * 2 / 3;

pub type AvailableActions = [bool; ActionIndex::COUNT];
pub type NeuralNetworkInput = [f64; StateIndex::COUNT];

lazy_static! {
    static ref INITIAL_INDICES: [usize; ActionIndex::COUNT] = {
        let mut indices = [0; ActionIndex::COUNT];
        for i in 0..ActionIndex::COUNT {
            indices[i] = i;
        }
        indices
    };
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub struct NeuralNetwork {
    weights_input_hidden: [[f64; HIDDEN_NODES]; StateIndex::COUNT],
    weights_hidden_output: [[f64; ActionIndex::COUNT]; HIDDEN_NODES],
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let weights_input_hidden = [[0.0; HIDDEN_NODES]; StateIndex::COUNT];
        let weights_hidden_output = [[0.0; ActionIndex::COUNT]; HIDDEN_NODES];

        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..StateIndex::COUNT {
            for j in 0..HIDDEN_NODES {
                self.weights_input_hidden[i][j] = rng.gen_range(-0.5..0.5);
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..ActionIndex::COUNT {
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

        for i in 0..StateIndex::COUNT {
            for j in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_input_hidden[i][j] = self.weights_input_hidden[i][j];
                } else {
                    child.weights_input_hidden[i][j] = partner.weights_input_hidden[i][j];
                }
            }
        }

        for i in 0..HIDDEN_NODES {
            for j in 0..ActionIndex::COUNT {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_hidden_output[i][j] = self.weights_hidden_output[i][j];
                } else {
                    child.weights_hidden_output[i][j] = partner.weights_hidden_output[i][j];
                }
            }
        }

        let mut rng = rand::thread_rng();
        for i in 0..StateIndex::COUNT {
            for j in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < mutation_rate {
                    child.weights_input_hidden[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..ActionIndex::COUNT {
                if rng.gen::<f64>() < mutation_rate {
                    child.weights_hidden_output[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }

        child
    }

    fn query(&self, inputs: &NeuralNetworkInput) -> [f64; ActionIndex::COUNT] {
        let mut hidden_outputs = [0.0; HIDDEN_NODES];
        let mut final_outputs = [0.0; ActionIndex::COUNT];

        for i in 0..HIDDEN_NODES {
            let mut sum = 0.0;
            for j in 0..StateIndex::COUNT {
                sum += inputs[j] * self.weights_input_hidden[j][i];
            }
            hidden_outputs[i] = NeuralNetwork::sigmoid(sum);
        }

        for i in 0..ActionIndex::COUNT {
            let mut sum = 0.0;
            for j in 0..HIDDEN_NODES {
                sum += hidden_outputs[j] * self.weights_hidden_output[j][i];
            }
            final_outputs[i] = NeuralNetwork::sigmoid(sum);
        }

        final_outputs
    }

    pub fn get_action(
        &self,
        inputs: &NeuralNetworkInput,
        available_actions: &AvailableActions,
    ) -> ActionIndex {
        let final_outputs = self.query(inputs);
        let mut indices: [usize; ActionIndex::COUNT] = INITIAL_INDICES.clone();
        indices.sort_by(|&a, &b| final_outputs[b].partial_cmp(&final_outputs[a]).unwrap());
        for action_index in indices {
            if available_actions[action_index] {
                return ActionIndex::from_usize(action_index);
            }
        }
        panic!("No available actions!")
    }

    fn to_bytes(&self) -> &[u8] {
        // Get the raw bytes of the struct
        let raw_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self as *const _ as *const u8,
                std::mem::size_of::<NeuralNetwork>(),
            )
        };
        raw_bytes
    }

    fn from_bytes(bytes: &[u8]) -> NeuralNetwork {
        assert_eq!(bytes.len(), std::mem::size_of::<NeuralNetwork>());
        unsafe { std::ptr::read(bytes.as_ptr() as *const NeuralNetwork) }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let bytes = self.to_bytes();
        file.write_all(&bytes)?;
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let in_network = NeuralNetwork::from_bytes(&bytes);
        self.weights_input_hidden = in_network.weights_input_hidden;
        self.weights_hidden_output = in_network.weights_hidden_output;
        Ok(())
    }
}
