use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;
use std::fmt;

use crate::lexicer::lex_utils;
use crate::lexicer::lex_table;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub appid: String,
    #[serde(default = "String::new")]
    pub title: String,
    #[serde(default = "String::new")]
    pub image: String,
    #[serde(default = "String::new")]
    pub icon_name: String,
    #[serde(default = "String::new")]
    pub group: String,
    #[serde(default = "HashMap::new")]
    pub parameters: HashMap<String, String>,
    #[serde(default = "String::new")]
    pub limit_sql: String,
    #[serde(default = "Vec::new")]
    pub menu: Vec<TableView>,
    #[serde(default = "lex_utils::default_bool")]
    pub shareable: bool,
    #[serde(default = "String::new")]
    pub tasks_table_name: String,
    #[serde(default = "String::new")]
    pub wiki: String,
    // Données calculées
    #[serde(default = "String::new")]
    pub in_footer: String, // nbre d'onglets de view dans le footer
    #[serde(default = "HashMap::new")]
    pub tables: HashMap<String, lex_table::Table>,

}

#[allow(dead_code)]
impl Application {
    pub fn load(appid: &str) -> Result<Application, String> {
        let lexic_path = std::env::var("LEXIC_PATH")
            .map_err(|e| format!("Unable to read LEXIC_PATH env var {:?}", e))?;
        let path = format!("{}/{}/config/application.yaml", &lexic_path, appid);
        log::info!("Load de {}", path);
        let f = std::fs::File::open(&path)
            .map_err(|e| format!("Could not open file {:?}", e))?;
        let mut application: Application  = serde_yaml::from_reader(f)
            .map_err(|e| format!("Could not read values {:?}", e))?;
        let mut ifooter = 0;
        for menu in &application.menu {
            let table: &lex_table::Table = &lex_table::Table::load(&appid, &menu.tableid)?;
            application.tables.insert(menu.tableid.to_string(), table.clone());
            if menu.in_footer {
                ifooter = ifooter + 1;
            }
        }
        // Nbre d'onglets dans le footer en english pour semantic
        application.in_footer = match ifooter {
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            _ => "".to_string(),
        };
        Ok(application)
    }
}
impl fmt::Display for Application {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}:{}", self.appid, self.title)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableView {
    #[serde(default = "String::new")]
    pub tableid: String,
    #[serde(default = "String::new")]
    pub viewid: String,
    #[serde(default = "lex_utils::default_bool")]
    pub in_footer: bool,
}
