use super::IpSource;
use crate::domain::{error::DomainError, isp::Isp, model::IpInfo};

pub struct Cf090227Source {
    base_url: String,
}

impl Cf090227Source {
    pub fn new() -> Self {
        Self {
            base_url: "https://cf.090227.xyz".to_string(),
        }
    }

    fn fetch_isp(&self, isp: Isp) -> Result<Vec<IpInfo>, DomainError> {
        let isp_param = match isp {
            Isp::Cmcc => "cmcc",
            Isp::Ct => "ct",
            Isp::Cu => "cu",
            Isp::Cn => return Ok(vec![]),
        };

        let url = format!("{}/{}", self.base_url, isp_param);
        let response = reqwest::blocking::get(&url)
            .map_err(|e| DomainError::FetchFailed(e.to_string()))?
            .text()
            .map_err(|e| DomainError::FetchFailed(e.to_string()))?;

        let ips = response
            .lines()
            .filter_map(|line| {
                let ip = line.split('#').next()?.trim().to_string();
                if ip.is_empty() {
                    None
                } else {
                    Some(IpInfo::new(ip, isp, None))
                }
            })
            .collect();

        Ok(ips)
    }
}

impl IpSource for Cf090227Source {
    fn supported_isps(&self) -> &[Isp] {
        &[Isp::Cmcc, Isp::Ct, Isp::Cu]
    }

    fn fetch(&self) -> Result<Vec<IpInfo>, DomainError> {
        let mut all = Vec::new();
        for isp in self.supported_isps() {
            all.extend(self.fetch_isp(*isp)?);
        }
        Ok(all)
    }

    fn name(&self) -> &'static str {
        "cf.090227.xyz"
    }
}

pub fn parse_cf090227_response(response: &str) -> Vec<IpInfo> {
    response
        .lines()
        .filter_map(|line| {
            let ip = line.split('#').next()?.trim().to_string();
            if ip.is_empty() {
                None
            } else {
                Some(IpInfo::new(ip, Isp::Cmcc, None))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cf090227_mock() {
        let mock = "104.19.60.223#CF优选-移动\n104.19.54.135#CF优选-移动";
        let ips = parse_cf090227_response(mock);
        assert_eq!(ips.len(), 2);
        assert_eq!(ips[0].ip, "104.19.60.223");
        assert_eq!(ips[1].ip, "104.19.54.135");
    }
}
