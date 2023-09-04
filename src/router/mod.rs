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

mod route_edit_update;
pub use self::route_edit_update::edit_update; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid/id

mod route_add;
pub use self::route_add::add; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid

mod route_add_insert;
pub use self::route_add_insert::add_insert; // curl http://0.0.0.0:8080//update/appid/tableid/viewid/formid

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
    pub level: i32, // info,debug,error,warning
}
impl Message {
    pub fn new(content: &str, level: i32) -> Message {
        Message { content: content.into(), level }
    }
}
