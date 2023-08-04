use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Portail {
    #[serde(default = "default_str")]
    pub title: String,
    #[serde(default = "default_str")]
    pub info: String,
    #[serde(default = "default_str")]
    pub icon_file: String,
    #[serde(default = "default_vec")]
    pub applications: Vec<String>
}
#[allow(dead_code)]
impl Portail {
    pub fn new() -> Portail {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");
        let dico_path = std::env::var("DICO_PATH")
            .expect("Unable to read DICO_PATH env var");
        let path = format!("{}/portail.yaml", &dico_path);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub app_id: String,
    #[serde(default = "default_str")]
    pub title: String,
    #[serde(default = "default_str")]
    pub image: String,
    #[serde(default = "default_str")]
    pub icon_name: String,
    #[serde(default = "default_str")]
    pub group: String,
    #[serde(default = "default_map")]
    pub parameters: HashMap<String, String>,
    #[serde(default = "default_table_view")]
    pub menu: Vec<TableView>,
    #[serde(default = "default_bool")]
    pub shareable: bool,
    #[serde(default = "default_str")]
    pub tasks_table_name: String,
    #[serde(default = "default_str")]
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
    #[serde(default = "default_str")]
    pub table_id: String,
    #[serde(default = "default_str")]
    pub view_id: String,
    #[serde(default = "default_bool")]
    pub in_footer: bool,
}

fn default_str() -> String {
    "".to_string()
}
fn default_map() -> HashMap<String, String> {
    HashMap::new()
}
fn default_bool() -> bool {
    false
}
#[allow(dead_code)]
fn default_vec() -> Vec<String> {
    Vec::new()
}
fn default_table_view() -> Vec<TableView> {
    Vec::new()
}
