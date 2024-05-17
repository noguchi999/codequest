use std::io::{stdin, Write, BufReader, BufRead};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn main() {
    let server_addr = "127.0.0.1:8880";
    let mut socket = TcpStream::connect(server_addr).expect("Failed to connect to server");
    socket.set_nonblocking(true).expect("Failed to set non-blocking mode");
    println!("Connected to server at {}", server_addr);
    start_thread(socket.try_clone().unwrap());
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(_) => {
                print!("{}", buffer);
                std::io::stdout().flush().unwrap();
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(_) => {
                println!("Server closed connection");
                break;
            }
        }
    });
}

fn input(msg: &str) -> String {
    if msg != "" { print!("{}", msg);};
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}