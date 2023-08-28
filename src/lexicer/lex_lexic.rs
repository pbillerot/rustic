use std::collections::HashMap;
use crate::lexicer::lex_portail;
use crate::lexicer::lex_application;
use crate::lexicer::lex_utils;
pub struct Lexic {
    pub portail: lex_portail::Portail,
    pub applications: HashMap<String, lex_application::Application>
}
impl Lexic {
    pub fn load() -> Result<Lexic, String> {
        let portail = lex_portail::Portail::load()?;
        let mut map = HashMap::new();
        for appid in &portail.appids {
            let app = lex_application::Application::load(appid.as_str())?;
            map.insert(appid.to_string(), app);
        }
        Ok(Lexic {
            portail: portail,
            applications : map,
        })
    }
}

impl Clone for Lexic {
    fn clone(&self) -> Lexic {
        // let mut map = HashMap::new();
        // for (key, val) in &self.applications {
        //     map.insert(key.to_string(), val.clone());
        // }
        Lexic {
            portail: self.portail.clone(),
            applications: lex_utils::do_clone_map(&self.applications),
        }
    }
}
