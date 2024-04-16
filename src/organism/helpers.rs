use std::{collections::HashMap, io::Write};

use strum::EnumCount;

use crate::euchre::enums::{ActionIndex, StateIndex};

use super::neural_network::{AvailableActions, NeuralNetworkInput};

pub fn get_player_action(
    inputs: &NeuralNetworkInput,
    available_actions: &AvailableActions,
    tutor_action: &ActionIndex,
) -> ActionIndex {
    describe_inputs(inputs);
    let options = get_choices(available_actions);
    let mut keys: Vec<usize> = options.keys().cloned().collect();
    keys.sort();

    println!("Options\n");
    for key in keys.iter() {
        println!("{}: {:?}", key, options.get(&key).unwrap());
    }

    loop {
        print!("Please choose an option from the above {:?}: ", keys);
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse input as usize
        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        if options.contains_key(&choice) {
            let action = *options.get(&choice).unwrap();
            if action == *tutor_action {
                println!("Correct!");
            } else {
                println!("Incorrect, tutor advises {:?} instead", tutor_action);
            }
            print!("Press enter to continue");
            std::io::stdout().flush().expect("Failed to flush stdout");
            std::io::stdin()
                .read_line(&mut String::new())
                .expect("Failed to read line");
            return action;
        }

        // Flush stdout
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
}

fn describe_inputs(inputs: &NeuralNetworkInput) {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().expect("Failed to flush stdout");
    println!("Information\n");
    for i in 0..StateIndex::COUNT {
        if inputs[i] == 1.0 {
            println!("{:?}", StateIndex::from_usize(i));
        }
    }
    println!("");
}

fn get_choices(available_actions: &AvailableActions) -> HashMap<usize, ActionIndex> {
    let mut options: HashMap<usize, ActionIndex> = HashMap::new();
    let mut key: usize = 1;
    for i in 0..available_actions.len() {
        if available_actions[i] {
            options.insert(key, ActionIndex::from_usize(i));
            key += 1;
        }
    }
    return options;
}
