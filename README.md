# CFST-CLI - Cloudflare 优选 IP 获取工具

一个用 Rust 编写的 Cloudflare 优选 IP 获取工具，用于从多个第三方 API 获取优选 IP 并生成 VLESS 代理配置。

## 说明

本工具不进行 IP 测速和优选，而是从第三方优选 API 获取已经过测速的优选 IP，
然后为这些 IP 生成可直接使用的 VLESS 代理配置。

## 功能特性

- 支持多数据源聚合（cf.090227.xyz、wetest.vip）
- 支持三大运营商线路（电信/移动/联通）及三网优化线路
- 自动去重，确保 IP 不重复
- 自动生成 VLESS+WebSocket+TLS 配置
- 支持自定义 VLESS 参数（UUID、端口、SNI 等）
- 输出 Base64 编码配置，可直接导入代理客户端

## 数据源

| 数据源 | 支持线路 | 说明 |
|--------|----------|------|
| cf.090227.xyz | 电信/移动/联通 | 分线路获取 |
| wetest.vip | 电信/移动/联通/三网 | 一次获取全部 |

## 安装

```bash
git clone https://github.com/lighterEB/cfst-cli.git
cd cfst
cargo build --release
```

## 使用示例

### 基础使用

```bash
# 获取移动和电信各 10 个优选 IP
cfst -i cmcc ct

# 获取联通 20 个优选 IP
cfst -i cu -c 20

# 获取三网优化线路的 IP（适用于电信/移动/联通均有优化的线路）
cfst -i cn -c 10

# 指定输出文件
cfst -i ct -c 20 -o my_nodes.txt
```

### 自定义 VLESS 配置

```bash
cfst -i ct -c 10 \
  --uuid "your-custom-uuid" \
  --port 8443 \
  --sni "your-domain.com" \
  --host "your-domain.com" \
  --path "/your-custom-path"
```

### 获取所有线路

```bash
# 获取电信、移动、联通、三网各 5 个 IP
cfst -i ct cmcc cu cn -c 5
```

### 查看帮助

```bash
cfst --help
```

## 命令行参数

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--isp <ISP>...` | `-i` | 线路类型（支持多选，空格分隔） | 必填 |
| `--count <N>` | `-c` | 每个线路获取 IP 数量 | 10 |
| `--output <PATH>` | `-o` | 输出文件路径 | result.txt |
| `--uuid <UUID>` | | VLESS UUID | 默认 UUID |
| `--port <PORT>` | `-p` | 端口号 | 443 |
| `--sni <SNI>` | | SNI (Server Name Indication) | custom.com |
| `--host <HOST>` | | Host 头 | custom.com |
| `--path <PATH>` | | WebSocket 路径 | /custom.com?ed=2560 |

### 线路可选值

| 值 | 说明 |
|----|------|
| `ct` | 中国电信 |
| `cmcc` | 中国移动 |
| `cu` | 中国联通 |
| `cn` | 三网优化（融合三大运营商，适用于多线路优化的 IP） |

## 项目结构

```
src/
├── main.rs              # 程序入口
├── lib.rs               # 核心逻辑
├── cli/
│   └── args.rs          # 命令行参数定义
├── domain/
│   ├── isp.rs           # ISP 枚举定义
│   ├── model.rs         # IpInfo 模型
│   └── error.rs         # 错误类型
└── sources/
    ├── mod.rs           # 数据源聚合器
    ├── cf090227.rs      # cf.090227.xyz API
    └── wetest.rs        # wetest.vip API
```

## 许可证

MIT License
