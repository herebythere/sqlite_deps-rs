pub use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Params {
    pub filepath: PathBuf,
    pub number_of_connections: usize,
}

pub struct ConnectionPool {
    params: Params,
    incoming_connections: Vec<Connection>,
    outgoing_connections: Vec<Connection>,
}

impl ConnectionPool {
    pub fn from_params(params: Params) -> ConnectionPool {
        ConnectionPool {
            incoming_connections: Vec::new(),
            outgoing_connections: Vec::new(),
            params,
        }
    }

    pub fn get_connection(&mut self) -> Result<Connection, String> {
        if let Some(conn) = self.pop() {
            return Ok(conn);
        }

        match Connection::open(&self.params.filepath) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn set_connection(&mut self, conn: Connection) -> Result<(), String> {
        self.pop();

        let connection_count = self.outgoing_connections.len() + self.incoming_connections.len();
        if connection_count < self.params.number_of_connections {
            self.incoming_connections.push(conn);
        }

        Ok(())
    }

    fn pop(&mut self) -> Option<Connection> {
        if self.outgoing_connections.len() == 0 {
            while let Some(connection) = self.incoming_connections.pop() {
                self.outgoing_connections.push(connection);
            }
        }

        self.outgoing_connections.pop()
    }
}
