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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url() {
        let url = "https://vivek.ink/:user/address/:key/:ip".to_owned();
        let request_url = "https://vivek.ink/vivekascoder/address/jumba/127.0.0.1/".to_owned();
        let mut path_param = PathParams::new(url);

        let map = path_param.parse(request_url);
        println!("{:?}", map);
        assert!(false);
    }
}
