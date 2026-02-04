use cfst::sources::{Cf090227Source, IpSource, WetestSource};

#[cfg(test)]
mod real_api_tests {
    use super::*;

    #[test]
    #[ignore = "需要网络连接，手动运行"]
    fn test_cf090227_real_api() {
        let source = Cf090227Source::new();
        let ips = source.fetch().expect("API A (cf.090227.xyz) 请求失败");
        assert!(!ips.is_empty(), "API A 返回为空");
        println!("API A 获取到 {} 个 IP", ips.len());
    }

    #[test]
    #[ignore = "需要网络连接，手动运行"]
    fn test_wetest_real_api() {
        let source = WetestSource::new();
        let ips = source.fetch().expect("API C (wetest.vip) 请求失败");
        assert!(!ips.is_empty(), "API C 返回为空");
        println!("API C 获取到 {} 个 IP", ips.len());
    }
}
