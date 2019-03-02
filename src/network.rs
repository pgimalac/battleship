use crate::model::game::GameType;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub const ATTACK: u8 = 42;
pub const CONFIRM: u8 = 43;

impl GameType {
    pub fn check_network(&mut self) -> Result<bool, String> {
        let mut socket = match self {
            GameType::Network { socket, .. } => socket.try_clone().map_err(|x| x.to_string())?,
            _ => panic!("Not a network game"),
        };

        let size = 16;
        let mut buffer: Vec<u8> = vec![0; size];
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
        Ok(false)
    }
}

pub fn create_host_socket() -> Result<TcpListener, String> {
    let mut tcp_list = TcpListener::bind("0.0.0.0:8080").map_err(|x| x.to_string());
    if let Ok(listener) = &mut tcp_list {
        listener.set_nonblocking(true).map_err(|x| x.to_string())?;
    }
    tcp_list
}

// TODO : improve error handling
pub fn find_host(address: &str) -> Result<TcpStream, String> {
    TcpStream::connect((address, 8080)).map_err(|x| x.to_string())
}

// TODO : improve error handling
pub fn wait_client(listener: &TcpListener) -> Option<TcpStream> {
    if let Ok((client, addr)) = listener.accept() {
        println!("A client was found : {}", addr);
        return Some(client);
    }
    None
}
