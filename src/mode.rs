use anyhow::Result;
use crate::crawler::CrawlResult;
use crate::js_fuzz::JsFuzzer;
use crate::url_fuzz::UrlFuzzer;
use crate::state::State;

pub struct Mode {
    state: State,
    js_fuzzer: JsFuzzer,
    url_fuzzer: UrlFuzzer,
}

impl Mode {
    pub fn new(state: State, js_fuzzer: JsFuzzer, url_fuzzer: UrlFuzzer) -> Self {
        Mode {
            state,
            js_fuzzer,
            url_fuzzer,
        }
    }

    pub async fn process(&self, result: &CrawlResult, mode: u8, fuzz_mode: Option<u8>) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        // 添加当前URL到已访问集合
        self.state.add_visited_url(result.url.clone()).await;

        // 根据模式处理URL
        match mode {
            // 正常模式：只处理页面中的URL
            1 => {
                for url in &result.urls {
                    if !self.state.is_visited(url).await {
                        urls.push(url.clone());
                    }
                }
            }
            // 深入模式：处理页面URL和JS中的URL
            2 => {
                // 处理页面URL
                for url in &result.urls {
                    if !self.state.is_visited(url).await {
                        urls.push(url.clone());
                    }
                }

                // 处理JS URL
                for js_url in &result.js_urls {
                    if !self.state.is_js_visited(js_url).await {
                        self.state.add_js_url(js_url.clone()).await;
                        urls.push(js_url.clone());
                    }
                }
            }
            // 安全深入模式：处理页面URL、JS URL和敏感信息
            3 => {
                // 处理页面URL
                for url in &result.urls {
                    if !self.state.is_visited(url).await {
                        urls.push(url.clone());
                    }
                }

                // 处理JS URL
                for js_url in &result.js_urls {
                    if !self.state.is_js_visited(js_url).await {
                        self.state.add_js_url(js_url.clone()).await;
                        urls.push(js_url.clone());
                    }
                }

                // 处理敏感信息
                if !result.sensitive_info.is_empty() {
                    println!("[!] Found sensitive information in {}: {:?}", result.url, result.sensitive_info);
                }
            }
            _ => {}
        }

        // 处理Fuzz模式
        if let Some(fuzz) = fuzz_mode {
            match fuzz {
                // 基础Fuzz：只处理404页面
                1 => {
                    if result.status == 404 {
                        let fuzz_urls = self.url_fuzzer.fuzz(&[result.clone()], &result.url, None).await?;
                        for url in fuzz_urls {
                            if !self.state.is_fuzz_visited(&url).await {
                                self.state.add_fuzz_url(url.clone()).await;
                                urls.push(url);
                            }
                        }
                    }
                }
                // JS Fuzz：处理JS文件中的路径
                2 => {
                    let js_fuzz_urls = self.js_fuzzer.fuzz(&[result.clone()]).await?;
                    for url in js_fuzz_urls {
                        if !self.state.is_fuzz_visited(&url).await {
                            self.state.add_fuzz_url(url.clone()).await;
                            urls.push(url);
                        }
                    }
                }
                // 组合Fuzz：同时进行URL和JS Fuzz
                3 => {
                    // URL Fuzz
                    if result.status == 404 {
                        let fuzz_urls = self.url_fuzzer.fuzz(&[result.clone()], &result.url, None).await?;
                        for url in fuzz_urls {
                            if !self.state.is_fuzz_visited(&url).await {
                                self.state.add_fuzz_url(url.clone()).await;
                                urls.push(url);
                            }
                        }
                    }

                    // JS Fuzz
                    let js_fuzz_urls = self.js_fuzzer.fuzz(&[result.clone()]).await?;
                    for url in js_fuzz_urls {
                        if !self.state.is_fuzz_visited(&url).await {
                            self.state.add_fuzz_url(url.clone()).await;
                            urls.push(url);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(urls)
    }
}