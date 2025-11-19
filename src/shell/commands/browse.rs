use crate::shell::Shell;
use crate::html_renderer::HtmlRenderer;

pub fn execute(_shell: &mut Shell, args: &[&str]) {
    if args.is_empty() {
        println!("Usage: browse <url>");
        return;
    }

    let url = args[0];
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                let content_type = response.headers()
                    .get("content-type")
                    .and_then(|ct| ct.to_str().ok())
                    .unwrap_or("")
                    .to_string();

                match response.text() {
                    Ok(text) => {
                        let is_html = content_type.contains("text/html") ||
                                     text.contains("<html") ||
                                     text.contains("<!DOCTYPE html");

                        if is_html {
                            // Render HTML content
                            let renderer = HtmlRenderer::new().with_width(80);
                            let rendered = renderer.render(&text);
                            let links = renderer.render_links(&text);

                            println!("🌐 Page from: {}", url);
                            println!("{}", "═".repeat(80));
                            println!("{}", rendered);
                            println!("{}", "═".repeat(80));

                            if !links.is_empty() {
                                println!("\n🔗 Links found:");
                                for (i, (text, href)) in links.iter().enumerate() {
                                    println!("  {}. {} -> {}", i + 1, text, href);
                                }
                            }
                        } else {
                            // Display plain text content
                            let preview = if text.len() > 2000 {
                                format!("{}... (truncated)", &text[..2000])
                            } else {
                                text
                            };
                            println!("📄 Content from {}:", url);
                            println!("{}", "═".repeat(80));
                            println!("{}", preview);
                            println!("{}", "═".repeat(80));
                        }
                    }
                    Err(e) => println!("❌ Error reading response: {}", e),
                }
            } else {
                println!("❌ HTTP {}: {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown"));
            }
        }
        Err(e) => println!("❌ Error fetching {}: {}", url, e),
    }
}