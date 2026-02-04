use super::IpSource;
use crate::domain::{error::DomainError, isp::Isp, model::IpInfo};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct WetestResponse {
    pub status: bool,
    pub info: HashMap<String, Vec<WetestIpInfo>>,
}

#[derive(Deserialize)]
pub struct WetestIpInfo {
    pub ip: String,
    pub line: String,
}

pub struct WetestSource {
    url: String,
}

impl WetestSource {
    pub fn new() -> Self {
        Self {
            url: "https://www.wetest.vip/api/cf2dns/get_cloudflare_ip?key=o1zrmHAF&type=v4"
                .to_string(),
        }
    }

    pub fn line_to_isp(line: &str) -> Option<Isp> {
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
            return Err(DomainError::FetchFailed(
                "API 返回 status=false".to_string(),
            ));
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

pub fn parse_wetest_response(response: &str) -> Result<Vec<IpInfo>, serde_json::Error> {
    let response: WetestResponse = serde_json::from_str(response)?;

    let mut ips = Vec::new();
    for (_, ip_list) in response.info {
        for item in ip_list {
            if let Some(isp) = WetestSource::line_to_isp(&item.line) {
                ips.push(IpInfo::new(item.ip, isp, None));
            }
        }
    }
    Ok(ips)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wetest_mock() {
        let mock = r#"{"status":true,"info":{"CM":[{"ip":"104.18.38.125","line":"cm"}],"CT":[{"ip":"162.159.39.237","line":"ct"}],"CU":[{"ip":"104.26.13.170","line":"cu"}],"CN":[{"ip":"108.162.198.179","line":"cn"}]}}"#;
        let ips = parse_wetest_response(mock).unwrap();
        assert_eq!(ips.len(), 4);

        let isps: Vec<_> = ips.iter().map(|ip| ip.isp).collect();
        assert!(isps.contains(&Isp::Cmcc));
        assert!(isps.contains(&Isp::Ct));
        assert!(isps.contains(&Isp::Cu));
        assert!(isps.contains(&Isp::Cn));
    }
}
