use clap::{Args, Parser, ValueEnum};
use std::{fs, path::PathBuf};
use base64::{Engine as _, engine::general_purpose};
use crate::cli::args::{Cli, VlessOptions};
use crate::domain::isp::Isp as DomainIsp;
use crate::domain::isp::Isp;
pub mod domain;
pub mod cli;

pub fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>>{
    let mut res = String::new();
    for item in &cli.isp {
        let isp:DomainIsp = (*item).into();
        match fetch_info(isp, cli.count) {
            Ok(content) => {
                res.push_str(&content);
                res.push_str("\n");
            }
            Err(e) => {
                eprintln!("请求地址失败: {}", e);
            }
        }
    }

    let mut final_config = String::new();
    for line in res.lines() {
        if let Some(config) = generate_config(line, &cli.vless) {
            final_config.push_str(&config);
            final_config.push_str("\n");
        }
    }
    let config_encoded = general_purpose::STANDARD.encode(final_config);
    match save_config(&config_encoded, &cli.output) {
        Ok(()) => {
            println!("节点信息已写入文件：{:?}", cli.output);
            Ok(())
        }
        Err(e) => {
            eprintln!("节点信息保存失败：{}", e);
            Err(Box::new(e))
        }
    }
}

fn generate_config(line: &str, vless: &VlessOptions) -> Option<String> {
    let (ip, remark) = line.split_once("#")?;
    let ip = ip.trim();
    let remark = remark.trim();
    let path_encoded = standard_url_encode(&vless.path);
    let config = format!(
        "vless://{}@{}:{}?encryption=none&security=tls&sni={}&type=ws&host={}&path={}#{}",
        vless.uuid,
        ip,
        vless.port,
        vless.sni,
        vless.host,
        path_encoded,
        remark
    );
    Some(config)
}

fn save_config(config: &str, path: &PathBuf) ->Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, config)?;
    Ok(())
}

fn fetch_info(isp: Isp, count: usize) -> Result<String, Box<dyn std::error::Error>> {
    let isp_str = match isp {
        Isp::Cu => "cu",
        Isp::Cmcc => "cmcc",
        Isp::Ct => "ct"
    };
    let url = format!("https://cf.090227.xyz/{isp_str}?ips={count}");
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}

fn standard_url_encode(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.as_bytes() {
        match byte {
            b'A'..= b'Z' | b'a'..= b'z' | b'0' ..= b'9' | b'/' | b'?' | b'=' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char)
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}