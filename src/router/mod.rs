use actix_web::{HttpMessage, HttpRequest};
///
/// Gestion des routes
///
use serde::{Deserialize, Serialize};

mod route_lexic;
pub use self::route_lexic::lexicall; // curl http://0.0.0.0:8080/lexic/action

mod route_portail;
pub use self::route_portail::portail; // curl http://0.0.0.0:8080/

mod route_application;
pub use self::route_application::application; // curl http://0.0.0.0:8080/app/appid

mod route_view;
pub use self::route_view::view; // curl http://0.0.0.0:8080/list/appid/tableid/viewid

mod route_form;
pub use self::route_form::form; // curl http://0.0.0.0:8080//view/appid/tableid/viewid/formid/id

mod route_edit;
pub use self::route_edit::edit; // curl http://0.0.0.0:8080//edit/appid/tableid/viewid/formid/id

mod route_edit_post;
pub use self::route_edit_post::edit_post; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid/id

mod route_add;
pub use self::route_add::add; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid

mod route_add_post;
pub use self::route_add_post::add_post; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid

#[allow(dead_code)]
pub const MESSAGE_LEVEL_INFO: i32 = 0;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_WARNING: i32 = 1;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_ERROR: i32 = 2;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_DEBUG: i32 = 3;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String, // le message
    pub level: i32,      // info,debug,error,warning
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    pub items: Vec<Message>,
}
#[allow(dead_code)]
impl Messages {
    pub fn new() -> Messages {
        Messages { items: Vec::new() }
    }
    pub fn info(&mut self, message: &str) {
        self.items.push(Message {
            content: message.into(),
            level: MESSAGE_LEVEL_INFO,
        });
    }
    pub fn debug(&mut self, message: &str) {
        self.items.push(Message {
            content: message.to_string(),
            level: MESSAGE_LEVEL_DEBUG,
        });
    }
    pub fn error(&mut self, message: &str) {
        self.items.push(Message {
            content: message.to_string(),
            level: MESSAGE_LEVEL_ERROR,
        });
    }
    pub fn warning(&mut self, message: &str) {
        self.items.push(Message {
            content: message.to_string(),
            level: MESSAGE_LEVEL_WARNING,
        });
    }
    pub fn save_in_request(&self, req: &HttpRequest) {
        println!("---->> save_in_request [{:?}]", self);
        req.extensions_mut().insert::<Messages>(self.clone());
    }
    pub fn get_from_request(req: &HttpRequest) -> Messages {
        match req.extensions_mut().get::<Messages>() {
            Some(m) => {
                println!("<<---- get_from_request [{:?}]", &m);
                m.clone()
            },
            None => Messages::new()
        }
    }
}
