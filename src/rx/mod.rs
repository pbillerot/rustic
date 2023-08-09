/// Gestion des routes
///
/// ACCUEIL
/// /login
/// /logout
/// /about

/// CRUD
/// /list/:app/:table/:view
/// /dashboard/:app/:table/:view
/// /view/:app/:table/:view/:id
/// /add/:app/:table/:view/:form
/// /edit/:app/:table/:view/:form/:id
/// /delete/:app/:table/:view/:id

/// ACTIONS
/// /actionv/:app/:table/:view/:action
/// /actionp/:app/:table/:view/:id
/// /actionf/:app/:table/:view/:form/:id/:action
/// /actione/:app/:table/:view/:form/:id/:action
/// /actionx/:app/:table/:view/:id/:action
/// /ajax/:app/:table/:view/:form/:action

/// VIEW interactions
/// /search/:app/:table/:view
/// /filter/:app/:table/:view
/// /sort/:app/:table/:view

/// PARTAGE de l'APPLICATION
/// /share/:appid/:shareid

/// EDITEUR : EDDY
/// /eddy/document/:key
/// /eddy/log

mod rx_portail;
pub use self::rx_portail::portail; // curl http://0.0.0.0:8080/

mod tweet;
pub use self::tweet::list;
pub use self::tweet::get;
pub use self::tweet::create;
pub use self::tweet::delete;