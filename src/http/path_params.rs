// Simple URL parser.
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct PathParams {
    path: String,
}

#[derive(Debug)]
pub enum UrlParseError {
    ValueParseError,
}

impl Display for UrlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("UrlParseError: {:?}", self);
        Ok(())
    }
}

impl Error for UrlParseError {}

impl PathParams {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }

    pub fn parse(&mut self, url: String) -> Result<HashMap<String, String>, UrlParseError> {
        let mut map: HashMap<String, String> = HashMap::new();
        let mut path_iter = self.path.split('/');
        let mut url_iter = url.split('/');

        loop {
            match path_iter.next() {
                None => {
                    url_iter.next();
                    break;
                }
                Some(key) => {
                    let value = match url_iter.next() {
                        Some(v) => v,
                        None => return Err(UrlParseError::ValueParseError),
                    };
                    if key.starts_with(':') {
                        map.insert(key[1..].to_owned(), value.to_owned());
                    }
                }
            }
        }

        Ok(map)
    }

    /// Return whether the `url` is equal to `path`.
    pub fn is_eq(&self, url: String) -> bool {
        // Use two pointer, start from 0.
        // match exact string in both string until tou encounter `:`.
        // in that case match upto `/` in both the string,
        // Then again match exact string
        let path_str = self.path.as_bytes();
        let url_str = url.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < path_str.len() {
            if path_str[i] as char == ':' {
                // Increment the pointer i upto next `/`.
                // and Increment the pointer j upto next `/`.
                println!("i: {:?}", i);
                while path_str[i] as char != '/' {
                    println!("i: {:?}", i);
                    i += 1;
                    if i == path_str.len() {
                        break;
                    }
                }
                while url_str[j] as char != '/' {
                    j += 1;
                    if j == url_str.len() {
                        break;
                    }
                }
            } else if path_str[i] as char != url_str[j] as char {
                println!("i: {:?}, j: {:?}", i, j);
                return false;
            }

            // Increment the pointers.
            i += 1;
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url() {
        let url = "/:user/address/:key/:ip/".to_owned();
        let request_url = "/vivekascoder/address/jumba/127.0.0.1/".to_owned();
        let mut path_param = PathParams::new(url);

        let map = path_param.parse(request_url.clone());
        println!("{:?}", map);
        println!("is_eq: {:?}", path_param.is_eq(request_url));
        assert!(false);
    }
}
