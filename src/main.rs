use std::net::{TcpListener, TcpStream};
use log::{info, error};

fn handle_connection() {
   //...
   println!("handle_connection ------");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4132").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_connection();
            }
            Err(e) => {
                println!("listener error {}", e)
            }
        }
    }
    Ok(())
}