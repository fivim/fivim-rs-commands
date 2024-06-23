use fivim_rs_utils::search as xu_search;
use std::{io, path::PathBuf};

pub fn search_in_dir(
    dir_path: String,
    is_re_mode: bool,
    search: String,
    context_size: usize,
    wrapper_prefix: String,
    wrapper_postfix: String,
    html_like_exts: &Vec<String>,
) -> Result<Vec<xu_search::SearchFileRes>, io::Error> {
    let p = PathBuf::from(&dir_path);

    xu_search::search_in_dir(
        &p,
        &search,
        is_re_mode,
        context_size,
        &wrapper_prefix,
        &wrapper_postfix,
        html_like_exts,
    )
}

pub fn search_in_file(
    file_path: String,
    is_re_mode: bool,
    search: String,
    context_size: usize,
    wrapper_prefix: String,
    wrapper_postfix: String,
    html_like_exts: &Vec<String>,
) -> Result<Vec<xu_search::SearchFileRes>, io::Error> {
    let p = PathBuf::from(&file_path);

    xu_search::search_in_file(
        &p,
        &search,
        is_re_mode,
        context_size,
        &wrapper_prefix,
        &wrapper_postfix,
        html_like_exts,
    )
}
