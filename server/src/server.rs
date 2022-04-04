use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

pub struct Server{
    listener: TcpListener,
    users: HashMap<String, TcpStream>,
}

impl Server {

    //initializer that builds a serde_json parsed into a address and builds the server
    pub fn new(config: serde_json::Value) -> io::Result<Server>{
        let listener_addr =
            config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

        Ok(Server {
            listener: TcpListener::bind(listener_addr)?,
            users: HashMap::new(),
        })
    }


    //starts the server mainloop for handling individual requests
    pub fn start(&self){
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
    
            Server::handle_request(stream);
        }
    }

    fn handle_request(mut stream: TcpStream){
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
                
                let response = Server::match_procedure(procedure, data);

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        
        }
    }

    fn match_procedure(procedure: &str, data: &str) -> &'static str{

        let procedure = match procedure{
            "check username" => |data| {
                println!("{}", data);
                "good"
            },
            _ => |_| "bads",
        };

        procedure(data)
    }
}