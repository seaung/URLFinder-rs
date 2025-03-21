use anyhow::Result;
use regex::Regex;
use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::utils::{normalize_url, is_domain_match, is_status_match};
use crate::config::Config;
use crate::Cli;
use crate::utils;
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Crawler {
    client: Client,
    config: Arc<Config>,
    semaphore: Arc<Semaphore>,
    cli: Cli,
}

#[derive(Debug)]
pub struct CrawlResult {
    pub url: String,
    pub status: u16,
    pub content_type: String,
    pub urls: Vec<String>,
    pub js_urls: Vec<String>,
    pub sensitive_info: Vec<String>,
    pub source: String,
}

impl Crawler {
    pub fn get_mode(&self) -> u8 {
        self.cli.mode
    }

    pub fn get_fuzz_mode(&self) -> Option<u8> {
        self.cli.fuzz
    }

    pub fn crawled_count(&self) -> usize {
        0 // TODO: Implement crawled count tracking
    }

    pub fn new(
        config: Config,
        threads: usize,
        timeout: u64,
        _mode: u8,
        _max_count: Option<usize>,
        _base_url: Option<String>,
        user_agent: Option<String>,
        cookie: Option<String>,
        proxy: Option<String>,
        _fuzz_mode: Option<u8>,
    ) -> Result<Self> {
        let mut client_builder = ClientBuilder::new();
        
        // 设置User-Agent
        if let Some(ua) = user_agent {
            client_builder = client_builder.user_agent(ua);
        } else {
            client_builder = client_builder.user_agent(&config.headers.user_agent);
        }

        // 设置Cookie
        if let Some(cookie_str) = cookie {
            client_builder = client_builder.default_headers(reqwest::header::HeaderMap::from_iter(vec![
                (reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_str)?)
            ]));
        }

        // 设置代理
        if let Some(proxy_str) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(&proxy_str)?);
        }

        // 设置超时
        client_builder = client_builder
            .timeout(std::time::Duration::from_secs(timeout))
            .cookie_store(true);

        let client = client_builder.build()?;

        Ok(Crawler {
            client,
            config: Arc::new(config),
            semaphore: Arc::new(Semaphore::new(threads)),
            cli: Cli::parse(),
        })
    }

    pub async fn crawl(&self, target_url: &str, _depth: u8) -> Result<CrawlResult> {
        let mode = self.get_mode();
        let _fuzz_mode = self.get_fuzz_mode();

        // 检查域名是否匹配
        if let Some(domain) = &self.cli.domain {
            if !is_domain_match(target_url, domain) {
                return Ok(CrawlResult {
                    url: target_url.to_string(),
                    status: 0,
                    content_type: String::new(),
                    urls: Vec::new(),
                    js_urls: Vec::new(),
                    sensitive_info: Vec::new(),
                    source: String::new(),
                });
            }
        }

        // 检查是否达到最大爬取数量
        if let Some(max_count) = self.cli.max_count {
            if self.crawled_count() >= max_count {
                return Ok(CrawlResult {
                    url: target_url.to_string(),
                    status: 0,
                    content_type: String::new(),
                    urls: Vec::new(),
                    js_urls: Vec::new(),
                    sensitive_info: Vec::new(),
                    source: String::new(),
                });
            }
        }
        let _permit = self.semaphore.acquire().await?;

        let resp = self.client.get(target_url).send().await?;
        let status = resp.status().as_u16();
        let content_type = resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("").to_string();

        let content = resp.text().await?;

        // 检查状态码是否匹配
        if let Some(status_str) = &self.cli.status {
            if !is_status_match(status, &utils::parse_status_codes(status_str)) {
                return Ok(CrawlResult {
                    url: target_url.to_string(),
                    status,
                    content_type,
                    urls: Vec::new(),
                    js_urls: Vec::new(),
                    sensitive_info: Vec::new(),
                    source: content,
                });
            }
        }

        let mut urls = Vec::new();
        let mut js_urls = Vec::new();
        let mut sensitive_info = Vec::new();

        // 根据模式处理URL和敏感信息
        match mode {
            // 正常模式：只处理页面中的URL
            1 => {
                for pattern in &self.config.url_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            let url = url.as_str().to_string();
                            if !self.is_filtered(&url) {
                                urls.push(self.normalize_url(&url, target_url)?);    
                            }
                        }
                    }
                }
            }
            // 深入模式：处理页面URL和JS中的URL
            2 => {
                // 处理页面URL
                for pattern in &self.config.url_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            let url = url.as_str().to_string();
                            if !self.is_filtered(&url) {
                                urls.push(self.normalize_url(&url, target_url)?);    
                            }
                        }
                    }
                }

                // 处理JS URL
                for pattern in &self.config.js_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            let url = url.as_str().to_string();
                            if !self.is_js_filtered(&url) {
                                js_urls.push(self.normalize_url(&url, target_url)?);    
                            }
                        }
                    }
                }
            }
            // 安全深入模式：处理页面URL、JS URL和敏感信息
            3 => {
                // 处理页面URL
                for pattern in &self.config.url_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            let url = url.as_str().to_string();
                            if !self.is_filtered(&url) {
                                urls.push(self.normalize_url(&url, target_url)?);    
                            }
                        }
                    }
                }

                // 处理JS URL
                for pattern in &self.config.js_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(url) = cap.get(0) {
                            let url = url.as_str().to_string();
                            if !self.is_js_filtered(&url) {
                                js_urls.push(self.normalize_url(&url, target_url)?);    
                            }
                        }
                    }
                }

                // 处理敏感信息
                for pattern in &self.config.sensitive_patterns {
                    let re = Regex::new(pattern)?;
                    for cap in re.captures_iter(&content) {
                        if let Some(info) = cap.get(0) {
                            sensitive_info.push(info.as_str().to_string());
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(CrawlResult {
            url: target_url.to_string(),
            status,
            content_type,
            urls,
            js_urls,
            sensitive_info,
            source: content,
        })
    }

    fn is_filtered(&self, url: &str) -> bool {
        self.config.url_filters.iter().any(|pattern| {
            if let Ok(re) = Regex::new(pattern) {
                re.is_match(url)
            } else {
                false
            }
        })
    }

    fn is_js_filtered(&self, url: &str) -> bool {
        self.config.js_filters.iter().any(|pattern| {
            if let Ok(re) = Regex::new(pattern) {
                re.is_match(url)
            } else {
                false
            }
        })
    }

    fn normalize_url(&self, url: &str, base_url: &str) -> Result<String> {
        // 如果设置了基础URL，优先使用基础URL
        let effective_base = if let Some(base) = &self.cli.base_url {
            base
        } else {
            base_url
        };
        normalize_url(url, effective_base)
    }


}