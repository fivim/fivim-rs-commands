use crate::types;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, FixedOffset};
use enassi_rs_utils::{fs as xu_fs, fs_dir::FileNode, zip as xu_zip};
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

pub fn copy_file(file_path: &str, target_file_path: &str) -> Result<(), Box<dyn Error>> {
    fs::copy(&file_path, &target_file_path)?;
    return Ok(());
}

pub fn add_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    File::create(&file_path)?;
    return Ok(());
}

pub fn delete_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(&file_path)?;
    return Ok(());
}

pub fn exist_file(file_path: &str) -> bool {
    return xu_fs::exists(&file_path);
}

pub fn read_file_to_string(file_path: &str) -> Result<String, Box<dyn Error>> {
    return xu_fs::read_to_string(&file_path);
}

pub fn write_base64_into_file(file_path: &str, file_content: &str) -> Result<(), Box<dyn Error>> {
    return xu_fs::write_base64_str(&file_path, &file_content);
}

pub fn write_string_into_file(file_path: &str, file_content: &str) -> Result<(), Box<dyn Error>> {
    return xu_fs::write_str(&file_path, &file_content);
}

pub fn write_bytes_into_file(
    file_path: &str,
    file_content: &Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    return xu_fs::write_bytes(&file_path, &file_content);
}

pub fn read_file_to_base64_string(file_path: &str) -> Result<String, Box<dyn Error>> {
    let vec_cnt = xu_fs::read_to_bytes(&file_path, false)?;
    let ened = STANDARD.encode(vec_cnt);
    return Ok(ened);
}

pub fn add_dir(dir_path: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir(&dir_path)?;
    return Ok(());
}

pub fn delete_dir(dir_path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all(&dir_path)?;
    return Ok(());
}

pub fn get_dir_size(dir_path: &str) -> Result<u64, Box<dyn Error>> {
    return xu_fs::get_dir_size(&dir_path);
}

pub fn list_dir_children(dir_path: &str) -> Result<Vec<xu_fs::DirChildren>, Box<dyn Error>> {
    let list = xu_fs::get_children_list(&dir_path)?;
    return Ok(list);
}

pub fn rename(path_old: &str, path_new: &str, is_dir: bool) -> Result<(), Box<dyn Error>> {
    if is_dir {
        let _ = fs::remove_dir_all(&path_new);
        let _ = fs::create_dir(&path_new);
        let _ = xu_fs::check_or_create_dir(&path_new);
    }

    fs::rename(path_old, path_new)?;
    return Ok(());
}

pub fn update_file_modified_time(
    file_path: &str,
    iso8601_string: &str,
) -> Result<(), Box<dyn Error>> {
    xu_fs::set_modified_from_iso8601(&file_path, &iso8601_string)?;
    return Ok(());
}

// Format time string
// According to ISO 8601, like: `2017-04-20T11:32:00.000+08:00`
const FILE_INFO_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.9f%:z";
fn tm_string(tm: DateTime<FixedOffset>) -> String {
    return format!("{}", tm.format(FILE_INFO_TIME_FORMAT));
}

fn convert_file_info(fi: xu_fs::FileInfo) -> types::FileInfo {
    let res = types::FileInfo {
        name: fi.name,
        path: fi.path,
        size: fi.size,
        created_time: tm_string(fi.created),
        accessed_time: tm_string(fi.accessed),
        modified_time: tm_string(fi.modified),
        err_msg: fi.err_msg,
    };

    return res;
}

pub fn file_info(file_path: &str) -> types::FileInfo {
    let ccc = xu_fs::file_info(Path::new(file_path));
    return convert_file_info(ccc);
}

// Every line like: 2024-01-03 23:51:06.000000000 +0800			/IT/flutter/debian.md
pub fn walk_dir_items_get_path_and_modify_time(
    file_path: &str,
    dir_path: &str,
    exclude_dires: &Vec<String>,
    ssep: &str,
) -> Result<String, Box<dyn Error>> {
    let ccc = xu_fs::tree_info_vec(&dir_path, &exclude_dires)?;
    let mut list: Vec<types::FileInfo> = [].to_vec();

    for item in ccc {
        list.push(convert_file_info(item))
    }

    let mut ooo = String::new();

    for item in &list {
        let line = format!(
            "{}{}{}\n",
            item.modified_time,
            ssep,
            item.path.replace(&dir_path, "")
        );
        ooo.push_str(&line);
    }

    if file_path != "" {
        xu_fs::write_str(file_path, ooo.as_str())?;
    };

    return Ok(ooo);
}

// Traverse the specified directory to obtain the paths of all files and directories under it.
pub fn walk_dir_items_get_path(
    dir_path: &str,
    exclude_dires: &Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let ccc = xu_fs::tree_info_vec(&dir_path, &exclude_dires)?;
    let mut list: Vec<types::FileInfo> = [].to_vec();

    for item in ccc {
        list.push(convert_file_info(item))
    }

    let mut ooo: Vec<String> = [].to_vec();

    for item in &list {
        ooo.push(item.path.replace(&dir_path, ""));
    }

    return Ok(ooo);
}

#[test]
fn test_fi() {
    let fi = file_info("/xxx/TODO.md");
    println!(">>> fi ::{:?}", fi);
}

pub fn tree_info(dir_path: &str) -> Result<FileNode, Box<dyn Error>> {
    return xu_fs::tree_info(Path::new(dir_path), "\r\r\r");
}

pub fn zip_dir(file_path: &str, dir_path: &str) -> Result<(), Box<dyn Error>> {
    xu_zip::zip_dir(&dir_path, &file_path)?;
    return Ok(());
}

pub fn unzip_file(file_path: &str, dir_path: &str) -> Result<(), Box<dyn Error>> {
    xu_fs::check_or_create_dir(&dir_path)?;
    xu_zip::unzip_file(&file_path, &dir_path)?;
    return Ok(());
}

pub fn search_document(
    dir_path: &Path,
    use_re: bool,
    search: &str,
    wrapper: &str,
    context_size: usize,
) -> Result<Vec<String>, io::Error> {
    xu_fs::search_document(dir_path, use_re, search, wrapper, context_size)
}
