use http::{
    middlewares::HttpHandler,
    request::{self, Connection, Error, Request, Response, StatusCode},
};
use matchit::Router;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
mod http;
use anyhow::{anyhow, Result};
use async_trait::async_trait;

#[derive(Clone)]
struct HandleHiRequest {
    pub msg: &'static str,
    pub handle_url: String,
}

// This is supposed to act as a handler for some spcific route.
// TODO: Maybe try to use macros?
#[async_trait]
impl HttpHandler for HandleHiRequest {
    // Is there a way to parse request here which matches with the handle_url?
    async fn handle_connection(&self, conn: &mut Connection) -> Result<(), Error> {
        let response = Response {
            status: StatusCode::ok(),
            headers: HashMap::new(),
            body: self.msg.to_owned(),
        };
        conn.respond(response).await?;
        Ok(())
    }
    fn get_handle_url(&self) -> String {
        self.handle_url.clone()
    }
}

#[derive(Clone)]
struct HandleAboutRequest {
    pub msg: &'static str,
    pub handle_url: String,
}

// This is supposed to act as a handler for some spcific route.
// TODO: Maybe try to use macros?
#[async_trait]
impl HttpHandler for HandleAboutRequest {
    // Is there a way to parse request here which matches with the handle_url?
    async fn handle_connection(&self, conn: &mut Connection) -> Result<(), Error> {
        let response = Response {
            status: StatusCode::ok(),
            headers: HashMap::new(),
            body: self.msg.to_owned(),
        };
        conn.respond(response).await?;
        Ok(())
    }
    fn get_handle_url(&self) -> String {
        self.handle_url.clone()
    }
}

impl From<request::Error> for anyhow::Error {
    fn from(value: request::Error) -> Self {
        anyhow!("Error: {:?}", value)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // List of all path handlers
    let hi_handler: Box<dyn HttpHandler> = Box::new(HandleHiRequest {
        msg: "Vivek is cool IG",
        handle_url: "/user".to_string(),
    });
    let about_handle: Box<dyn HttpHandler> = Box::new(HandleAboutRequest {
        msg: "This is about page",
        handle_url: "/about".to_owned(),
    });

    let handlers: Vec<&Box<dyn HttpHandler>> = vec![&hi_handler, &about_handle];
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
