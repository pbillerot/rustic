// GESTION DE L'URL DE RETOUR d'un formulaire

use actix_session::Session;
use actix_web::HttpRequest;

pub fn compute_back(req: &HttpRequest, session: &Session) {
  let b1 = match session.get::<String>("back1") {
      Ok(Some(b)) => b,
      Ok(None) => String::new(),
      Err(_) => String::new(),
  };
  let b2 = match session.get::<String>("back2") {
      Ok(Some(b)) => b,
      Ok(None) => String::new(),
      Err(_) => String::new(),
  };
  let b3 = match session.get::<String>("back3") {
      Ok(Some(b)) => b,
      Ok(None) => String::new(),
      Err(_) => String::new(),
  };

  if req.path().contains("/view/") {
    session.insert("back", req.path().to_string()).unwrap();
    session.insert("back1", req.path().to_string()).unwrap();
    session.remove("back2");
    session.remove("back3");
  } else if req.path().contains("/form/") {
      if !b3.is_empty() {
        if b3 == req.path() {
          session.insert("back", b2).unwrap();
        } else {
          if b2 == req.path() {
            session.remove("back3");
            session.insert("back", b1).unwrap();
          } else {
            session.insert("back", b2).unwrap();
          }
        }
      } else {
        // b3 empty
        if !b2.is_empty() {
          if b2 == req.path() {
            session.insert("back", b1).unwrap();
          } else {
            session.insert("back3", req.path()).unwrap();
            session.insert("back", b2).unwrap();
          }
        } else {
          // b2 empty
          session.insert("back2", req.path()).unwrap();
          session.insert("back", b1).unwrap();
        }
      }
  } else {
    if !b3.is_empty() {
      session.insert("back", b3).unwrap();
    } else if !b2.is_empty() {
      session.insert("back", b2).unwrap();
    } else if !b1.is_empty() {
      session.insert("back", b1).unwrap();
    } else {
      session.insert("back", "/").unwrap();
    }
  }
}

pub fn get_back(session: &Session) -> String {
  match session.get::<String>("back") {
    Ok(Some(b)) => return b,
    Ok(None) => return String::new(),
    Err(_) => return String::new(),
  };
}
