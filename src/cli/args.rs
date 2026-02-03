use crate::domain::isp as domain;
use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Isp {
    Ct,   // 中国电信
    Cmcc, // 中国移动
    Cu,   // 中国联通
    Cn,   // 多线/三网
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

impl From<Isp> for domain::Isp {
    fn from(value: Isp) -> Self {
        match value {
            Isp::Ct => domain::Isp::Ct,
            Isp::Cmcc => domain::Isp::Cmcc,
            Isp::Cu => domain::Isp::Cu,
            Isp::Cn => domain::Isp::Cn,
        }
    }
}

// impl Isp {
//     pub fn as_str(&self) -> &str {
//         match self {
//             Isp::Cmcc => "cmcc",
//             Isp::Ct => "ct",
//             Isp::Cu => "cu",
//         }
//     }
// }
