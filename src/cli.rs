use std::path::PathBuf;
use anyhow::Result;
use crate::Cli;

pub struct CliRunner {
    cli: Cli,
}

impl CliRunner {
    pub fn new(cli: Cli) -> Self {
        CliRunner { cli }
    }

    pub fn validate(&self) -> Result<()> {
        // 检查必要的参数
        if self.cli.url.is_none() && self.cli.file.is_none() && self.cli.unified_file.is_none() {
            anyhow::bail!("必须指定目标URL(-u)或URL文件(-f/-ff)")
        }

        // 检查模式参数
        if !(1..=3).contains(&self.cli.mode) {
            anyhow::bail!("抓取模式(-m)必须为1-3之间的数字")
        }

        // 检查Fuzz模式参数
        if let Some(fuzz) = self.cli.fuzz {
            if !(1..=3).contains(&fuzz) {
                anyhow::bail!("Fuzz模式(-z)必须为1-3之间的数字")
            }
        }

        Ok(())
    }

    pub fn get_urls(&self) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        // 处理单个URL
        if let Some(url) = &self.cli.url {
            urls.push(url.clone());
        }

        // 处理URL文件
        if let Some(file_path) = &self.cli.file {
            let content = std::fs::read_to_string(file_path)?;
            urls.extend(content.lines().map(String::from));
        }

        // 处理统一URL文件
        if let Some(file_path) = &self.cli.unified_file {
            let content = std::fs::read_to_string(file_path)?;
            urls.extend(content.lines().map(String::from));
        }

        Ok(urls)
    }

    pub fn get_config_path(&self) -> Option<PathBuf> {
        self.cli.config.clone()
    }

    pub fn get_output_path(&self) -> Option<PathBuf> {
        self.cli.output.clone()
    }

    pub fn get_threads(&self) -> usize {
        self.cli.threads
    }

    pub fn get_timeout(&self) -> u64 {
        self.cli.timeout
    }

    pub fn get_mode(&self) -> u8 {
        self.cli.mode
    }

    pub fn get_max_count(&self) -> Option<usize> {
        self.cli.max_count
    }

    pub fn get_status_filter(&self) -> Option<String> {
        self.cli.status.clone()
    }

    pub fn get_domain_filter(&self) -> Option<String> {
        self.cli.domain.clone()
    }

    pub fn get_base_url(&self) -> Option<String> {
        self.cli.base_url.clone()
    }

    pub fn get_user_agent(&self) -> Option<String> {
        self.cli.user_agent.clone()
    }

    pub fn get_cookie(&self) -> Option<String> {
        self.cli.cookie.clone()
    }

    pub fn get_proxy(&self) -> Option<String> {
        self.cli.proxy.clone()
    }

    pub fn get_fuzz_mode(&self) -> Option<u8> {
        self.cli.fuzz
    }
}