use std::collections::HashMap;

use crate::http::{
    middlewares::HttpHandler,
    request::{Connection, Error, Response, StatusCode},
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct HandleHiRequest {
    pub msg: &'static str,
    pub handle_url: String,
}

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
