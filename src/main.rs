mod client_server;
mod game_processor;

use client_server::listen;

use bevy::prelude::{App, IntoSystemConfigs, Startup, States, World};
use client_server::Connections;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum GameState {
    #[default]
    Initializing, // the server is awaiting connections
    Playing, // the server is playing the game
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_systems(Startup, init_connection_resource.before(listen))
        .add_systems(Startup, listen)
        .run();
}

fn init_connection_resource(world: &mut World) {
    world.insert_resource::<Connections>(Connections::new());
}
