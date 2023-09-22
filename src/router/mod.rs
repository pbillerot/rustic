///
/// Gestion des routes
///


// /
mod route_portail;
pub use self::route_portail::portail;

// /app/appid
mod route_application;
pub use self::route_application::application;

// /list/appid/tableid/viewid
mod route_view;
pub use self::route_view::view;

// /view/appid/tableid/viewid/formid/id
mod route_form;
pub use self::route_form::form;

// /edit/appid/tableid/viewid/formid/id
mod route_edit;
pub use self::route_edit::edit;

// /update/appid/tableid/viewid/formid/id
mod route_edit_post;
pub use self::route_edit_post::edit_post; //

 // add/appid/tableid/viewid/formid
mod route_add;
pub use self::route_add::add;

 // insert/appid/tableid/viewid/formid
mod route_add_post;
pub use self::route_add_post::add_post;

// delete/appid/tableid/viewid/formid/id
mod route_delete_post;
pub use self::route_delete_post::delete_post;

// /sort/appid/tableid/viewid
mod route_sort;
pub use self::route_sort::sort;

// /filter/appid/tableid/viewid
mod route_filter;
pub use self::route_filter::filter;



// /lexic/action
mod route_lexic;
pub use self::route_lexic::lexicall;
