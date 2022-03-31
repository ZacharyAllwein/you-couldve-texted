use std::io::prelude::*;
use std::net::TcpListener;

pub fn start(config: serde_json::Value) {
    let listener_addr =
        config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

    let listener = TcpListener::bind(listener_addr).unwrap();

    let mut users: Vec<String> = Vec::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let buf_string = String::from_utf8_lossy(&buffer);

        if buf_string.contains("check username"){
            let user_name = buf_string.split(':').collect::<Vec<&str>>()[1].to_string();

            if !users.contains(&user_name){
                println!("available");
                users.push(user_name);
            }
        }
    }
}
