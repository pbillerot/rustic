use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;

use crate::lexic::lex_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub app_id: String,
    #[serde(default = "lex_utils::default_str")]
    pub title: String,
    #[serde(default = "lex_utils::default_str")]
    pub image: String,
    #[serde(default = "lex_utils::default_str")]
    pub icon_name: String,
    #[serde(default = "lex_utils::default_str")]
    pub group: String,
    #[serde(default = "lex_utils::default_map")]
    pub parameters: HashMap<String, String>,
    #[serde(default = "TableView::default")]
    pub menu: Vec<TableView>,
    #[serde(default = "lex_utils::default_bool")]
    pub shareable: bool,
    #[serde(default = "lex_utils::default_str")]
    pub tasks_table_name: String,
    #[serde(default = "lex_utils::default_str")]
    pub wiki: String,
}
#[allow(dead_code)]
impl Application {
    pub fn new(appid: String) -> Application {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let dico_path = std::env::var("DICO_PATH")
            .expect("Unable to read DICO_PATH env var");
        let path = format!("{}/{}/config/application.yaml", &dico_path, &appid);
        let f = std::fs::File::open(&path).expect("Could not open file.");
        let application  = serde_yaml::from_reader(f).expect("Could not read values.");

        application
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableView {
    #[serde(default = "lex_utils::default_str")]
    pub table_id: String,
    #[serde(default = "lex_utils::default_str")]
    pub view_id: String,
    #[serde(default = "lex_utils::default_bool")]
    pub in_footer: bool,
}
impl TableView {
    pub fn default() -> Vec<TableView> {
        Vec::new()
    }
}
