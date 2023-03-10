use crate::http::{
    middlewares::HttpViewHandler,
    request::{Connection, Error, Response, StatusCode},
};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HandleAboutRequest {
    pub msg: &'static str,
    pub handle_url: String,
}

// This is supposed to act as a handler for some spcific route.
// TODO: Maybe try to use macros?
#[async_trait]
impl HttpViewHandler for HandleAboutRequest {
    // Is there a way to parse request here which matches with the handle_url?
    fn new(msg: &'static str, mut handle_url: String) -> HandleAboutRequest {
        if !handle_url.ends_with('/') {
            handle_url.push('/');
        }
        Self {
            msg: msg,
            handle_url: handle_url,
        }
    }

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
