type Action = i32;

trait Game {
    // Method to initialize the game state
    fn initialize(&mut self);

    // Method to update the game state based on actions
    fn update(&mut self, action: Action);

    // Method to check if the game is over
    fn is_game_over(&self) -> bool;

    // Method to get the active agent
    fn active_agent(&self) -> Box<dyn Agent>;

    // Method to display the current state of the game
    fn display(&self);
}

trait Agent {
    // Method to choose an action based on the current state of the game
    fn choose_action(&self, available_actions: &Vec<Action>, game_state: &dyn Game) -> Action;
}



fn main() {
    println!("Hello, world!");
}
