use super::path_params::PathParams;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Debug)]
pub enum Error {
    ParsingError,
    Utf8Error(std::string::FromUtf8Error),
    IOError(std::io::Error),
    NotFoundError,
}

impl From<std::io::Error> for Error {
    fn from(internal_err: std::io::Error) -> Self {
        Error::IOError(internal_err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(internal_err: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(internal_err)
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "OPTION" => Method::OPTION,
            "DELETE" => Method::DELETE,
            _ => panic!("Error"),
        }
    }
}

#[derive(Debug)]
pub enum Version {
    HTTP1_1,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::HTTP1_1 => f.write_str("HTTP/1.1"),
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        if s == "HTTP/1.1" {
            Version::HTTP1_1
        } else {
            Version::HTTP1_1
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}

impl Request {
    pub async fn new(reader: &mut tokio::net::TcpStream) -> Result<Request, Error> {
        let mut first_line: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut buffer: Vec<u8> = Vec::new();

        loop {
            let b = reader.read_u8().await?;
            buffer.push(b);

            // Line's over
            if b as char == '\n' {
                if first_line.is_empty() {
                    first_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    // Clear the buffer to read the next line now.
                    buffer.clear()
                } else {
                    // Break
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break;
                    }

                    // Parse headers
                    let header_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    buffer.clear();

                    let mut iter = header_line.split(" ");
                    let key = match iter.next() {
                        Some(key) => key,
                        None => return Err(Error::ParsingError),
                    };
                    let value = match iter.next() {
                        Some(value) => {
                            if value.starts_with(" ") {
                                String::from(value)[1..].to_string()
                            } else {
                                value.to_string()
                            }
                        }
                        None => return Err(Error::ParsingError),
                    };
                    headers.insert(key.to_string(), value);
                }
            }
        }
        // Let's parse method, version and query parameters.
        let mut first_line_iter = first_line.split(" ");
        let method: Method = first_line_iter.next().unwrap().into();
        let uri_str = first_line_iter.next().unwrap().to_string();

        let mut uri_iter = uri_str.split("?");

        // The first thing is the uri
        let uri = match uri_iter.next() {
            Some(u) => u.to_string(),
            None => return Err(Error::ParsingError),
        };

        let mut query_params: HashMap<String, String> = HashMap::new();
        match uri_iter.next() {
            Some(q) => {
                for kv in q.split("&") {
                    let mut iter = kv.split("=");
                    let key = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    let value = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    query_params.insert(key.to_string(), value.to_string());
                }
            }
            None => (),
        }
        let version: Version = first_line_iter.next().unwrap().into();
        Ok(Self {
            method: method,
            uri: uri,
            version: version,
            headers,
            query_params: query_params,
            path_params: HashMap::new(),
        })
    }
}

#[derive(Debug)]
pub struct Connection {
    pub request: Request,
    pub socket: TcpStream,
}

pub struct StatusCode {
    pub code: usize,
    pub msg: &'static str,
}

impl StatusCode {
    pub fn ok() -> Self {
        StatusCode {
            code: 200,
            msg: "OK",
        }
    }
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Connection {
    pub async fn new(mut socket: TcpStream) -> Result<Connection, Error> {
        let req = Request::new(&mut socket).await?;
        Ok(Connection {
            request: req,
            socket: socket,
        })
    }

    /// To send response back
    pub async fn respond(&mut self, res: Response) -> Result<(), Error> {
        self.socket
            .write_all(
                format!(
                    "{} {} {}\r\n",
                    self.request.version, res.status.code, res.status.msg
                )
                .as_bytes(),
            )
            .await?;

        for (k, v) in res.headers.iter() {
            self.socket
                .write_all(format!("{}: {}\r\n", k, v).as_bytes())
                .await?;
        }

        self.socket.write_all(b"\r\n").await?;

        // Finally add reponse body.
        if res.body.len() > 0 {
            println!("Body {}", res.body);
            self.socket
                .write_all(format!("{}\r\n", res.body).as_bytes())
                .await?;
        }
        self.socket.write_all(b"\r\n").await?;
        println!("responded");
        Ok(())
    }
}
