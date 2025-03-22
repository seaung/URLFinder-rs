use clap::Parser;
use std::path::PathBuf;

mod banner;
mod cli;
mod config;
mod crawler;
mod output;
mod utils;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    /// 目标URL
    #[arg(short = 'u', long)]
    url: Option<String>,

    /// 批量URL文件路径
    #[arg(short = 'f', long)]
    file: Option<PathBuf>,

    /// 统一处理批量URL结果
    #[arg(short = 'F', long = "ff")]
    unified_file: Option<PathBuf>,

    /// 自定义User-Agent
    #[arg(short = 'a', long)]
    user_agent: Option<String>,

    /// 自定义BaseURL
    #[arg(short = 'b', long)]
    base_url: Option<String>,

    /// Cookie设置
    #[arg(short = 'c', long)]
    cookie: Option<String>,

    /// 指定域名(支持正则)
    #[arg(short = 'd', long)]
    domain: Option<String>,

    /// 配置文件路径
    #[arg(short = 'i', long)]
    config: Option<PathBuf>,

    /// 抓取模式: 1=正常, 2=深入, 3=安全深入
    #[arg(short = 'm', long, default_value = "1")]
    mode: u8,

    /// 最大抓取数量
    #[arg(long = "max")]
    max_count: Option<usize>,

    /// 结果输出路径
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// 状态码过滤
    #[arg(short = 's', long)]
    status: Option<String>,

    /// 线程数
    #[arg(short = 't', long, default_value = "50")]
    threads: usize,

    /// 超时时间(秒)
    #[arg(long = "time", default_value = "5")]
    timeout: u64,

    /// 代理设置
    #[arg(short = 'x', long)]
    proxy: Option<String>,

    /// 404链接Fuzz模式
    #[arg(short = 'z', long)]
    fuzz: Option<u8>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 显示banner
    banner::show_banner();

    // 解析命令行参数
    let cli = Cli::parse();
    let cli_runner = cli::CliRunner::new(cli);

    // 验证命令行参数
    cli_runner.validate()?;

    // 获取配置
    let config = if let Some(config_path) = cli_runner.get_config_path() {
        config::Config::load(&config_path)?
    } else {
        config::Config::default()
    };

    // 创建爬虫实例
    let crawler = crawler::Crawler::new(
        config,
        cli_runner.get_threads(),
        cli_runner.get_timeout(),
        cli_runner.get_mode(),
        cli_runner.get_max_count(),
        cli_runner.get_base_url(),
        cli_runner.get_user_agent(),
        cli_runner.get_cookie(),
        cli_runner.get_proxy(),
        cli_runner.get_fuzz_mode(),
    )?;

    // 获取目标URL列表
    let urls = cli_runner.get_urls()?;

    // 创建输出目录
    let output_path = cli_runner.get_output_path().unwrap_or_else(|| PathBuf::from("output"));
    std::fs::create_dir_all(&output_path)?;

    // 创建输出处理器
    let output_writer = output::OutputWriter::new(output_path);

    // 存储爬取结果
    let mut results = Vec::new();

    // 解析状态码过滤器
    let status_filter = cli_runner.get_status_filter()
        .map(|s| utils::parse_status_codes(&s))
        .unwrap_or_default();

    // 获取域名过滤器
    let domain_filter = cli_runner.get_domain_filter();

    // 爬取URL
    for url in urls {
        match crawler.crawl(&url, 1).await {
            Ok(result) => {
                // 检查状态码和域名是否匹配
                if !utils::is_status_match(result.status, &status_filter) {
                    continue;
                }

                if let Some(domain_pattern) = &domain_filter {
                    if !utils::is_domain_match(&result.url, domain_pattern) {
                        continue;
                    }
                }

                let output_result = output::OutputResult {
                    url: result.url,
                    status: result.status,
                    content_type: result.content_type,
                    urls: result.urls,
                    js_urls: result.js_urls,
                    sensitive_info: result.sensitive_info,
                };
                results.push(output_result);
            }
            Err(e) => {
                eprintln!("Error crawling {}: {}", url, e);
            }
        }
    }

    // 输出结果
    output_writer.write_json(&results)?;
    output_writer.write_csv(&results)?;
    output_writer.write_html(&results)?;

    println!("扫描完成，共处理 {} 个URL", results.len());
    Ok(())
}
