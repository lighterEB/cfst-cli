use crate::domain::{error::DomainError, isp::Isp, model::IpInfo};
use super::IpSource;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct WetestResponse {
    status: bool,
    info: HashMap<String, Vec<WetestIpInfo>>,
}

#[derive(Deserialize)]
struct WetestIpInfo {
    ip: String,
    line: String,
}

pub struct WetestSource {
    url: String,
}

impl WetestSource {
    pub fn new() -> Self {
        Self {
            url: "https://www.wetest.vip/api/cf2dns/get_cloudflare_ip?key=o1zrmHAF&type=v4".to_string(),
        }
    }
    
    fn line_to_isp(line: &str) -> Option<Isp> {
        match line.to_lowercase().as_str() {
            "cm" => Some(Isp::Cmcc),
            "ct" => Some(Isp::Ct),
            "cu" => Some(Isp::Cu),
            "cn" => Some(Isp::Cn),
            _ => None,
        }
    }
}

impl IpSource for WetestSource {
    fn supported_isps(&self) -> &[Isp] {
        &[Isp::Cmcc, Isp::Ct, Isp::Cu, Isp::Cn]
    }
    
    fn fetch(&self) -> Result<Vec<IpInfo>, DomainError> {
        let response: WetestResponse = reqwest::blocking::get(&self.url)
            .map_err(|e| DomainError::FetchFailed(e.to_string()))?
            .json()
            .map_err(|e| DomainError::ParseFailed(e.to_string()))?;
        
        if !response.status {
            return Err(DomainError::FetchFailed("API 返回 status=false".to_string()));
        }
        
        let mut ips = Vec::new();
        for (_, ip_list) in response.info {
            for item in ip_list {
                if let Some(isp) = Self::line_to_isp(&item.line) {
                    ips.push(IpInfo::new(item.ip, isp, None));
                }
            }
        }
        Ok(ips)
    }
    
    fn name(&self) -> &'static str {
        "wetest.vip"
    }
}