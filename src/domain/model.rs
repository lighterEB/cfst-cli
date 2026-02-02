use super::isp::Isp;
#[derive(Debug, Clone)]
pub struct IpInfo {
    ip: String,
    isp: Isp,
    remark: String,
}

impl IpInfo {
    pub fn new(ip: String, isp: Isp, remark: Option<String>) -> Self {
        let remark = match remark {
            None => "CF优选".to_string(),
            Some(remark) => remark,
        };
        Self { ip, isp, remark }
    }
}
