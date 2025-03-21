use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct State {
    visited_urls: Arc<Mutex<HashSet<String>>>,
    js_urls: Arc<Mutex<HashSet<String>>>,
    fuzz_urls: Arc<Mutex<HashSet<String>>>,
}

impl State {
    pub fn new() -> Self {
        State {
            visited_urls: Arc::new(Mutex::new(HashSet::new())),
            js_urls: Arc::new(Mutex::new(HashSet::new())),
            fuzz_urls: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn add_visited_url(&self, url: String) -> bool {
        let mut urls = self.visited_urls.lock().await;
        urls.insert(url)
    }

    pub async fn is_visited(&self, url: &str) -> bool {
        let urls = self.visited_urls.lock().await;
        urls.contains(url)
    }

    pub async fn add_js_url(&self, url: String) -> bool {
        let mut urls = self.js_urls.lock().await;
        urls.insert(url)
    }

    pub async fn is_js_visited(&self, url: &str) -> bool {
        let urls = self.js_urls.lock().await;
        urls.contains(url)
    }

    pub async fn add_fuzz_url(&self, url: String) -> bool {
        let mut urls = self.fuzz_urls.lock().await;
        urls.insert(url)
    }

    pub async fn is_fuzz_visited(&self, url: &str) -> bool {
        let urls = self.fuzz_urls.lock().await;
        urls.contains(url)
    }

    pub async fn get_visited_count(&self) -> usize {
        let urls = self.visited_urls.lock().await;
        urls.len()
    }

    pub async fn get_js_count(&self) -> usize {
        let urls = self.js_urls.lock().await;
        urls.len()
    }

    pub async fn get_fuzz_count(&self) -> usize {
        let urls = self.fuzz_urls.lock().await;
        urls.len()
    }
}