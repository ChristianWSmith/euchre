use lazy_static::lazy_static;
use rand::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

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

#[derive(PartialEq, Debug)]
#[repr(C)]
pub struct NeuralNetwork {
    weights_input_hidden: [[f64; HIDDEN_NODES]; StateIndex::COUNT],
    weights_hidden_output: [[f64; ActionIndex::COUNT]; HIDDEN_NODES],
    connections_input_hidden: [[bool; HIDDEN_NODES]; StateIndex::COUNT],
    connections_hidden_output: [[bool; ActionIndex::COUNT]; HIDDEN_NODES],
    hidden_biases: [f64; HIDDEN_NODES],
    final_biases: [f64; ActionIndex::COUNT],
    hidden_activation_functions: [ActivationFunctionType; HIDDEN_NODES],
    final_activation_functions: [ActivationFunctionType; ActionIndex::COUNT],
    hidden_activation_arguments: [f64; HIDDEN_NODES],
    final_activation_arguments: [f64; ActionIndex::COUNT],
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
        let hidden_activation_arguments = [0.0; HIDDEN_NODES];
        let final_activation_arguments = [0.0; ActionIndex::COUNT];

        NeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
            connections_input_hidden,
            connections_hidden_output,
            hidden_biases,
            final_biases,
            hidden_activation_functions,
            final_activation_functions,
            hidden_activation_arguments,
            final_activation_arguments,
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..StateIndex::COUNT {
            for j in 0..HIDDEN_NODES {
                self.weights_input_hidden[i][j] = rng.gen_range(-0.5..0.5);
                self.connections_input_hidden[i][j] = rng.gen::<f64>() < 0.5;
            }
        }
        for i in 0..HIDDEN_NODES {
            for j in 0..ActionIndex::COUNT {
                self.weights_hidden_output[i][j] = rng.gen_range(-0.5..0.5);
                self.connections_hidden_output[i][j] = rng.gen::<f64>() < 0.5;
            }
        }
        for i in 0..HIDDEN_NODES {
            self.hidden_biases[i] = rng.gen_range(-0.5..0.5);
            self.hidden_activation_functions[i] =
                ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            self.hidden_activation_arguments[i] =
                NeuralNetwork::default_activation_argument(&self.hidden_activation_functions[i]);
        }
        for i in 0..ActionIndex::COUNT {
            self.final_biases[i] = rng.gen_range(-0.5..0.5);
            self.final_activation_functions[i] =
                ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
            self.final_activation_arguments[i] =
                NeuralNetwork::default_activation_argument(&self.final_activation_functions[i]);
        }
    }

    fn default_activation_argument(activation_function: &ActivationFunctionType) -> f64 {
        let mut rng = rand::thread_rng();
        match activation_function {
            ActivationFunctionType::Sigmoid => 0.0,
            ActivationFunctionType::LeakyRelu => rng.gen_range(0.0..0.1),
            ActivationFunctionType::Tanh => 0.0,
        }
    }

    fn mutate_activation_argument(
        activation_function: &ActivationFunctionType,
        activation_argument: &f64,
        mutation_magnitude: &f64,
    ) -> f64 {
        let mut rng = rand::thread_rng();
        match activation_function {
            ActivationFunctionType::Sigmoid => {
                activation_argument + rng.gen_range(-*mutation_magnitude..*mutation_magnitude)
            }
            ActivationFunctionType::LeakyRelu => 0.0f64.max(
                activation_argument
                    + rng.gen_range(-*mutation_magnitude * 0.1..*mutation_magnitude * 0.1),
            ),
            ActivationFunctionType::Tanh => 0.0,
        }
    }

    fn sigmoid(x: f64) -> f64 {
        x.max(x)
    }

    fn leaky_relu(x: f64, activation_argument: f64) -> f64 {
        1.0 / (1.0 + (-x + activation_argument).exp())
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

        // Combination - Weights and Connections and Biases
        for j in 0..HIDDEN_NODES {
            if rng.gen::<f64>() < 0.5 {
                child.hidden_biases[j] = self.hidden_biases[j];
            } else {
                child.hidden_biases[j] = partner.hidden_biases[j];
            }
            for i in 0..StateIndex::COUNT {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_input_hidden[i][j] = self.weights_input_hidden[i][j];
                    child.connections_input_hidden[i][j] = self.connections_input_hidden[i][j];
                } else {
                    child.weights_input_hidden[i][j] = partner.weights_input_hidden[i][j];
                    child.connections_input_hidden[i][j] = partner.connections_input_hidden[i][j];
                }
            }
        }

        for j in 0..ActionIndex::COUNT {
            if rng.gen::<f64>() < 0.5 {
                child.final_biases[j] = self.final_biases[j];
            } else {
                child.final_biases[j] = partner.final_biases[j];
            }
            for i in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < 0.5 {
                    child.weights_hidden_output[i][j] = self.weights_hidden_output[i][j];
                    child.connections_hidden_output[i][j] = self.connections_hidden_output[i][j];
                } else {
                    child.weights_hidden_output[i][j] = partner.weights_hidden_output[i][j];
                    child.connections_hidden_output[i][j] = partner.connections_hidden_output[i][j];
                }
            }
        }

        // Combination - Activations and Arguments
        for i in 0..HIDDEN_NODES {
            if rng.gen::<f64>() < 0.5 {
                child.hidden_activation_functions[i] = self.hidden_activation_functions[i];
                child.hidden_activation_arguments[i] = self.hidden_activation_arguments[i];
            } else {
                child.hidden_activation_functions[i] = partner.hidden_activation_functions[i];
                child.hidden_activation_arguments[i] = partner.hidden_activation_arguments[i];
            }
        }
        for i in 0..ActionIndex::COUNT {
            if rng.gen::<f64>() < 0.5 {
                child.final_activation_functions[i] = self.final_activation_functions[i];
                child.final_activation_arguments[i] = self.final_activation_arguments[i];
            } else {
                child.final_activation_functions[i] = partner.final_activation_functions[i];
                child.final_activation_arguments[i] = partner.final_activation_arguments[i];
            }
        }

        // Mutation - Weights and Connections and Biases
        let mut rng = rand::thread_rng();
        for j in 0..HIDDEN_NODES {
            if rng.gen::<f64>() < mutation_rate {
                child.hidden_biases[j] += rng.gen_range(-mutation_magnitude..mutation_magnitude);
            }
            for i in 0..StateIndex::COUNT {
                if rng.gen::<f64>() < mutation_rate {
                    child.connections_input_hidden[i][j] = !child.connections_input_hidden[i][j];
                }
                if child.connections_input_hidden[i][j] && rng.gen::<f64>() < mutation_rate {
                    child.weights_input_hidden[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }
        for j in 0..ActionIndex::COUNT {
            if rng.gen::<f64>() < mutation_rate {
                child.final_biases[j] += rng.gen_range(-mutation_magnitude..mutation_magnitude);
            }
            for i in 0..HIDDEN_NODES {
                if rng.gen::<f64>() < mutation_rate {
                    child.connections_hidden_output[i][j] = !child.connections_hidden_output[i][j];
                }
                if child.connections_hidden_output[i][j] && rng.gen::<f64>() < mutation_rate {
                    child.weights_hidden_output[i][j] +=
                        rng.gen_range(-mutation_magnitude..mutation_magnitude);
                }
            }
        }

        // Mutation - Activations and Arguments
        for i in 0..HIDDEN_NODES {
            if rng.gen::<f64>() < mutation_rate {
                child.hidden_activation_functions[i] =
                    ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
                child.hidden_activation_arguments[i] = NeuralNetwork::default_activation_argument(
                    &child.hidden_activation_functions[i],
                );
            }
            if rng.gen::<f64>() < mutation_rate {
                child.hidden_activation_arguments[i] = NeuralNetwork::mutate_activation_argument(
                    &child.hidden_activation_functions[i],
                    &child.hidden_activation_arguments[i],
                    &mutation_magnitude,
                );
            }
        }
        for i in 0..ActionIndex::COUNT {
            if rng.gen::<f64>() < mutation_rate {
                child.final_activation_functions[i] =
                    ACTIVATION_FUNCTION_TYPES[rng.gen_range(0..ActivationFunctionType::COUNT)];
                child.final_activation_arguments[i] = NeuralNetwork::default_activation_argument(
                    &child.final_activation_functions[i],
                );
            }
            if rng.gen::<f64>() < mutation_rate {
                child.final_activation_arguments[i] = NeuralNetwork::mutate_activation_argument(
                    &child.final_activation_functions[i],
                    &child.final_activation_arguments[i],
                    &mutation_magnitude,
                )
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
                    hidden_outputs[i] =
                        NeuralNetwork::leaky_relu(sum, self.hidden_activation_arguments[i])
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
                    final_outputs[i] =
                        NeuralNetwork::leaky_relu(sum, self.final_activation_arguments[i])
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
        self.hidden_activation_arguments = in_network.hidden_activation_arguments;
        self.final_activation_arguments = in_network.final_activation_arguments;
        Ok(())
    }
}
