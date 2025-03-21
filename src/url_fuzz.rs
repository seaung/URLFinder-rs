use anyhow::Result;
use regex::Regex;
use url::Url;

use crate::crawler::CrawlResult;
use crate::config::Config;

pub struct UrlFuzzer {
    config: Config,
}

impl UrlFuzzer {
    pub fn new(config: Config) -> Self {
        UrlFuzzer { config }
    }

    pub async fn fuzz(&self, results: &[CrawlResult], target_url: &str, domain: Option<&str>) -> Result<Vec<String>> {
        let mut paths = Vec::new();

        // Extract base paths from URLs
        for result in results {
            // Skip non-404 URLs
            if result.status != 404 {
                continue;
            }

            // Extract path up to last directory
            if let Ok(url) = Url::parse(&result.url) {
                if let Some(segments) = url.path_segments() {
                    let path = segments.collect::<Vec<_>>();
                    if path.len() > 1 {
                        let base_path = path[..path.len()-1].join("/");
                        paths.push(format!("{}{}", url.origin().ascii_serialization(), base_path));
                    }
                }
            }

            // Extract domain base path
            if let Ok(re) = Regex::new(r"(https?://[^/]+)/") {
                if let Some(cap) = re.captures(&result.url) {
                    if let Some(base) = cap.get(1) {
                        paths.push(base.as_str().to_string());
                    }
                }
            }
        }

        // Use target URL domain if no domain specified
        let host = if let Some(d) = domain {
            d.to_string()
        } else {
            if let Ok(url) = Url::parse(target_url) {
                url.host_str().unwrap_or("").to_string()
            } else {
                return Ok(Vec::new());
            }
        };

        // Remove duplicates
        paths.sort();
        paths.dedup();

        // Generate fuzz URLs
        let mut fuzz_urls = Vec::new();
        for path in paths {
            for fuzz_path in &self.config.url_fuzz_paths {
                fuzz_urls.push(format!("{}{}", path, fuzz_path));
            }
        }

        Ok(fuzz_urls)
    }
}