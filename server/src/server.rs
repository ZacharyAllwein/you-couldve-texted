use std::io::prelude::*;
use std::net::TcpListener;

pub fn start(config: serde_json::Value) {
    let listener_addr =
        config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

    let listener = TcpListener::bind(listener_addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
