use http::request::{self, Connection};
use tokio::net::TcpListener;
use views::get_handlers;
mod http;
mod views;
use anyhow::{anyhow, Result};

impl From<request::Error> for anyhow::Error {
    fn from(value: request::Error) -> Self {
        anyhow!("Error: {:?}", value)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let mut con = Connection::new(socket).await?;
        if !con.request.uri.ends_with('/') {
            con.request.uri.push('/');
        }

        let handlers = get_handlers();

        for ref handle in handlers {
            if handle.get_handle_url() == con.request.uri {
                handle.handle_connection(&mut con).await?;
            }
        }
    }
}
