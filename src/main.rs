use http::request::{self, Connection, Error, Request, Response, StatusCode};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
mod http;

// fn handle_connection(mut stream: TcpStream) {}

#[tokio::main]
async fn main() -> Result<(), request::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        // let request = handle_tcp_strema(&mut socket).await?;
        println!("S");
        let mut con = Connection::new(socket).await?;
        println!("Conn");
        let response = Response {
            status: StatusCode::ok(),
            headers: HashMap::new(),
            body: "Hello World".to_string(),
        };
        con.respond(response).await?;
    }
}
