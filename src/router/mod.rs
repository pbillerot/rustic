///
/// Gestion des routes
///

mod rt_lexic;
pub use self::rt_lexic::lexicall; // curl http://0.0.0.0:8080/lexic/{action}

mod rt_portail;
pub use self::rt_portail::portail; // curl http://0.0.0.0:8080/

mod rt_application;
pub use self::rt_application::application; // curl http://0.0.0.0:8080/app/{appid}

mod rt_list;
pub use self::rt_list::list; // curl http://0.0.0.0:8080/appid/tableid/viewid

