use super::isp::Isp;
#[derive(Debug, Clone)]
pub struct IpInfo {
    pub ip: String,
    pub isp: Isp,
    pub remark: String,
}

impl IpInfo {
    pub fn new(ip: String, isp: Isp, remark: Option<String>) -> Self {
        Self {
            ip,
            isp,
            remark: remark.unwrap_or_default(),
        }
    }
    pub fn get_remark(&self) -> String {
        if self.remark.eq("") {
            format!("CF优选-{}", self.isp.to_name())
        } else {
            self.remark.clone()
        }
    }
}
