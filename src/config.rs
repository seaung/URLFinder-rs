use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// 请求头配置
    #[serde(default)]
    pub headers: Headers,
    /// URL匹配规则
    #[serde(default)]
    pub url_patterns: Vec<String>,
    /// JS匹配规则
    #[serde(default)]
    pub js_patterns: Vec<String>,
    /// 敏感信息匹配规则
    #[serde(default)]
    pub sensitive_patterns: Vec<String>,
    /// URL过滤规则
    #[serde(default)]
    pub url_filters: Vec<String>,
    /// JS过滤规则
    #[serde(default)]
    pub js_filters: Vec<String>,
    /// URL递归深度
    #[serde(default = "default_url_depth")]
    pub url_depth: u8,
    /// JS递归深度
    #[serde(default = "default_js_depth")]
    pub js_depth: u8,
    /// URL Fuzz路径
    #[serde(default)]
    pub url_fuzz_paths: Vec<String>,
    /// JS Fuzz路径
    #[serde(default)]
    pub js_fuzz_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Headers {
    #[serde(default)]
    pub user_agent: String,
    #[serde(default)]
    pub cookie: String,
    #[serde(default)]
    pub accept: String,
    #[serde(default)]
    pub accept_language: String,
    #[serde(default)]
    pub accept_encoding: String,
}

fn default_url_depth() -> u8 { 1 }
fn default_js_depth() -> u8 { 3 }

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Config {
            headers: Headers {
                user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
                cookie: String::new(),
                accept: String::from("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
                accept_language: String::from("en-US,en;q=0.5"),
                accept_encoding: String::from("gzip, deflate"),
            },
            url_patterns: vec![
                r#"https?://[\w\-\.]+(:\d+)?(/[\w\-\./?%&=]*)?"#.to_string(),
                r#"(/[\w\-\./?%&=]+)+"#.to_string(),
            ],
            js_patterns: vec![
                r#"https?://[\w\-\.]+(:\d+)?[\w\-\./?%&=]*\.js"#.to_string(),
                r#"(/[\w\-\./?%&=]*\.js)+"#.to_string(),
            ],
            sensitive_patterns: vec![
                r#"(password|secret|token|key)\s*[=:]\s*['"][^'"]+['"]"#.to_string(),
                r#"(api|v1|v2|v3)/[\w\-\./?%&=]+"#.to_string(),
            ],
            url_filters: vec![
                r#"\.(css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot|mp3|mp4|avi|swf)$"#.to_string(),
            ],
            js_filters: vec![
                r#"^(https?:)?//cdn\."#.to_string(),
            ],
            url_depth: default_url_depth(),
            js_depth: default_js_depth(),
            url_fuzz_paths: vec![
                "/admin".to_string(),
                "/api".to_string(),
                "/v1".to_string(),
                "/v2".to_string(),
                "/swagger".to_string(),
                "/docs".to_string(),
            ],
            js_fuzz_paths: vec![
                "config.js".to_string(),
                "api.js".to_string(),
                "main.js".to_string(),
                "app.js".to_string(),
                "index.js".to_string(),
            ],
        }
    }
}