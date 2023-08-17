use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use crate::lexic::lex_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Portail {
    #[serde(default = "lex_utils::default_str")]
    pub title: String,
    #[serde(default = "lex_utils::default_str")]
    pub info: String,
    #[serde(default = "lex_utils::default_str")]
    pub icon_file: String,
    #[serde(default = "Vec::new")]
    pub applications: Vec<String>,
}
#[allow(dead_code)]
impl Portail {
    pub fn load() -> Result<Portail, String> {
        let lexic_path = std::env::var("LEXIC_PATH")
            .map_err(|e| format!("Unable to read LEXIC_PATH env var {:?}", e))?;
        let path = format!("{}/portail.yaml", &lexic_path);
        log::info!("Load de {}", path);
        let f = std::fs::File::open(&path)
            .map_err(|e| format!("Could not open file {:?}", e))?;
        let portail: Portail  = serde_yaml::from_reader(f)
            .map_err(|e| format!("Could not read values {:?}", e))?;

        Ok(portail)

    }
}
impl Clone for Portail {
    fn clone(&self) -> Portail {
        let mut v = Vec::new();
        for k in &self.applications {
            v.push(k.to_string());
        }
        Portail {
            title: self.title.clone(),
            info: self.info.clone(),
            icon_file: self.icon_file.clone(),
            applications: v,
        }
    }
}

