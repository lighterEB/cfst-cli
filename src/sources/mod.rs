mod cf090227;
mod uouin;
mod wetest;

pub use cf090227::Cf090227Source;
pub use wetest::WetestSource;

use crate::{
    domain::{error::DomainError, isp::Isp, model::IpInfo},
    sources::uouin::UouinSource,
};
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
    pub fn new(selected_sources: &[crate::cli::args::ApiSource]) -> Self {
        let mut sources: Vec<Box<dyn IpSource>> = Vec::new();

        if selected_sources.is_empty() {
            // 如果用户未选择，默认加载全部
            sources.push(Box::new(Cf090227Source::new()));
            sources.push(Box::new(WetestSource::new()));
            sources.push(Box::new(UouinSource::new()));
        } else {
            // 否则按需实例化
            for src in selected_sources {
                match src {
                    crate::cli::args::ApiSource::Cf090227 => {
                        sources.push(Box::new(Cf090227Source::new()))
                    }
                    crate::cli::args::ApiSource::Wetest => {
                        sources.push(Box::new(WetestSource::new()))
                    }
                    crate::cli::args::ApiSource::Uouin => {
                        sources.push(Box::new(UouinSource::new()))
                    }
                }
            }
        }

        Self { sources }
    }

    pub fn fetch(&self, isps: &[Isp], count: usize, quiet: bool) -> Vec<IpInfo> {
        let mut all_ips: Vec<IpInfo> = Vec::new();

        for source in &self.sources {
            match source.fetch() {
                Ok(ips) => all_ips.extend(ips),
                Err(e) => {
                    if !quiet {
                        eprintln!("[{}] {}", source.name(), e)
                    }
                }
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
