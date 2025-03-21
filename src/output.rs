use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct OutputResult {
    pub url: String,
    pub status: u16,
    pub content_type: String,
    pub urls: Vec<String>,
    pub js_urls: Vec<String>,
    pub sensitive_info: Vec<String>,
}

pub struct OutputWriter {
    output_path: PathBuf,
}

impl OutputWriter {
    pub fn new(output_path: PathBuf) -> Self {
        OutputWriter { output_path }
    }

    pub fn write_json(&self, results: &[OutputResult]) -> Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        let mut file = File::create(self.output_path.join("result.json"))?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn write_csv(&self, results: &[OutputResult]) -> Result<()> {
        let mut wtr = csv::Writer::from_path(self.output_path.join("result.csv"))?;

        for result in results {
            wtr.write_record(&[
                &result.url,
                &result.status.to_string(),
                &result.content_type,
                &result.urls.join(", "),
                &result.js_urls.join(", "),
                &result.sensitive_info.join(", "),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn write_html(&self, results: &[OutputResult]) -> Result<()> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string(
            "report",
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>URLFinder Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        tr:nth-child(even) { background-color: #f9f9f9; }
    </style>
</head>
<body>
    <h1>URLFinder Scan Results</h1>
    <table>
        <tr>
            <th>URL</th>
            <th>Status</th>
            <th>Content Type</th>
            <th>Found URLs</th>
            <th>JS URLs</th>
            <th>Sensitive Info</th>
        </tr>
        {{#each results}}
        <tr>
            <td>{{url}}</td>
            <td>{{status}}</td>
            <td>{{content_type}}</td>
            <td>{{join urls ", "}}</td>
            <td>{{join js_urls ", "}}</td>
            <td>{{join sensitive_info ", "}}</td>
        </tr>
        {{/each}}
    </table>
</body>
</html>"#,
        )?;

        let html = handlebars.render("report", &serde_json::json!({ "results": results }))?;
        let mut file = File::create(self.output_path.join("result.html"))?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }
}