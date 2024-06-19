use anyhow::anyhow;
use fivim_rs_utils::web::{
    self as xu_web, request_data, HttpMethod, HttpResponse, ReaponseDataType,
};
use std::{collections::HashMap, fs};

fn get_http_method(method_str: &str) -> HttpMethod {
    let mmm = method_str.to_lowercase();
    let method = match mmm.as_str() {
        "get" => HttpMethod::Get,
        "post" => HttpMethod::Post,
        "head" => HttpMethod::Head,
        "delete" => HttpMethod::Delete,
        "put" => HttpMethod::Put,
        &_ => HttpMethod::None,
    };

    method
}

fn get_resp_data_type(data_type_str: &str) -> ReaponseDataType {
    let mmm = data_type_str.to_lowercase();
    let data_type = match mmm.as_str() {
        "text" => ReaponseDataType::Text,
        "base64" => ReaponseDataType::Base64,
        &_ => ReaponseDataType::None,
    };

    data_type
}

pub async fn download_file(
    method: &str,
    url: &str,
    header_map: &HashMap<String, String>,
    params_map: &HashMap<String, String>,
    file_path: &str,
    is_large_file: bool,
    progress_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let _ = fs::remove_file(&file_path); // Try to delete this file fist.

    let method = get_http_method(&method);
    if method == xu_web::HttpMethod::None {
        return Err(anyhow!("HTTP method is incorrect!").into());
    }

    if is_large_file {
        let count = xu_web::downlaod_file_large(
            method,
            &url,
            &file_path,
            &header_map,
            &params_map,
            &progress_name,
        )
        .await?;
        Ok(count > 0)
    } else {
        let _ = xu_web::downlaod_file(method, &url, &file_path, &header_map, &params_map).await?;
        Ok(true)
    }
}

pub async fn http_request(
    method: &str,
    url: &str,
    header_map: &HashMap<String, String>,
    params_map: &HashMap<String, String>,
    body: &str,
    resp_data_type: &str,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let method = get_http_method(&method);
    if method == HttpMethod::None {
        return Err(anyhow!("HTTP method is incorrect!").into());
    }

    let data_type = get_resp_data_type(resp_data_type);
    if data_type == ReaponseDataType::None {
        return Err(anyhow!("resp_data_type is incorrect!").into());
    }

    let ccc = request_data(
        method,
        &url,
        &header_map,
        &params_map,
        body.to_owned(),
        data_type,
    )
    .await?;
    return Ok(ccc);
}
