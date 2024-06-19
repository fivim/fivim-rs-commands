use base64::engine::general_purpose::STANDARD as b64_STANDARD;
use base64::Engine;
use fivim_rs_encrypt::{aes_gcm, chacha20poly1305 as x_cp};
use fivim_rs_utils::{fs as xu_fs, hash as xu_hash};
use std::error::Error;

pub fn encrypt_string_into_file(
    pwd: &Vec<u8>,
    file_path: &str,
    file_content: &str,
) -> Result<(), Box<dyn Error>> {
    let ccc = aes_gcm::encrypt(
        file_content,
        aes_gcm::key_from_vec(&pwd),
        aes_gcm::nonce_from_vec(&pwd),
    )?;

    xu_fs::write_bytes(file_path, &ccc)?;
    return Ok(());
}

pub fn decrypt_file_to_string(pwd: &Vec<u8>, file_path: &str) -> Result<String, Box<dyn Error>> {
    let ccc = xu_fs::read_to_bytes(&file_path, false)?;

    let ddd = aes_gcm::decrypt(
        &ccc,
        aes_gcm::key_from_vec(&pwd),
        aes_gcm::nonce_from_vec(&pwd),
    )?;
    let sss = String::from_utf8(ddd)?;
    return Ok(sss);
}

pub fn encrypt_string(pwd: &Vec<u8>, content: &str) -> Result<String, Box<dyn Error>> {
    let eee = aes_gcm::encrypt(
        content,
        aes_gcm::key_from_vec(&pwd),
        aes_gcm::nonce_from_vec(&pwd),
    )?;
    let sss = serde_json::to_string(&eee)?;
    return Ok(sss);
}

pub fn decrypt_string(pwd: &Vec<u8>, content: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let ddd = aes_gcm::decrypt(
        &content,
        aes_gcm::key_from_vec(&pwd),
        aes_gcm::nonce_from_vec(&pwd),
    )?;
    let sss = String::from_utf8(ddd)?;
    return Ok(sss);
}

pub fn encrypt_string_array(
    pwd: &Vec<u8>,
    content: &Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res_lines: Vec<String> = [].to_vec();
    let key = aes_gcm::key_from_vec(&pwd);
    let nonce = aes_gcm::nonce_from_vec(&pwd);
    for line in content {
        if line == "" {
            res_lines.push("".to_owned());
            continue;
        }

        let eee = aes_gcm::encrypt(&line, key, nonce)?;
        let ened = b64_STANDARD.encode(eee);
        res_lines.push(ened)
    }

    return Ok(res_lines);
}

pub fn decrypt_string_array(
    pwd: &Vec<u8>,
    content: &Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res_lines: Vec<String> = [].to_vec();

    let key = aes_gcm::key_from_vec(&pwd);
    let nonce = aes_gcm::nonce_from_vec(&pwd);
    for line in content {
        if line == "" {
            res_lines.push("".to_owned());
            continue;
        }

        let bv = b64_STANDARD.decode(line)?;
        let ddd = aes_gcm::decrypt(&bv, key, nonce)?;
        let sss = String::from_utf8(ddd)?;
        res_lines.push(sss)
    }

    return Ok(res_lines);
}

pub fn encrypt_local_file(
    pwd: &Vec<u8>,
    file_path_from: &str,
    file_path_to: &str,
) -> Result<(), Box<dyn Error>> {
    let res = x_cp::encrypt_large_file(
        &file_path_from,
        &file_path_to,
        &x_cp::key_from_vec(&pwd),
        &x_cp::nonce_from_vec_large(&pwd),
    )?;

    return Ok(res);
}

pub fn encrypt_local_file_content_base64(
    pwd: &Vec<u8>,
    content: &Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    let res = x_cp::encrypt_large_file_content_base64(
        &content,
        &x_cp::key_from_vec(&pwd),
        &x_cp::nonce_from_vec_large(&pwd),
    )?;

    return Ok(res);
}

pub fn decrypt_local_file(
    pwd: &Vec<u8>,
    file_path_from: &str,
    file_path_to: &str,
) -> Result<(), Box<dyn Error>> {
    let res = x_cp::decrypt_large_file(
        &file_path_from,
        &file_path_to,
        &x_cp::key_from_vec(&pwd),
        &x_cp::nonce_from_vec_large(&pwd),
    )?;

    return Ok(res);
}

pub fn decrypt_local_file_base64(pwd: &Vec<u8>, file_path: &str) -> Result<String, Box<dyn Error>> {
    let res = x_cp::decrypt_large_file_base64(
        &file_path,
        &x_cp::key_from_vec(&pwd),
        &x_cp::nonce_from_vec_large(&pwd),
    )?;

    return Ok(res);
}

#[test]
fn test_encrypt_base64() {
    let pwd: Vec<u8> = [
        31, 31, 41, 50, 61, 67, 73, 73, 74, 75, 99, 115, 120, 121, 144, 151, 152, 154, 162, 176,
        178, 211, 212, 212, 213, 214, 220, 224, 231, 234, 240, 251,
    ]
    .to_vec();
    let content = "qwertyuiop".as_bytes().to_vec();

    println!("text before encryption:  {:?}", &content);
    let enenen = b64_STANDARD.encode(&content);
    println!("base64 before encryption:  {}", &enenen);

    let file_content_base64 = match encrypt_local_file_content_base64(&pwd, &content) {
        Ok(sss) => sss,
        Err(e) => {
            dbg!(e);
            return;
        }
    };

    let file_path = "./test_base64";
    match xu_fs::write_base64_str(&file_path, &file_content_base64) {
        Ok(_) => {
            match decrypt_local_file_base64(&pwd, &file_path) {
                Ok(sss) => {
                    println!("base64 after decryption: {}", sss);
                    match b64_STANDARD.decode(&sss) {
                        Ok(www) => {
                            // let qwer = String::from_utf8(www).unwrap();
                            let qwer = std::str::from_utf8(&www).unwrap();

                            println!("text after decryption: {}", qwer)
                        }
                        Err(ee) => {
                            dbg!(ee);
                            return;
                        }
                    };

                    return;
                }
                Err(e) => {
                    dbg!(e);
                    return;
                }
            };
        }
        Err(e) => {
            dbg!(e);
            return;
        }
    };
}

pub fn sha256_by_file_path(file_path: &str) -> Result<String, Box<dyn Error>> {
    return xu_hash::sha256_by_file_path(&file_path);
}

pub fn string_crc32(string: &str) -> u32 {
    return xu_hash::crc32_by_bytes(string.as_bytes());
}

pub fn string_sha256(content: &str) -> String {
    return xu_hash::sha256_by_bytes(content.as_bytes()).to_string();
}
