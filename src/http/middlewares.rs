use async_trait::async_trait;

use super::request::Connection;
use super::request::Error;

#[async_trait]
pub trait HttpHandler {
    async fn handle_connection(&self, conn: &mut Connection) -> Result<(), Error>;
    fn get_handle_url(&self) -> String;
}
