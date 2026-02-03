use crate::cli::args::{Cli, VlessOptions};
use crate::domain::isp::Isp as DomainIsp;
use crate::domain::model::IpInfo;
use crate::sources::AggregatedSource;
use base64::{engine::general_purpose, Engine as _};
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
    let path_encoded = url_encode(&vless.path);
    let remark_encoded = url_encode(&info.get_remark());
    format!(
        "vless://{}@{}:{}?encryption=none&security=tls&sni={}&type=ws&host={}&path={}#{}",
        vless.uuid, info.ip, vless.port, vless.sni, vless.host, path_encoded, remark_encoded
    )
}

fn save_config(config: &str, path: &PathBuf) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, config)?;
    Ok(())
}

/// 标准 URL 编码，只保留 unreserved characters (RFC 3986)
/// 不编码: A-Z a-z 0-9 - _ . ~
/// 其他所有字符都编码
fn url_encode(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char)
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_path() {
        let input = "/62acd959-eb9a-47e3-b677-3914ffa2b166-vless?ed=2056";
        let result = url_encode(input);
        println!("Path Input:  {}", input);
        println!("Path Output: {}", result);

        // / ? = 应该被编码
        assert_eq!(
            result,
            "%2F62acd959-eb9a-47e3-b677-3914ffa2b166-vless%3Fed%3D2056"
        );
    }

    #[test]
    fn test_url_encode_chinese() {
        let input = "CF优选-移动";
        let result = url_encode(input);
        println!("Remark Input:  {}", input);
        println!("Remark Output: {}", result);

        // 中文应该被编码，- 不应该被编码
        assert!(result.contains("-"));
        assert!(result.starts_with("CF%"));
    }
}
