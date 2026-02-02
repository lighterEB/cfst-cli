#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Isp {
    Ct,   // 中国电信
    Cmcc, // 中国移动
    Cu,   // 中国联通
    Cn,   // 多线/三网
}

impl Isp {
    /// 返回映射中文名，用于拼接 remark
    pub fn to_name(self) -> &'static str {
        match self {
            Isp::Cmcc => "移动",
            Isp::Ct => "电信",
            Isp::Cu => "联通",
            Isp::Cn => "三网",
        }
    }
}
