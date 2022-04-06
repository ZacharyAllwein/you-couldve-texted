use std::io::{self, prelude::*};
use std::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    //constructor that bubbles up errors.
    pub fn new(socket_address: String) -> io::Result<Client> {
        Ok(Client {
            connection: TcpStream::connect(socket_address)?,
        })
    }

    //login function, once connected makes the user input usernames until they enter a valid one.
    pub fn login(&mut self) {
        loop {
            let mut username = String::new();
            print!("Enter session username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut username).unwrap();

            //sanitize data also turns username into a &str so thats cool?
            let username = username.trim();
            let response = self.send_server("check username", &username);

            if response.starts_with("success") {
                break;
            }
        }
    }

    //lots of duplication maybe I can make one function
    pub fn run(&mut self) {
        loop {
            let mut command = String::new();
            print!(">");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut command).unwrap();

            //sanitize data also turns username into a &str so thats cool?
            let command = command.trim();

            if command == "" {
                continue;
            }

            let commands = command.split_whitespace().collect::<Vec<&str>>();
            let response = self.send_server(commands[0], commands[1]);

            println!("{}", response);
        }
    }

    //handles most of the messaging to server
    fn send_server(&mut self, method: &str, data: &str) -> String {
        //format a you-couldve-texted protocol request
        let request = format!("YTCP {}\r\n{}\r\n", method, data);

        //Send the request as bytes
        self.connection.write(request.as_bytes()).unwrap();
        self.connection.flush().unwrap();

        //Read in any response and return it
        let mut buf = [0; 1024];
        self.connection.read(&mut buf).unwrap();

        let response = String::from_utf8_lossy(&buf);

        //split on carriage return + newline and extract actual response data
        response.split("\r\n").nth(1).unwrap().to_string()
    }
}
