use std::net::{TcpListener, TcpStream};
use log::{info, error};

fn handle_connection(stream: TcpStream) {
   //...
   println!("handle_connection ------ {:?}", stream);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4132").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("listener error {}", e)
            }
        }
    }
    Ok(())
}