use cfst::sources::{
    Cf090227Source, IpSource, WetestSource, parse_cf090227_response, parse_wetest_response,
};
use cfst::domain::isp::Isp;

const MOCK_CF090227: &str = r#"104.19.60.223#CF优选-移动
104.19.54.135#CF优选-移动
104.19.34.226#CF优选-移动"#;

const MOCK_WETEST: &str = r#"{
    "status": true,
    "info": {
        "CM": [{"ip": "104.18.38.125", "line": "cm"}],
        "CT": [{"ip": "162.159.39.237", "line": "ct"}],
        "CU": [{"ip": "104.26.13.170", "line": "cu"}],
        "CN": [{"ip": "108.162.198.179", "line": "cn"}]
    }
}"#;

#[cfg(test)]
mod mock_tests {
    use super::*;

    #[test]
    fn test_parse_cf090227_mock() {
        let ips = parse_cf090227_response(MOCK_CF090227);
        assert_eq!(ips.len(), 3);
        assert_eq!(ips[0].ip, "104.19.60.223");
        assert_eq!(ips[1].ip, "104.19.54.135");
        assert_eq!(ips[2].ip, "104.19.34.226");
        assert!(ips.iter().all(|ip| ip.ip.starts_with("104.19.")));
    }

    #[test]
    fn test_parse_wetest_mock() {
        let ips = parse_wetest_response(MOCK_WETEST).unwrap();
        assert_eq!(ips.len(), 4);

        let isps: Vec<_> = ips.iter().map(|ip| ip.isp).collect();
        assert!(isps.contains(&Isp::Cmcc));
        assert!(isps.contains(&Isp::Ct));
        assert!(isps.contains(&Isp::Cu));
        assert!(isps.contains(&Isp::Cn));

        let ip_ips: Vec<_> = ips.iter().map(|ip| ip.ip.as_str()).collect();
        assert!(ip_ips.contains(&"104.18.38.125"));
        assert!(ip_ips.contains(&"162.159.39.237"));
        assert!(ip_ips.contains(&"104.26.13.170"));
        assert!(ip_ips.contains(&"108.162.198.179"));
    }

    #[test]
    fn test_parse_wetest_line_mapping() {
        let mock = r#"{"status":true,"info":{"CM":[{"ip":"1.1.1.1","line":"cm"}],"CT":[{"ip":"2.2.2.2","line":"CT"}],"CU":[{"ip":"3.3.3.3","line":"Cu"}],"CN":[{"ip":"4.4.4.4","line":"CN"}]}}"#;
        let ips = parse_wetest_response(mock).unwrap();

        assert_eq!(ips.len(), 4);

        let ip_to_isp: std::collections::HashMap<_, _> = ips
            .iter()
            .map(|ip| (ip.ip.as_str(), ip.isp))
            .collect();

        assert_eq!(ip_to_isp.get("1.1.1.1"), Some(&Isp::Cmcc));
        assert_eq!(ip_to_isp.get("2.2.2.2"), Some(&Isp::Ct));
        assert_eq!(ip_to_isp.get("3.3.3.3"), Some(&Isp::Cu));
        assert_eq!(ip_to_isp.get("4.4.4.4"), Some(&Isp::Cn));
    }
}

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
