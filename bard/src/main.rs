use std::{net::{TcpListener, TcpStream}, io::{BufRead, BufReader, Write, stdin}};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut client_stream = stream.try_clone().unwrap();

        println!("Connection established from {}", stream.local_addr().unwrap().ip());

        loop {
            let mut buf = BufReader::new(&mut stream);
            let response = buf.lines().next().unwrap().unwrap();

            println!("{}", response);

            let mut stdin = stdin().lock();
            let input = stdin.lines().next().unwrap().unwrap();

            client_stream.write_all(input.as_bytes()).unwrap();
        }
    }
}

