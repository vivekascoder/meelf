use http::request::{Error, Request};
use std::io;
use std::io::BufReader;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
mod http;

// fn handle_connection(mut stream: TcpStream) {}

async fn handle_tcp_strema(socket: &mut TcpStream) -> Result<Request, Error> {
    let request = Request::new(socket).await?;
    println!("Request: {:?}", request);
    Ok(request)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        handle_tcp_strema(&mut socket).await;
    }
    Ok(())
}
