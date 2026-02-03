use crate::cli::args::{Cli, VlessOptions};
use crate::domain::isp::Isp as DomainIsp;
use crate::domain::model::IpInfo;
use crate::sources::AggregatedSource;
use base64::{Engine as _, engine::general_purpose};
use std::{fs, path::PathBuf};
pub mod cli;
pub mod domain;
pub mod sources;

pub fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let isps: Vec<DomainIsp> = cli.isp.iter().map(|i| (*i).into()).collect();

    let source = AggregatedSource::new();
    let ip_infos = source.fetch(&isps, cli.count);

    if ip_infos.is_empty() {
        eprintln!("未获取到任何 IP 信息");
        return Ok(());
    }

    let config = ip_infos
        .iter()
        .map(|info| generate_config(info, &cli.vless))
        .collect::<Vec<_>>()
        .join("\n");

    let encoded = general_purpose::STANDARD.encode(&config);
    save_config(&encoded, &cli.output)?;

    println!("节点信息已写入文件：{:?}", cli.output);
    Ok(())
}

fn generate_config(info: &IpInfo, vless: &VlessOptions) -> String {
    let path_encoded = standard_url_encode(&vless.path);
    let config = format!(
        "vless://{}@{}:{}?encryption=none&security=tls&sni={}&type=ws&host={}&path={}#{}",
        vless.uuid,
        info.ip,
        vless.port,
        vless.sni,
        vless.host,
        path_encoded,
        info.get_remark()
    );
    config
}

fn save_config(config: &str, path: &PathBuf) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, config)?;
    Ok(())
}

fn standard_url_encode(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'/'
            | b'?'
            | b'='
            | b'-'
            | b'_'
            | b'.'
            | b'~' => encoded.push(*byte as char),
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}
