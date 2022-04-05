use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn run(config: serde_json::Value) {
    //turning config into useful address to bind the listener on
    let listener_addr =
        config["ip"].as_str().unwrap().to_string() + ":" + config["port"].as_str().unwrap();

    let listener = TcpListener::bind(listener_addr).unwrap();

    //hashmap to keep track of unique users and their TcpStreams
    let users: HashMap<&str, &TcpStream> = HashMap::new();

    //iterate through incoming streams and send them to a handler
    for stream in listener.incoming() {
        
        let mut stream = stream.unwrap();

        //main loop to read off of the que and respond to requests
        loop {

            //read the buffer into an array and turn it into a String for easier manipulation
            let mut buf = [0; 1024];
            stream.read(&mut buf).unwrap();
            let request = String::from_utf8_lossy(&buf);

            //all request sent should be using you couldve texted protocol, if they aren't break out of main loop
            if !request.starts_with("YTCP") {
                break;
            }

            //deref coercion and pattern matching to separate on carriage return and separate data from header
            if let &[header, data, ..] = &*request.split("\r\n").collect::<Vec<&str>>() {

                //seperate header from YTCP to get the procedure call
                let procedure = &header[5..header.len()];

                //generate a respone based on the procedure
                let response = match procedure {
                    "check username" => check_username(data, users),
                    _ => "Error: Procedure Not Found",
                }

                //format response to use YTC protocol
                let response = format!("YTCP\r\n{}", response);
                
                //write the response back to the client and continue on
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}

//this is going to need to be changed up once I have access to an IDE
fn check_username<'a>(data: &'a str, stream: &'a TcpStream, users: &mut HashMap<&str, &TcpStream>) -> &str{
    if users.contains_key(&data){
        "fail"
    }
    else{
        users.push(data, stream);
        "success"
    }
}
