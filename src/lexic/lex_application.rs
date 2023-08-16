use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;

use crate::lexic::lex_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub appid: String,
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
    pub fn load(appid: &str) -> Application {
        // dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let lexic_path = std::env::var("LEXIC_PATH")
            .expect("Unable to read LEXIC_PATH env var");
        let path = format!("{}/{}/config/application.yaml", &lexic_path, appid);
        log::info!("Load de {}", path);
        let f = std::fs::File::open(&path).expect("Could not open file.");
        let application  = serde_yaml::from_reader(f).expect("Could not read values.");

        application
    }
}
impl Clone for Application {
    fn clone(&self) -> Application {
        Application {
            appid: self.appid.clone(),
            title: self.title.clone(),
            image: self.image.clone(),
            icon_name: self.icon_name.clone(),
            group: self.group.clone(),
            parameters: self.parameters.clone(),
            menu: self.menu.clone(),
            shareable: self.shareable.clone(),
            tasks_table_name: self.tasks_table_name.clone(),
            wiki: self.wiki.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableView {
    #[serde(default = "lex_utils::default_str")]
    pub tableid: String,
    #[serde(default = "lex_utils::default_str")]
    pub viewid: String,
    #[serde(default = "lex_utils::default_bool")]
    pub in_footer: bool,
}
impl TableView {
    pub fn default() -> Vec<TableView> {
        Vec::new()
    }
}
impl Clone for TableView {
    fn clone(&self) -> TableView {
        TableView {
            tableid: self.tableid.clone(),
            viewid: self.viewid.clone(),
            in_footer: self.in_footer.clone(),
        }
    }
}