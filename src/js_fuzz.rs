use anyhow::Result;
use regex::Regex;
use url::Url;

use crate::crawler::CrawlResult;
use crate::config::Config;

pub struct JsFuzzer {
    config: Config,
}

impl JsFuzzer {
    pub fn new(config: Config) -> Self {
        JsFuzzer { config }
    }

    pub async fn fuzz(&self, results: &[CrawlResult]) -> Result<Vec<String>> {
        let mut js_paths = Vec::new();

        // Extract base paths from JS URLs
        for result in results {
            for js_url in &result.js_urls {
                // Extract path up to last directory
                if let Ok(url) = Url::parse(js_url) {
                    if let Some(segments) = url.path_segments() {
                        let path = segments.collect::<Vec<_>>();
                        if path.len() > 1 {
                            let base_path = path[..path.len()-1].join("/");
                            js_paths.push(format!("{}{}", url.origin().ascii_serialization(), base_path));
                        }
                    }
                }

                // Extract domain base path
                if let Ok(re) = Regex::new(r"(https?://[^/]+)/") {
                    if let Some(cap) = re.captures(js_url) {
                        if let Some(base) = cap.get(1) {
                            js_paths.push(base.as_str().to_string());
                        }
                    }
                }
            }
        }

        // Remove duplicates
        js_paths.sort();
        js_paths.dedup();

        // Generate fuzz URLs
        let mut fuzz_urls = Vec::new();
        for path in js_paths {
            for fuzz_path in &self.config.js_fuzz_paths {
                fuzz_urls.push(format!("{}{}", path, fuzz_path));
            }
        }

        Ok(fuzz_urls)
    }
}