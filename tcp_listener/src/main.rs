use std::fmt;
use std::fmt::{Display, Pointer};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;
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
    ClientConnected,
    ClientDisconected,
    NewMessage
}

fn server() {

}

fn client(mut stream: TcpStream) {
    let _ = writeln!(stream, "Ola, cliente!").map_err(|err| {
        eprintln!("Could not write user message: {err}");
    });
    todo!()
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
    let adress = "127.0.0.1:6969";
    let listener = TcpListener::bind(adress).map_err(|err| {
        eprintln!("ERROR: could not bind {adress}: {err}");
    })?;
    println!("listening on {}", Sensitve(adress));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| client(stream));
            }
            Err(err) => {
                eprint!("Could not spawn a thread: {}", err);
            }
        }
    }
    Ok(())
}
