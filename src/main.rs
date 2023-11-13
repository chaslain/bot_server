mod client_server;
mod game_processor;
use std::{
    env,
    net::TcpListener,
    io::Error
};

use bevy::prelude::{App, Startup, ResMut, Commands, World, IntoSystemConfigs};
use client_server::{Connections, Connection};

fn main() {
    App::new()
    .add_systems(Startup, init_connection_resource.before(listen))
    .add_systems(Startup, listen)
    .run();
}

fn init_connection_resource(world: &mut World)
{
    world.insert_resource::<Connections>(Connections::new());
}


fn listen(mut connections: ResMut<Connections>) {
    let args: Vec<String>  = env::args().collect();
    let mut principal_participants: usize = 2;

    let listener = TcpListener::bind("0.0.0.0:12221").expect("Failed to listen on port 12221");

    while connections.connections.len() < principal_participants
    {
        let connection = listener.accept();

        match connection
        {
            Ok((stream, address)) => {
                connections.connections.push(Connection::new(stream, address));

            },
            Err(e) => panic!("Could not initialize incoming connection - {}", e)

        }
    }
}