use http::{
    middlewares::HttpViewHandler,
    request::{self, Connection},
};
use tokio::net::TcpListener;
use views::{about::HandleAboutRequest, home::HandleHiRequest};
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
    // List of all path handlers
    let hi_handler: Box<dyn HttpViewHandler> = Box::new(HandleHiRequest::new(
        "Vivek is cool IG",
        "/user".to_string(),
    ));
    let about_handle: Box<dyn HttpViewHandler> = Box::new(HandleAboutRequest::new(
        "This is about page",
        "/about".to_owned(),
    ));

    let handlers: Vec<&Box<dyn HttpViewHandler>> = vec![&hi_handler, &about_handle];
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let mut con = Connection::new(socket).await?;
        if !con.request.uri.ends_with('/') {
            con.request.uri.push('/');
        }

        for ref handle in handlers.clone() {
            if handle.get_handle_url() == con.request.uri {
                handle.handle_connection(&mut con).await?;
            }
        }
    }
}
