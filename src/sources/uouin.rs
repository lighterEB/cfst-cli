use super::IpSource;
use crate::domain::{error::DomainError, isp::Isp, model::IpInfo};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
pub struct UouinResponse {
    pub statu: String,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(flatten, deserialize_with = "deserialize_isp_map")]
    pub nodes: HashMap<Isp, NodeGroup>,
}

fn deserialize_isp_map<'de, D>(deserializer: D) -> Result<HashMap<Isp, NodeGroup>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: HashMap<String, NodeGroup> = HashMap::deserialize(deserializer)?;
    raw.into_iter()
        .filter_map(|(k, v)| {
            let isp = match k.as_str() {
                "bgp" => Some(Isp::Cn),
                "ctcc" => Some(Isp::Ct),
                "cmcc" => Some(Isp::Cmcc),
                "cucc" => Some(Isp::Cu),
                "ipv6" => None,
                _ => None,
            };
            isp.map(|i| Ok((i, v)))
        })
        .collect()
}

#[derive(Deserialize, Debug)]
pub struct NodeGroup {
    pub info: Vec<UouinInfo>,
}

#[derive(Deserialize, Debug)]
pub struct UouinInfo {
    pub ip: String,
}

pub struct UouinSource {
    base_url: String,
    key: String,
    time: String,
}

impl UouinSource {
    pub fn new() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            base_url: "https://api.uouin.com/index.php/index/Cloudflare".to_string(),
            key: general_key(ts),
            time: ts.to_string(),
        }
    }
}

impl IpSource for UouinSource {
    fn supported_isps(&self) -> &[Isp] {
        &[Isp::Cmcc, Isp::Ct, Isp::Cu, Isp::Cn]
    }

    fn fetch(&self) -> Result<Vec<IpInfo>, DomainError> {
        let url = format!("{}?key={}&time={}", &self.base_url, &self.key, &self.time);
        // let response: UouinResponse = reqwest::blocking::get(url)
        //     .map_err(|e| DomainError::FetchFailed(e.to_string()))?
        //     .json()
        //     .map_err(|e| DomainError::ParseFailed(e.to_string()))?;
        let response =
            reqwest::blocking::get(url).map_err(|e| DomainError::FetchFailed(e.to_string()))?;
        let text = response.text().unwrap();
        println!("{}", text);
        let parsed: UouinResponse =
            serde_json::from_str(&text).map_err(|e| DomainError::ParseFailed(e.to_string()))?;
        if !parsed.statu.as_bytes().eq("true".as_bytes()) {
            return Err(DomainError::FetchFailed(
                "API 返回 status=false".to_string(),
            ));
        }
        let ips: Vec<IpInfo> = parsed
            .data
            .nodes
            .iter()
            .flat_map(|(k, v)| {
                v.info
                    .iter()
                    .map(move |i| IpInfo::new(i.ip.clone(), k.clone(), None))
            })
            .collect();
        Ok(ips)
    }

    fn name(&self) -> &'static str {
        "uouin"
    }
}

fn general_key(ts: u128) -> String {
    let origin_text = format!("3aaeb442a2c052076bc47b3ce656211670cloudflareapikey{ts}");
    format!("{:x}", md5::compute(origin_text))
}
