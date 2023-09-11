///
/// Gestion des routes
///

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

mod route_delete_post;
pub use self::route_delete_post::delete_post; // curl http://0.0.0.0:8080//delete/appid/tableid/viewid/formid/id

