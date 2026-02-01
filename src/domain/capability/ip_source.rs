use crate::domain::model::IpInfo;
use crate::domain::isp::Isp;
use crate::domain::error::DomainError;

pub trait IpSource {
    fn fetch(&self, isp: Isp) -> Result<Vec<IpInfo>, DomainError>;
}