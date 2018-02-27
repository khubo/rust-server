/*
* Simple single threaded server
*/
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

// handle connections
fn handle_connection(mut connection: TcpStream) {

    //create response
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let body = "Hola Fellaw Rustacian!";
    let response = format!("{}{}", status, body);

    println!("Request Recieved");

    // respond to request
    connection.write(response.as_bytes()).unwrap();
    connection.flush().unwrap();
}

fn main() {
    // bind and listen at port 1337
    let host = "127.0.0.1:1337"; 
    let server = TcpListener::bind(host).unwrap();

    // accept and handle connection
    for cxn in server.incoming() {
        let cxn = cxn.unwrap();
        handle_connection(cxn);
    }
}