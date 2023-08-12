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
    pub applications: Vec<String>
}
#[allow(dead_code)]
impl Portail {
    pub fn new() -> Portail {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let dico_path = std::env::var("DICO_PATH")
            .expect("Unable to read DICO_PATH env var");
        let path = format!("{}/portail.yaml", &dico_path);
        log::info!("Load de {}", path);
        let f = std::fs::File::open(&path).expect("Could not open file.");
        let myport: Portail  = serde_yaml::from_reader(f).expect("Could not read values.");

        myport
    }
}
impl Clone for Portail {
    fn clone(&self) -> Portail {
        Portail {
            title: self.title.clone(),
            info: self.info.clone(),
            icon_file: self.icon_file.clone(),
            applications: self.applications.clone(),
        }

    }
}

