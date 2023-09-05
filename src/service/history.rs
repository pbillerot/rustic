use actix_session::Session;



pub struct History;

impl History {
    pub fn add(path: &str, session: Session) {
        if let Some(mut history) = session.get::<Vec<String>>("history").unwrap() {
            if path == "/" || path.starts_with("/app") {
                session.remove("history");
            } else {
                history.push(path.to_string());
                session.insert("history", history).unwrap();
            }

        } else {
            let mut history: Vec<String> = Vec::new();
            history.push(path.to_string());
            session.insert("history", history).unwrap();
        }
    }
    pub fn get_route_on_err(session: Session) -> String {
        if let Some(mut history) = session.get::<Vec<String>>("history").unwrap() {
            if let Some(path1) = history.pop() {
                session.insert("history", history).unwrap();
                path1
            } else {
                "/".to_string()
            }
        } else {
            "/".to_string()
        }
    }
    pub fn get_route_on_ok(session: Session) -> String {
        if let Some(mut history) = session.get::<Vec<String>>("history").unwrap() {
            // boucle jusqu'à trouver /view ou /form ou /dashboard
            loop {
                if let Some(path1) = history.pop() {
                    session.insert("history", history).unwrap();
                    path1
                } else {
                    "/".to_string()
                }

            }
        } else {
            "/".to_string()
        }
    }
}
