use std::env;
use std::process;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

fn main() {
    
    let socket_address = match env::args().nth(1){
        None => {
            eprintln!("No socket address");
            process::exit(0);
        },
        Some(address) => address,
    };

    let mut connection = TcpStream::connect(socket_address).unwrap();

    let mut username = String::new();
    
    print!("Enter session username: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut username).unwrap();

    println!("username available: {}", send_stuff(&mut connection, format!("check username:{}", username)));
}

fn send_stuff(connection: &mut TcpStream, stuff: String) -> String{
        

    connection.write(stuff.as_bytes()).unwrap();
    connection.flush().unwrap();

    let mut response = String::new();
    connection.read_to_string(&mut response).unwrap();

    response
}
