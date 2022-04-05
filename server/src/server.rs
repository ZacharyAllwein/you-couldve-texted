use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn start(config: serde_json::Value) {
    //turning config into useful address to bind the listener on
    let listener_addr =
        config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

    let listener = TcpListener::bind(listener_addr).unwrap();
    let users: HashMap<&str, TcpStream> = HashMap::new();

    //iterate through incoming streams and send them to a handler
    for stream in listener.incoming() {
        handle_request(stream.unwrap(), &mut users);
    }
}

fn handle_request(mut stream: TcpStream, users: &mut HashMap<&str, TcpStream>) {
    loop {
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf);
        if !request.starts_with("YTCP") {
            return;
        }
        //deref coercion and pattern matching to separate on carriage return and separate data from header
        if let &[header, data, ..] = &*request.split("\r\n").collect::<Vec<&str>>() {
            let procedure = &header[5..header.len()];
            let response = match_procedure(procedure, data, &mut users);

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn match_procedure(
    procedure: &str,
    data: &str,
    users: &mut HashMap<&str, TcpStream>,
) -> &'static str {
    let procedure = match procedure {
        "check username" => |data| {
            println!("{}", data);
            "good"
        },
        _ => |_| "bads",
    };

    procedure(data)
}
