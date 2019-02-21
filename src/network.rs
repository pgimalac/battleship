use crate::game::GameType;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub const ATTACK: u8 = 42;
pub const CONFIRM: u8 = 43;

impl GameType {
    pub fn check_network(&mut self) -> Result<bool, String> {
        let mut socket = match self {
            GameType::Network { socket, .. } => socket.try_clone().unwrap(),
            _ => panic!("Not a network game"),
        };

        try_string!(socket.set_read_timeout(Some(Duration::from_nanos(1))));

        let size = 16;
        let mut buffer: Vec<u8> = vec![0; size];
        loop {
            let n = match socket.read(&mut buffer) {
                Ok(n) => {
                    if n < size {
                        n
                    } else {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            };
            if n == 3 && buffer[0] == ATTACK {
                let x = buffer[1];
                let y = buffer[2];
                println!("Message received : attack ({};{})", x, y);
                self.opponent_attack((x, y))?;
            } else if n == 4 && buffer[0] == CONFIRM {
                let x = buffer[1];
                let y = buffer[2];
                let b = buffer[3] != 0;
                println!(
                    "Message received : confirm attack at ({};{}) as {}",
                    x, y, b
                );
                self.confirm_attack((x, y), b)?;
            } else if n == 0 {
                return Err("Peer deconnected".to_string());
            } else {
                println!("Unexpected message, length {}", buffer.len());
            }
        }
    }
}

pub fn wait_client() -> Option<TcpStream> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    if let Ok((client, addr)) = listener.accept() {
        println!("A client was found : {}", addr);
        return Some(client);
    }
    None
}
