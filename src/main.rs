use rand::prelude::*;
use std::mem;
use std::thread;

mod neural_network;
use neural_network::*;

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

fn main() {
    // Game (20)
    // self team score           - 10 indices (0-9 points)
    // opponent team score       - 10 indices (0-9 points)

    // Round (62)
    // dealer                    - 4 indices  (1 for each player, left/ally/right)
    // self team trick count     - 5 indices  (0-4 tricks)
    // opponent team trick count - 5 indices  (0-4 tricks)
    // upcard                    - 24 indices (1 for each card)
    // hand                      - 24 indices (1 for each card)

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

    // Trick (105 * 4 = 420)
    // lead player               - 4 indices (self/left/ally/right)
    // lead suit                 - 5 indices (4 suits, 1 "not set")
    // first card                - 24 indices (1 for each card)
    // second card               - 24 indices (1 for each card)
    // third card                - 24 indices (1 for each card)
    // fourth card               - 24 indices (1 for each card)

    // Total (554)

    // number of max simultaneously extant networks times 2
    const NUM_NETWORKS: usize = 4;
    let stack_size: usize = mem::size_of::<NeuralNetwork>() * NUM_NETWORKS * 2;

    // Spawn a thread with custom stack size
    let handle = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(|| {
            let mut inputs: [f64; INPUT_NODES] = [0.0; INPUT_NODES];
            let mut rng = rand::thread_rng();

            for i in 0..INPUT_NODES {
                inputs[i] = rng.gen::<f64>();
            }

            let mut nn1 = NeuralNetwork::new();
            nn1.init();
            let mut nn2 = NeuralNetwork::new();
            nn2.init();

            let child = nn1.crossover(&nn2, 0.01, 0.1);

            let result1 = nn1.query(&inputs);
            let result2 = nn2.query(&inputs);
            let result3 = child.query(&inputs);
            println!("{:?}\n{:?}\n{:?}", result1, result2, result3);
        })
        .unwrap(); // Handle the Result to check for errors

    // Wait for the thread to finish
    handle.join().unwrap();
}
