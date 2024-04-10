use strum_macros::EnumCount as EnumCountMacro;

#[derive(EnumCountMacro)]
pub enum Action {
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
