use std::env;
use std::io::{self, prelude::*, stdin, stdout};
use std::net::TcpStream;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //program takes in a socket address to connect to, if nothing in program exits
    let socket_address = match env::args().nth(1) {
        None => {
            eprintln!("No socket address");
            process::exit(0);
        }
        Some(address) => address,
    };

    let connection = match TcpStream::connect(socket_address) {
        Err(_) => {
            eprintln!("Invalid socket address");
            process::exit(0);
        }
        Ok(conn) => conn,
    };

    connection.set_nonblocking(true).unwrap();

    let connection = Arc::new(Mutex::new(connection));

    let con_clone1 = Arc::clone(&connection);
    let con_clone2 = Arc::clone(&connection);

    let user_input_loop = thread::spawn(move || loop {
        let mut buf = String::new();

        print!(">");
        stdout().flush().unwrap();

        stdin().read_line(&mut buf).unwrap();

        let mut buf = buf.split_whitespace();

        let procedure = match buf.next() {
            Some(proc) if proc.trim() != "" => proc,
            _ => continue,
        };

        let data = buf.collect::<Vec<&str>>().join(" ");

        // format a you-couldve-texted protocol request
        let request = format!("YTCP {}\r\n{}\r\n", procedure, data);

        //acquire connections lock and send request as bytes
        let mut connection = con_clone1.lock().unwrap();
        connection.write(request.as_bytes()).unwrap();
        connection.flush().unwrap();
    });

    let server_reader = thread::spawn(move || loop {
        let mut buf = [0; 1024];

        let mut connection = con_clone2.lock().unwrap();

        match connection.read(&mut buf) {
            Ok(_) => {
                println!("\n{}", String::from_utf8_lossy(&buf));

                //on a successful read, reformat stdout
                print!(">");
                stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => panic!("Encountered io Error: {}", e),
        }
    });

    user_input_loop.join().unwrap();
    server_reader.join().unwrap();
}
