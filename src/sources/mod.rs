mod cf090227;
mod wetest;

pub use cf090227::{parse_cf090227_response, Cf090227Source};
pub use wetest::{parse_wetest_response, WetestIpInfo, WetestResponse, WetestSource};

use crate::domain::{error::DomainError, isp::Isp, model::IpInfo};
use std::collections::HashSet;

pub trait IpSource {
    fn supported_isps(&self) -> &[Isp];

    fn fetch(&self) -> Result<Vec<IpInfo>, DomainError>;

    fn name(&self) -> &'static str;
}

pub struct AggregatedSource {
    sources: Vec<Box<dyn IpSource>>,
}

impl AggregatedSource {
    pub fn new() -> Self {
        Self {
            sources: vec![
                Box::new(Cf090227Source::new()),
                Box::new(WetestSource::new()),
            ],
        }
    }

    pub fn fetch(&self, isps: &[Isp], count: usize) -> Vec<IpInfo> {
        let mut all_ips: Vec<IpInfo> = Vec::new();

        for source in &self.sources {
            match source.fetch() {
                Ok(ips) => all_ips.extend(ips),
                Err(e) => eprintln!("[{}] {}", source.name(), e),
            }
        }

        let mut result = Vec::new();
        for isp in isps {
            let mut seen = HashSet::new();
            let filtered: Vec<_> = all_ips
                .iter()
                .filter(|info| &info.isp == isp && seen.insert(info.ip.clone()))
                .take(count)
                .cloned()
                .collect();
            result.extend(filtered);
        }
        result
    }
}
