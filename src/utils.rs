use anyhow::Result;
use regex::Regex;
use url::Url;

pub fn normalize_url(url: &str, base_url: &str) -> Result<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(url.to_string())
    } else if url.starts_with("//") {
        let base = Url::parse(base_url)?;
        Ok(format!("{}{}", base.scheme(), url))
    } else if url.starts_with('/') {
        let base = Url::parse(base_url)?;
        Ok(format!("{}//{}{}", base.scheme(), base.host_str().unwrap_or(""), url))
    } else {
        let base = Url::parse(base_url)?;
        let path = base.path();
        let parent = if path.ends_with('/') {
            path.to_string()
        } else {
            format!("{}/", path.rsplit('/').nth(1).unwrap_or(""))
        };
        Ok(format!("{}//{}{}{}", base.scheme(), base.host_str().unwrap_or(""), parent, url))
    }
}

pub fn is_domain_match(url: &str, domain_pattern: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(host) = parsed_url.host_str() {
            if let Ok(re) = Regex::new(domain_pattern) {
                return re.is_match(host);
            }
        }
    }
    false
}

pub fn parse_status_codes(status_str: &str) -> Vec<u16> {
    if status_str.to_lowercase() == "all" {
        return vec![];
    }
    
    status_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

pub fn is_status_match(status: u16, filter_status: &[u16]) -> bool {
    filter_status.is_empty() || filter_status.contains(&status)
}