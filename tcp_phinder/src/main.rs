use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("Received message: {:?}", buffer.trim());
        // TODO: Broadcast message to all clients
    }
    println!("Client disconnected: {:?}", stream.peer_addr().unwrap());
}

fn start_server(port: u32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New client connected: {:?}", stream.peer_addr().unwrap());
        let stream_clone = stream.try_clone().unwrap();
        thread::spawn(move || {
            handle_client(stream_clone);
        });
    }
}

fn start_client(server: &str, port: u32) {
    let mut stream = TcpStream::connect(format!("{}:{}", server, port)).expect("Couldn't connect to the server");
    // if let Ok(stream) = TcpStream::connect(format!("{}:{}", server, port)) {
    //     println!("Connected to the server!");
    // } else {
    //     println!("Couldn't connect to server...");
    // }
    println!("Connected to server {}:{}", server, port);
    let stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || {
        handle_server_messages(stream_clone);
    });
    let mut reader = BufReader::new(std::io::stdin());
    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        if buffer.trim() == "/quit" {
            break;
        }
        stream.write_all(buffer.as_bytes()).unwrap();
    }
    println!("Disconnected from server");
}

fn handle_server_messages(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("{}", buffer.trim());
    }
    println!("Disconnected from server");
}
fn main() {
    let server_port = 6968;
    let client_port = 6969;
    thread::spawn(move || {
        start_server(server_port);
    });
    start_client("127.0.0.1", client_port);
}
