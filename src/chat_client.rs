use std::io::{stdin, Write, BufReader, BufRead};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn main() {
    let server_addr = "127.0.0.1:8880";
    let mut socket = TcpStream::connect(server_addr).expect("Failed to connect to server");
    socket.set_nonblocking(true).expect("Failed to set non-blocking mode");
    println!("Connected to server at {}", server_addr);
}