// GESTION DE L'URL DE RETOUR d'un formulaire

use actix_session::Session;
use actix_web::HttpRequest;

pub fn get_back(req: &HttpRequest, session: &Session) -> String {
  if req.path().contains("/view/") {
    session.insert("back1", req.path().to_string()).unwrap(); // /view
    session.remove("back2"); // /form
    session.remove("back3"); //     view /form
    return req.path().to_string();
  } else if req.path().contains("/form/") {
    match session.get::<String>("back3") {
      Ok(Some(b3)) => return b3,
      Ok(None) => { // pas de b3
        match session.get::<String>("back2") {
          Ok(Some(b2)) => {
            if b2 != req.path() {
              // appel d'un nouveau formulaire de niveau 3
              session.insert("back3", req.path().to_string()).unwrap();
              return req.path().to_string()
            } else {
              return b2
            };
          },
          Ok(None) => { // pas de b2
            // appel d'un nouveau formulaire de niveau 2
            session.insert("back2", req.path().to_string()).unwrap();
            return req.path().to_string()
          },
          Err(_) => return req.path().to_string()
        };
      },
      Err(_) => return req.path().to_string()
    };
  } else {
    match session.get::<String>("back3") {
      Ok(Some(b3)) => return b3,
      Ok(None) => {
        match session.get::<String>("back2") {
          Ok(Some(b2)) => return b2,
          Ok(None) => {
            match session.get::<String>("back1") {
              Ok(Some(b1)) => return b1,
              Ok(None) => return req.path().to_string(),
              Err(_) => return req.path().to_string()
            };
          },
          Err(_) => return req.path().to_string()
        };
      },
      Err(_) => return req.path().to_string()
    };
  };
}
