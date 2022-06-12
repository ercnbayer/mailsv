
pub mod mail_handler;
use std::net::TcpListener;
use mail_handler::receive_updates_on_socket;
use anyhow::{Result};
fn main()->Result<()> {
    //println!("Hello, world!");
  let socket = TcpListener::bind("127.0.0.1:587")?;//change bind value to actual one
    socket.incoming().for_each(|res| match res {
        Ok(conn) => {if let Err(err) = receive_updates_on_socket(conn) {
            println!("Closed SMTP session due to error : {}", err);
            
        }
    }
        Err(err) => eprintln!("Failure accepting connection :{}", err),
    });
    Ok(())
}










