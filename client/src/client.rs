use std::io::{self, prelude::*};
use std::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    //constructor that bubbles up errors.
    pub fn new(socket_address: String) -> io::Result<Client> {

        let mut stream = TcpStream::connect(socket_address)?;
        stream.set_nonblocking(true)?;

        Ok(Client {
            connection: stream,
        })
    }

    //login function, once connected makes the user input usernames until they enter a valid one.
    pub fn login(&mut self) {
        let mut username = String::new();

        loop {
            print!("Enter session username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut username).unwrap();
            while self.send_server("check username", &username).is_err() {}
    }
}

    //handles most of the messaging to server
    fn send_server(&mut self, method: &str, data: &String) -> io::Result<()>{
        //format a you-couldve-texted protocol request
        let request = format!("YTCP {}\r\n{}", method, data);

        //Send the request as bytes
        self.connection.write(request.as_bytes())?;
        self.connection.flush().unwrap();

        //Read in any response and return it
        // let mut response = String::new();
        // self.connection.read_to_string(&mut response).unwrap();
        // response

        Ok(())
    }
}
