use std::{
    env,
    net::{SocketAddr, TcpListener, TcpStream},
};

use bevy::prelude::{NextState, ResMut, Resource, State, World};

use crate::GameState;

#[derive(Resource)]
pub struct Connections {
    pub connections: Vec<Connection>,
}

impl Connections {
    pub fn new() -> Connections {
        Connections {
            connections: vec![],
        }
    }
}

impl Connection {
    pub fn new(tcp_stream: TcpStream, socket_address: SocketAddr) -> Connection {
        Connection {
            tcp_stream,
            socket_address,
        }
    }
}

pub struct Connection {
    tcp_stream: TcpStream,
    socket_address: SocketAddr,
}

pub fn listen(mut connections: ResMut<Connections>, mut state: ResMut<NextState<GameState>>) {
    let principal_participants: usize = env::args()
        .collect::<Vec<String>>()
        .get(0)
        .expect("Please specify # of bots")
        .parse::<usize>()
        .expect("Must specify number of bots as a NUMBER.");

    let listener = TcpListener::bind("0.0.0.0:12221").expect("Failed to listen on port 12221");

    while connections.connections.len() < principal_participants {
        let connection = listener.accept();

        match connection {
            Ok((stream, address)) => {
                connections
                    .connections
                    .push(Connection::new(stream, address));
            }
            Err(e) => panic!("Could not initialize incoming connection - {}", e),
        }
    }

    state.set(GameState::Playing);
}
