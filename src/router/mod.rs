///
/// Gestion des routes
///

mod route_lexic;
pub use self::route_lexic::lexicall; // curl http://0.0.0.0:8080/lexic/action

mod route_portail;
pub use self::route_portail::portail; // curl http://0.0.0.0:8080/

mod route_application;
pub use self::route_application::application; // curl http://0.0.0.0:8080/app/appid

mod route_list;
pub use self::route_list::list; // curl http://0.0.0.0:8080/list/appid/tableid/viewid

mod route_view;
pub use self::route_view::view; // curl http://0.0.0.0:8080//view/appid/tableid/viewid/id

