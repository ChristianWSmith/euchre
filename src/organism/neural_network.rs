use lazy_static::lazy_static;
use rand::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use crate::euchre::enums::{ActionIndex, StateIndex};
use crate::organism::helpers::get_player_action;

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
    static ref ACTIVATION_FUNCTION_TYPES: [ActivationFunctionType; ActivationFunctionType::COUNT] = {
        let mut activation_function_types =
            [ActivationFunctionType::Sigmoid; ActivationFunctionType::COUNT];
        let mut i = 0;
        for aft in ActivationFunctionType::iter() {
            activation_function_types[i] = aft;
            i += 1;
        }
        activation_function_types
    };
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, EnumIter, EnumCount)]
#[repr(C)]
enum ActivationFunctionType {
    Sigmoid,
    LeakyRelu,
    Tanh,
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct NeuralNetwork {
    pub tutor_mode: bool,
    weights_input_hidden: [[f64; HIDDEN_NODES]; StateIndex::COUNT],
    weights_hidden_output: [[f64; ActionIndex::COUNT]; HIDDEN_NODES],
    connections_input_hidden: [[bool; HIDDEN_NODES]; StateIndex::COUNT],
    connections_hidden_output: [[bool; ActionIndex::COUNT]; HIDDEN_NODES],
    hidden_biases: [f64; HIDDEN_NODES],
    final_biases: [f64; ActionIndex::COUNT],
    hidden_activation_functions: [ActivationFunctionType; HIDDEN_NODES],
    final_activation_functions: [ActivationFunctionType; ActionIndex::COUNT],
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let weights_input_hidden = [[0.0; HIDDEN_NODES]; StateIndex::COUNT];
        let weights_hidden_output = [[0.0; ActionIndex::COUNT]; HIDDEN_NODES];
        let connections_input_hidden = [[true; HIDDEN_NODES]; StateIndex::COUNT];
        let connections_hidden_output = [[true; ActionIndex::COUNT]; HIDDEN_NODES];
        let hidden_biases = [0.0; HIDDEN_NODES];
        let final_biases = [0.0; ActionIndex::COUNT];
        let hidden_activation_functions = [ActivationFunctionType::Sigmoid; HIDDEN_NODES];
        let final_activation_functions = [ActivationFunctionType::Sigmoid; ActionIndex::COUNT];

        NeuralNetwork {
            tutor_mode: false,
            weights_input_hidden,
            weights_hidden_output,
            connections_input_hidden,
            connections_hidden_output,
            hidden_biases,
            final_biases,
            hidden_activation_functions,
            final_activation_functions,
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        // Hidden
        for j in 0..HIDDEN_NODES {
            // Hidden - Bias
            self.hidden_biases[j] = rng.gen_range(-0.5..0.5);
            // Hidden - Activation
            self.hidden_activation_functions[j] =
                ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            for i in 0..StateIndex::COUNT {
                // Hidden - Weight
                self.weights_input_hidden[i][j] = rng.gen_range(-0.5..0.5);
                // Hidden - Connection
                self.connections_input_hidden[i][j] = rng.gen::<f64>() < 0.5;
            }
        }
        // Final
        for j in 0..ActionIndex::COUNT {
            // Final - Bias
            self.final_biases[j] = rng.gen_range(-0.5..0.5);
            // Final - Activation
            self.final_activation_functions[j] =
                ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            for i in 0..HIDDEN_NODES {
                // Final - Weight
                self.weights_hidden_output[i][j] = rng.gen_range(-0.5..0.5);
                // Final - Connection
                self.connections_hidden_output[i][j] = rng.gen::<f64>() < 0.5;
            }
        }
    }

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn leaky_relu(x: f64) -> f64 {
        x.max(x * 0.01)
    }

    fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    pub fn crossover(
        &self,
        partner: &NeuralNetwork,
        mutation_rate: f64,
        mutation_magnitude: f64,
    ) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        let mut child = NeuralNetwork::new();

        // Combination
        // Combination - Hidden
        for j in 0..HIDDEN_NODES {
            // Combination - Hidden - Bias
            if rng.gen::<f64>() < 0.5 {
                child.hidden_biases[j] = self.hidden_biases[j];
            } else {
                child.hidden_biases[j] = partner.hidden_biases[j];
            }
            // Combination - Hidden - Activation
            if rng.gen::<f64>() < 0.5 {
                child.hidden_activation_functions[j] = self.hidden_activation_functions[j];
            } else {
                child.hidden_activation_functions[j] = partner.hidden_activation_functions[j];
            }
            for i in 0..StateIndex::COUNT {
                // Combination - Hidden - Weight and Connection
                if rng.gen::<f64>() < 0.5 {
                    child.weights_input_hidden[i][j] = self.weights_input_hidden[i][j];
                    child.connections_input_hidden[i][j] = self.connections_input_hidden[i][j];
                } else {
                    child.weights_input_hidden[i][j] = partner.weights_input_hidden[i][j];
                    child.connections_input_hidden[i][j] = partner.connections_input_hidden[i][j];
                }
            }
        }
        // Combination - Final
        for j in 0..ActionIndex::COUNT {
            // Combination - Final - Bias
            if rng.gen::<f64>() < 0.5 {
                child.final_biases[j] = self.final_biases[j];
            } else {
                child.final_biases[j] = partner.final_biases[j];
            }
            // Combination - Final - Activation
            if rng.gen::<f64>() < 0.5 {
                child.final_activation_functions[j] = self.final_activation_functions[j];
            } else {
                child.final_activation_functions[j] = partner.final_activation_functions[j];
            }
            for i in 0..HIDDEN_NODES {
                // Combination - Final - Weight and Connection
                if rng.gen::<f64>() < 0.5 {
                    child.weights_hidden_output[i][j] = self.weights_hidden_output[i][j];
                    child.connections_hidden_output[i][j] = self.connections_hidden_output[i][j];
                } else {
                    child.weights_hidden_output[i][j] = partner.weights_hidden_output[i][j];
                    child.connections_hidden_output[i][j] = partner.connections_hidden_output[i][j];
                }
            }
        }

        // Mutation
        // Mutation - Hidden
        for j in 0..HIDDEN_NODES {
            // Mutation - Hidden - Bias
            if rng.gen::<f64>() < mutation_rate {
                child.hidden_biases[j] += rng.gen_range(-mutation_magnitude..mutation_magnitude);
            }
            // Mutation - Hidden - Activation
            if rng.gen::<f64>() < mutation_rate {
                child.hidden_activation_functions[j] =
                    ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            }
            for i in 0..StateIndex::COUNT {
                // Mutation - Hidden - Connection
                if rng.gen::<f64>() < mutation_rate {
                    child.connections_input_hidden[i][j] = !child.connections_input_hidden[i][j];
                }
                // Mutation - Hidden - Weight
                if child.connections_input_hidden[i][j] && rng.gen::<f64>() < mutation_rate {
                    child.weights_input_hidden[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }
        // Mutation - Final
        for j in 0..ActionIndex::COUNT {
            // Mutation - Final - Bias
            if rng.gen::<f64>() < mutation_rate {
                child.final_biases[j] += rng.gen_range(-mutation_magnitude..mutation_magnitude);
            }
            // Mutation - Final - Activation
            if rng.gen::<f64>() < mutation_rate {
                child.final_activation_functions[j] =
                    ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            }
            for i in 0..HIDDEN_NODES {
                // Mutation - Final - Connection
                if rng.gen::<f64>() < mutation_rate {
                    child.connections_hidden_output[i][j] = !child.connections_hidden_output[i][j];
                }
                // Mutation - Final - Weight
                if child.connections_hidden_output[i][j] && rng.gen::<f64>() < mutation_rate {
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
            let mut sum = self.hidden_biases[i];
            for j in 0..StateIndex::COUNT {
                if self.connections_input_hidden[j][i] {
                    sum += inputs[j] * self.weights_input_hidden[j][i];
                }
            }
            match self.hidden_activation_functions[i] {
                ActivationFunctionType::Sigmoid => hidden_outputs[i] = NeuralNetwork::sigmoid(sum),
                ActivationFunctionType::LeakyRelu => {
                    hidden_outputs[i] = NeuralNetwork::leaky_relu(sum)
                }
                ActivationFunctionType::Tanh => hidden_outputs[i] = NeuralNetwork::tanh(sum),
            }
        }

        for i in 0..ActionIndex::COUNT {
            let mut sum = self.final_biases[i];
            for j in 0..HIDDEN_NODES {
                if self.connections_hidden_output[j][i] {
                    sum += hidden_outputs[j] * self.weights_hidden_output[j][i];
                }
            }
            match self.final_activation_functions[i] {
                ActivationFunctionType::Sigmoid => final_outputs[i] = NeuralNetwork::sigmoid(sum),
                ActivationFunctionType::LeakyRelu => {
                    final_outputs[i] = NeuralNetwork::leaky_relu(sum)
                }
                ActivationFunctionType::Tanh => final_outputs[i] = NeuralNetwork::tanh(sum),
            }
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
                if self.tutor_mode {
                    return get_player_action(
                        inputs,
                        available_actions,
                        &ActionIndex::from_usize(action_index),
                    );
                }
                return ActionIndex::from_usize(action_index);
            }
        }
        panic!("No available actions!")
    }

    fn to_bytes(&self) -> &[u8] {
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
        self.connections_input_hidden = in_network.connections_input_hidden;
        self.connections_hidden_output = in_network.connections_hidden_output;
        self.hidden_biases = in_network.hidden_biases;
        self.final_biases = in_network.final_biases;
        self.hidden_activation_functions = in_network.hidden_activation_functions;
        self.final_activation_functions = in_network.final_activation_functions;
        Ok(())
    }

    pub fn stats(&self) {
        let mut connected_count = 0;
        let mut disconnected_count = 0;
        let mut sigmoid_count = 0;
        let mut leaky_relu_count = 0;
        let mut tanh_count = 0;
        for j in 0..HIDDEN_NODES {
            match self.hidden_activation_functions[j] {
                ActivationFunctionType::Sigmoid => sigmoid_count += 1,
                ActivationFunctionType::LeakyRelu => leaky_relu_count += 1,
                ActivationFunctionType::Tanh => tanh_count += 1,
            }
            for i in 0..StateIndex::COUNT {
                match self.connections_input_hidden[i][j] {
                    true => connected_count += 1,
                    false => disconnected_count += 1,
                }
            }
        }
        for j in 0..ActionIndex::COUNT {
            match self.final_activation_functions[j] {
                ActivationFunctionType::Sigmoid => sigmoid_count += 1,
                ActivationFunctionType::LeakyRelu => leaky_relu_count += 1,
                ActivationFunctionType::Tanh => tanh_count += 1,
            }
            for i in 0..HIDDEN_NODES {
                match self.connections_hidden_output[i][j] {
                    true => connected_count += 1,
                    false => disconnected_count += 1,
                }
            }
        }
        println!(
            "Connection Rate: {}",
            (connected_count as f64) / (connected_count as f64 + disconnected_count as f64)
        );
        println!(
            "Sigmoid Rate: {}",
            (sigmoid_count as f64)
                / (sigmoid_count as f64 + leaky_relu_count as f64 + tanh_count as f64)
        );
        println!(
            "Leaky Relu Rate: {}",
            (leaky_relu_count as f64)
                / (sigmoid_count as f64 + leaky_relu_count as f64 + tanh_count as f64)
        );
        println!(
            "Tanh Rate: {}",
            (tanh_count as f64)
                / (sigmoid_count as f64 + leaky_relu_count as f64 + tanh_count as f64)
        );
    }
}
