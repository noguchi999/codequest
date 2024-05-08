use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn main() {
    let server_addr = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_addr).expect("サーバの起動に失敗しました.");
    server.set_nonblocking(true).expect("利用できませんでした.");
    print!("サーバを起動しました. {}\n", server_addr);
}