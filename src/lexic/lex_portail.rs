use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Portail {
    #[serde(default = "String::new")]
    pub title: String,
    #[serde(default = "String::new")]
    pub info: String,
    #[serde(default = "String::new")]
    pub icon_file: String,
    #[serde(default = "Vec::new")]
    #[serde(rename = "applications")]
    pub appids: Vec<String>,
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
        for k in &self.appids {
            v.push(k.to_string());
        }
        Portail {
            title: self.title.clone(),
            info: self.info.clone(),
            icon_file: self.icon_file.clone(),
            appids: v,
        }
    }
}

