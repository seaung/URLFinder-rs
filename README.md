# URLFinder-rs
Rust实现的高性能URL提取工具，用于从网页中提取URL和敏感信息。

## 功能特点

- 多线程爬取，高效处理大量URL
- 自动提取JavaScript文件中的URL
- 敏感信息检测与提取
- 多种输出格式支持（JSON、CSV、HTML）
- 支持URL Fuzzing和JS路径Fuzzing
- 灵活的过滤规则和匹配模式

## 安装方法

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/seaung/URLFinder-rs.git
cd URLFinder-rs

# 编译项目
cargo build --release

# 运行程序
./target/release/urlfinder-rs --help
```

## 基本使用

### 命令行参数

```
➜  URLFinder-rs git:(main) ./target/release/urlfinder-rs --help

 URLFinder-rs v0.1.0
 By: seaung

 A fast and comprehensive tool for extracting URLs
 and sensitive information from web pages

Usage: urlfinder-rs [OPTIONS]

Options:
  -u, --url <URL>                目标URL
  -f, --file <FILE>              批量URL文件路径
  -F, --ff <UNIFIED_FILE>        统一处理批量URL结果
  -a, --user-agent <USER_AGENT>  自定义User-Agent
  -b, --base-url <BASE_URL>      自定义BaseURL
  -c, --cookie <COOKIE>          Cookie设置
  -d, --domain <DOMAIN>          指定域名(支持正则)
  -i, --config <CONFIG>          配置文件路径
  -m, --mode <MODE>              抓取模式: 1=正常, 2=深入, 3=安全深入 [default: 1]
      --max <MAX_COUNT>          最大抓取数量
  -o, --output <OUTPUT>          结果输出路径
  -s, --status <STATUS>          状态码过滤
  -t, --threads <THREADS>        线程数 [default: 50]
      --time <TIMEOUT>           超时时间(秒) [default: 5]
  -x, --proxy <PROXY>            代理设置
  -z, --fuzz <FUZZ>              404链接Fuzz模式
  -h, --help                     Print help
  -V, --version                  Print version
```

### 使用场景示例

#### 1. 扫描单个网站

```bash
# 基本扫描
urlfinder-rs -u https://example.com

# 使用深入模式扫描并输出结果到文件
urlfinder-rs -u https://example.com -m 2 -o results/
```

#### 2. 批量扫描多个网站

```bash
# 从文件读取URL列表进行扫描
urlfinder-rs -f urls.txt -t 100 --time 10

# 统一处理多个URL的结果
urlfinder-rs -F urls.txt -o combined_results/
```

#### 3. 使用代理和自定义请求头

```bash
# 使用代理
urlfinder-rs -u https://example.com -x http://127.0.0.1:8080

# 设置自定义User-Agent和Cookie
urlfinder-rs -u https://example.com -a "Mozilla/5.0" -c "session=abc123"
```

#### 4. 使用Fuzz模式发现隐藏资源

```bash
# 对404页面进行基础Fuzz
urlfinder-rs -u https://example.com -z 1

# 对JS文件进行Fuzz
urlfinder-rs -u https://example.com -z 2

# 综合Fuzz模式
urlfinder-rs -u https://example.com -z 3
```

## 抓取模式说明

- **模式1（正常）**：只处理页面中的URL，适合快速扫描
- **模式2（深入）**：处理页面URL和JS中的URL，提供更全面的发现
- **模式3（安全深入）**：处理页面URL、JS URL和敏感信息，适合安全审计

## 配置文件

可以通过YAML格式的配置文件自定义工具行为：

```yaml
# config.yaml 示例
headers:
  user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
  cookie: "session=abc123"
  accept: "text/html,application/xhtml+xml,application/xml;q=0.9"
  accept_language: "zh-CN,zh;q=0.9,en;q=0.8"

# URL匹配规则（正则表达式）
url_patterns:
  - "https?://[\w\-\.]+(:\d+)?(/[\w\-\./?%&=]*)?" 
  - "(/[\w\-\./?%&=]+)+"

# JS匹配规则
js_patterns:
  - "https?://[\w\-\.]+(:\d+)?[\w\-\./?%&=]*\.js"
  - "(/[\w\-\./?%&=]*\.js)+"

# 敏感信息匹配规则
sensitive_patterns:
  - "(password|secret|token|key)\s*[=:]\s*['\\\