use std::error::Error;

use fivim_rs_utils::json_toml as xu_json_toml;
use fivim_rs_utils::logger as xu_logger;
use fivim_rs_utils::progress as xu_progress;

// pub fn empty(req: RequestData) -> ResponseData {
//     let mut res = ResponseData::new_from_req(&req);
//     res.res_str = req.req_json;
//     return res;
// }

pub fn log(level: &str, content: &str) {
    let level_string = level;
    let content_str = content;

    match level_string {
        "ERROR" => xu_logger::log_error(content_str),
        "INFO" => xu_logger::log_info(content_str),
        "DEBUG" => xu_logger::log_debug(content_str),
        &_ => (),
    }
}

pub fn log_info(content: &str) {
    log("INFO", &content);
}
pub fn log_error(content: &str) {
    log("ERROR", &content);
}
pub fn log_debug(content: &str) {
    log("DEBUG", &content);
}

pub fn get_progress(progress_name: &str) -> xu_progress::Status {
    xu_progress::get(&progress_name)
}

pub fn json_to_toml(json_str: &str) -> Result<String, Box<dyn Error>> {
    xu_json_toml::json_to_toml(json_str)
}

pub fn toml_to_json(toml_str: &str) -> Result<String, Box<dyn Error>> {
    xu_json_toml::toml_to_json(toml_str)
}
