use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

pub fn start(config: serde_json::Value) {
    let listener_addr =
        config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

    let listener = TcpListener::bind(listener_addr).unwrap();

    let mut users: Vec<String> = Vec::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(stream);
    }
}

pub fn handle_request(mut stream: TcpStream) {

    println!("made connection");
    
    // let mut buf = [0; 1024];

    // stream.read_exact(&mut buf).unwrap();
    // let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

    // println!("{}", String::from_utf8_lossy(&msg));

    // stream.read_to_string(&mut request).unwrap();

    // println!("got here");

    // if !request.starts_with("YTCP") {
    //     return;
    // }
    // //deref coercion and pattern matching to separate on carriage return and separate data from header
    // if let &[header, data, ..] = &*request.split("\r\n").collect::<Vec<&str>>() {
    //     let method = &header[5..header.len()];
    //     println!("{}", method);
    // }

    // stream.write(b"").unwrap();
    // stream.flush().unwrap();
}
