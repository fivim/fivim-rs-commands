use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteFileRes {
    pub success: bool,
    #[serde(rename = "errMsg")]
    pub err_msg: String,
}

impl WriteFileRes {
    pub fn new() -> WriteFileRes {
        WriteFileRes {
            success: false,
            err_msg: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "accessedTime")]
    pub accessed_time: String,
    #[serde(rename = "modifiedTime")]
    pub modified_time: String,
    #[serde(rename = "errMsg")]
    pub err_msg: String,
}
