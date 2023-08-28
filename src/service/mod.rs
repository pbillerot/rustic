use serde::{Deserialize, Serialize};

pub mod srv_session;

#[allow(dead_code)]
pub const MESSAGE_LEVEL_INFO: i32 = 0;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_WARNING: i32 = 1;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_ERROR: i32 = 2;
#[allow(dead_code)]
pub const MESSAGE_LEVEL_DEBUG: i32 = 3;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String, // le message
    pub level: i32, // info,debug,error,warning
}
impl Message {
    pub fn new(content: &str, level: i32) -> Message {
        Message { content: content.into(), level }
    }
}
