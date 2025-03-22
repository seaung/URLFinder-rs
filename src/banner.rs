use colored::*;

pub fn show_banner() {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    
    println!("{}", " ".repeat(50).on_blue());
    println!("{}", format!(" URLFinder-rs v{} ", version).bold().white().on_blue());
    println!("{}", format!(" By: {} ", authors).bold().white().on_blue());
    println!("{}", " ".repeat(50).on_blue());
    println!("{}", " A fast and comprehensive tool for extracting URLs ".yellow());
    println!("{}", " and sensitive information from web pages ".yellow());
    println!();
    println!("{}", " Features: ".green().bold());
    println!("{}", " - Multi-threaded crawling ".cyan());
    println!("{}", " - JavaScript URL extraction ".cyan());
    println!("{}", " - Sensitive information detection ".cyan());
    println!("{}", " - Multiple output formats (JSON, CSV, HTML) ".cyan());
    println!();
}