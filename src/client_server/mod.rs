use std::net::{TcpStream, SocketAddr};

use bevy::prelude::Resource;


#[derive(Resource)]
pub struct Connections {
    pub connections: Vec<Connection>
}

impl Connections {
    pub fn new () -> Connections
    {
        Connections {
            connections: vec!()
        }
    }
}

impl Connection {
    pub fn new(tcp_stream: TcpStream, socket_address: SocketAddr) -> Connection
    {
        Connection {
            tcp_stream,
            socket_address
        }
    }
}

pub struct Connection {
    tcp_stream: TcpStream,
    socket_address: SocketAddr
}