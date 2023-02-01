use std::{collections::HashMap, hash::Hash};

use tokio::io::AsyncReadExt;

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
                    println!("First line: {:?}", first_line);
                    // Clear the buffer to read the next line now.
                    buffer.clear()
                } else {
                    // Break
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break;
                    }

                    // Parse headers
                    let header_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    println!("Header line: {:?}", header_line);
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
        Ok(Self {
            method: Method::DELETE,
            uri: "sfsf".to_string(),
            version: Version::HTTP1_1,
            headers,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
        })
    }
}
