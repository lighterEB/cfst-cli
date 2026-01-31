# CFST-CLI - Cloudflare 优选 IP 获取工具
一个用 Rust 编写的 Cloudflare 优选 IP 互获取工具，用于获取优选 IP 并生成 Vless 代理配置。

## 说明

本工具不进行 IP 测速和优选，而是从第三方优选 API 获取已经过测速的优选 IP，
然后为这些 IP 生成可直接使用的 Vless 代理配置。

## 功能特性

- 支持三大运营商（电信/移动/联通）优选 IP 获取
- 自动生成 Vless+WebSocket+TLS 配置
- 支持自定义 Vless 参数（UUID、端口、SNI等）

## 安装

```bash
git clone https://github.com/lighterEB/cfst-cli.git
cd cfst
cargo build --release
```
## 使用示例
### 基础使用：
```bash
# 获取电信和联通各10个优选IP
cargo run -- -i ct -i cu -c 10

# 指定输出文件
cargo run -- -i ct -c 20 -o my_nodes.txt
```

### 自定义 Vless 配置：
```bash
cargo run -- -i ct -c 10 \
  --uuid "your-custom-uuid" \
  --port 8443 \
  --sni "your-domain.com" \
  --host "your-domain.com" \
  --path "/your-custom-path"
```

### 查看所有参数：
```bash
cargo run -- --help
```

## 命令行参数
- `-i, --isp <ISP>` - 运营商（ct/cmcc/cu，可多选）【必填】

- `-c, --count <COUNT>` - 获取IP数量（默认：10）

- `-o, --output <OUTPUT>` - 输出文件路径（默认：result.txt）

- `--uuid <UUID>` - Vless UUID

- `-p, --port <PORT>` - 端口号（默认：443）

- `--sni <SNI>` - SNI

- `--host <HOST>` - Host

- `--path <PATH>` - WebSocket 路径

## 许可证
MIT License