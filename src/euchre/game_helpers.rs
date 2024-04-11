use super::enums::Position;

pub fn left_player(player: &Position) -> &Position {
    match player {
        Position::North => return &Position::East,
        Position::East => return &Position::South,
        Position::South => return &Position::West,
        Position::West => return &Position::North,
        _ => panic!("invalid position"),
    }
}
