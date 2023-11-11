use std::collections::HashMap;
use std::fmt::{Display, Pointer};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::result;
use std::sync::Arc;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

type Result<T> = result::Result<T, ()>;

// fn handle_client(stream: TcpStream) {
//     // let mut reader = BufReader::new(stream.try_clone().unwrap());
//     // let mut writer = BufWriter::new(stream);
//
//     let mut reader = BufReader::new(stream);
//
//     let mut line = String::new();
//     reader.read_line(&mut line).unwrap();
//     println!("Received: {}", line.trim());
//
//     // writer.write_all(line.as_bytes()).unwrap();
//     // writer.flush().unwrap();
// }

enum Message {
    ClientConnected {
        author: Arc<TcpStream>,
    },
    ClientDisconected {
        author: Arc<TcpStream>,
    },
    NewMessage {
        author: Arc<TcpStream>,
        bytes: Vec<u8>,
    },
}

struct Client {
    conn: Arc<TcpStream>,
}

fn server(messages: Receiver<Message>) {
    let mut clients = HashMap::new();
    loop {
        let msg = messages.recv().expect("could not receive message");
        match msg {
            Message::ClientConnected { author } => {
                let addr = author.as_ref().peer_addr().unwrap();
                clients.insert(addr, Client { conn: author });
            }
            Message::ClientDisconected { author } => {
                let addr = author.as_ref().peer_addr().unwrap();
                clients.remove(&addr);
            }
            Message::NewMessage { author, bytes } => {
                let author_addr = author.as_ref().peer_addr().unwrap();
                for (addr, client) in clients.iter() {
                    if *addr != author_addr {
                        let _ = client.conn.as_ref().write_all(&bytes);
                    }
                }
            }
        }
    }
}

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()> {
    messages
        .send(Message::ClientConnected {
            author: stream.clone(),
        })
        .map_err(|err| {
            eprintln!("ERROR: could connect to the server thread: {err}");
        })?;
    let mut buffer = Vec::new();
    buffer.resize(64, 0);
    loop {
        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprintln!("ERROR: could not read messagem from the client: {err}");
            let _ = messages.send(Message::ClientDisconected {
                author: stream.clone(),
            });
        })?;
        messages
            .send(Message::NewMessage {
                author: stream.clone(),
                bytes: buffer[0..n].to_vec(),
            })
            .map_err(|err| {
                eprintln!("ERROR: could connect to the server thread: {err}");
            })?;
    }
}

const SAFE_MODE: bool = true;

struct Sensitve<T>(T);

impl<T> Display for Sensitve<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        if SAFE_MODE {
            write!(f, "[HIDDEN]")
        } else {
            inner.fmt(f)
        }
    }
}

fn main() -> Result<()> {
    let adress = "0.0.0.0:6969";
    let listener = TcpListener::bind(adress).map_err(|err| {
        eprintln!("ERROR: could not bind {adress}: {err}");
    })?;
    println!("listening on {}", Sensitve(adress));

    let (message_sender, message_receiver) = channel();
    thread::spawn(|| server(message_receiver));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let message_sender = message_sender.clone();
                thread::spawn(|| client(stream, message_sender));
            }
            Err(err) => {
                eprint!("Could not spawn a thread: {}", err);
            }
        }
    }
    Ok(())
}
