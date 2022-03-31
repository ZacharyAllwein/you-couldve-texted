use std::env;
use std::process;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

fn main() {
    
    let socket_address = match env::args().nth(1){
        None => {
            eprintln!("No socket address");
            process::exit(1);
        },
        Some(address) => address,
    };

    let mut connection = Connection {connection: TcpStream::connect(socket_address).unwrap()};

    let mut username = String::new();
    
    print!("Enter session username: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username).unwrap();

    println!("username available: {}", connection.send_stuff(format!("check username {}", username)));
}

struct Connection{
    connection: TcpStream
}

impl Connection {
    fn send_stuff(&mut self, stuff: String) -> String{
        

        self.connection.write(stuff.as_bytes()).unwrap();
        self.connection.flush().unwrap();

        let mut response = String::new();
        self.connection.read_to_string(&mut response).unwrap();

        response
    }
}
