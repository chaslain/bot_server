use bevy::prelude::Component;

pub struct Player {
    p: Position,
    i: String,
}

#[derive(Component)]
pub struct Position {
    x: u32,
    y: u32,
}

pub struct Game {
    players: Vec<Player>,
}

// enum Update {
//     EntityCreate{e: Entity},
//     EntityDestroy{e: Entity}
// }
