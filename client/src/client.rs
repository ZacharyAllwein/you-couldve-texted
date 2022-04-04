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
            println!("{}", self.send_server("check username", &username));
    }
}

    //handles most of the messaging to server
    fn send_server(&mut self, method: &str, data: &String) -> String{
        //format a you-couldve-texted protocol request
        let request = format!("YTCP {}\r\n{}", method, data);

        //Send the request as bytes
        self.connection.write(request.as_bytes()).unwrap();
        self.connection.flush().unwrap();

        //Read in any response and return it
        let mut buf = [0; 1024];
        self.connection.read(&mut buf).unwrap();
        let response = String::from_utf8_lossy(&buf);

        response.to_string()
    }
}
