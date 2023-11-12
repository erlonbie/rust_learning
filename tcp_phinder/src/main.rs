use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::result;

type Result<T> = result::Result<T, ()>;

fn handle_client(stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).map_err(|err| {
            eprintln!("ERROR: NOT UTF-8: {err}");
        })?;
        println!("Received message: {:?} | from: {}", buffer.trim(), stream.peer_addr().unwrap());
        // TODO: Broadcast message to all clients
    }
}

fn handle_server_messages(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("{} abc", buffer.trim());
    }
    println!("Disconnected from server");
}

fn start_server(port: u32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New client connected: {:?}", stream.peer_addr().unwrap());
        // let stream_clone = stream.try_clone().unwrap();
        thread::spawn(move || {
            let _ = handle_client(stream);
        });
    }
}

fn start_client(server: &str, port: u32) {
    let mut stream = TcpStream::connect(format!("{}:{}", server, port)).expect("Couldn't connect to the server");
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

fn main() {
    let server_port = 6968;
    let client_port = 6968;
    thread::spawn(move || {
        start_server(server_port);
    });
    let handle = thread::spawn(move || {
        start_client("127.0.0.1", client_port);
    });
    handle.join().expect("waiting for the spawned thread to finish");
    // start_client("127.0.0.1", client_port);
}
