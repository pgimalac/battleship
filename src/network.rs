use crate::game::GameType;
use crate::quit;
use std::io::Read;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub const ATTACK: u8 = 42;
pub const CONFIRM: u8 = 43;

pub fn network_thread(mut mutex: Arc<Mutex<GameType>>, mut work: Arc<AtomicBool>) {
    let mut socket = match mutex.lock().unwrap().deref_mut() {
        GameType::Network { socket, .. } => socket.try_clone().unwrap(),
        _ => panic!("Not a network game"),
    };

    socket.set_read_timeout(None).unwrap();

    let size = 16;
    let mut buffer: Vec<u8> = vec![0; size];
    while work.load(Ordering::Relaxed) {
        let n = match socket.read(&mut buffer) {
            Ok(n) => {
                if n < size {
                    n
                } else {
                    println!("Message too long");
                    continue;
                }
            }
            _ => {
                println!("Error receiving.");
                continue;
            }
        };
        println!("Message received");
        if n == 3 && buffer[0] == ATTACK {
            let x = buffer[1];
            let y = buffer[2];
            println!("Message received : attack ({};{})", x, y);
            mutex.lock().unwrap().opponent_attack((x, y));
        } else if n == 4 && buffer[0] == CONFIRM {
            let x = buffer[1];
            let y = buffer[2];
            let b = buffer[3] != 0;
            println!(
                "Message received : confirm attack at ({};{}) as {}",
                x, y, b
            );
            mutex.lock().unwrap().confirm_attack((x, y), b);
        } else if n == 0 {
            quit(&mut mutex, &mut work);
        } else {
            println!("Unexpected message, length {}", buffer.len());
        }
    }
}
