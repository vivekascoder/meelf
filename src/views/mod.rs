use crate::http::middlewares::HttpViewHandler;

use self::{about::HandleAboutRequest, home::HandleHiRequest};

pub mod about;
pub mod home;

pub fn get_handlers() -> Vec<Box<dyn HttpViewHandler>> {
    let hi_handler: Box<dyn HttpViewHandler> = Box::new(HandleHiRequest::new(
        "Vivek is cool IG",
        "/user".to_string(),
    ));
    let about_handler: Box<dyn HttpViewHandler> = Box::new(HandleAboutRequest::new(
        "This is about page",
        "/about".to_owned(),
    ));

    let handlers: Vec<Box<dyn HttpViewHandler>> = vec![hi_handler, about_handler];
    handlers
}
