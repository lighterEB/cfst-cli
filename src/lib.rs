use clap::{Args, Parser, ValueEnum};
use std::{fs, path::PathBuf};
use base64::{Engine as _, engine::general_purpose};

#[derive(Parser)]
#[command(author, version, about = "CF IP 提取工具")]
pub struct Cli {
    /// 1. 运营商
    #[arg(short, long, value_enum, num_args=1..)]
    pub isp: Vec<Isp>,

    /// 2. 获取IP个数
    #[arg(short, long, default_value_t = 10)]
    pub count: usize,

    /// 3.写入文件路径
    #[arg(short, long, value_name = "OUTPUT", default_value = "result.txt")]
    pub output: PathBuf,

    #[command(flatten)]
    pub vless: VlessOptions,
}

#[derive(Args, Debug)]
pub struct VlessOptions {
    /// Vless UUID
    #[arg(long, default_value = "5a0d12ef-6cb7-49bd-b2aa-feb8c395aa9a")]
    pub uuid: String,

    /// 端口
    #[arg(short, long, default_value_t = 443)]
    pub port: u16,

    /// SNI (Server Name Indication)
    #[arg(long, default_value = "custom.com")]
    pub sni: String,

    /// Host
    #[arg(long, default_value = "custom.com")]
    pub host: String,

    /// Path (WS路径)
    #[arg(long, default_value = "/custom.com?ed=2560")]
    pub path: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Isp {
    Ct,   // 中国电信
    Cmcc, // 中国移动
    Cu,   // 中国联通
}

impl Isp {
    fn as_str(&self) -> &str {
        match self {
            Isp::Cmcc => "cmcc",
            Isp::Ct => "ct",
            Isp::Cu => "cu",
        }
    }
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>>{
    let mut res = String::new();
    for item in &cli.isp {
        match fetch_info(item.as_str(), cli.count) {
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

fn fetch_info(isp: &str, count: usize) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://cf.090227.xyz/{isp}?ips={count}");
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