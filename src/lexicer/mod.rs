// Modules de gestion du lexique

use std::collections::HashMap;

pub mod lex_lexic;

pub mod lex_portail;
pub mod lex_application;
pub mod lex_table;
pub mod lex_utils;

pub fn macvalue(source: &str, hvalue: &HashMap<String, String>) -> String {
    let mut result: String = source.into();
    for (key, value) in hvalue {
        result = result.replace(format!("[{}]", key).replace("[", "{").replace("]", "}").as_str(), value);
    }

    result
}
pub fn macelement(source: &str, helements: &HashMap<String, lex_table::Element>) -> String {
    let mut result: String = source.into();
    for (key, element) in helements {
        result = result.replace(format!("[{}]", key).replace("[", "{").replace("]", "}").as_str(), &element.value);
    }

    result
}
