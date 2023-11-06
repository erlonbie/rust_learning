use std::fmt::{Display, Pointer};
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, ()>;

fn handle_client(stream: TcpStream) {
    // let mut reader = BufReader::new(stream.try_clone().unwrap());
    // let mut writer = BufWriter::new(stream);

    let mut reader = BufReader::new(stream);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("Received: {}", line.trim());

    // writer.write_all(line.as_bytes()).unwrap();
    // writer.flush().unwrap();
}

const SAFE_MODE: bool = true;

struct Sensitve<T>(T);

impl<T> Display  for Sensitve<T> {
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

    for stream in listener.incoming() {
        let stream = stream.map_err(|err| {
            eprintln!("ERROR: could not connect to client: {err}");
        })?;
        println!("listening on {}, {}",Sensitve(adress), Sensitve(stream.peer_addr().unwrap()));
        handle_client(stream);
    }
    Ok(())
}
