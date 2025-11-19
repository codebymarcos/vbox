use scraper::{Html, Selector};

pub struct HtmlRenderer {
    max_width: usize,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        HtmlRenderer { max_width: 80 }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    pub fn render(&self, html: &str) -> String {
        let document = Html::parse_document(html);

        let mut output = String::new();

        // Extract title
        if let Ok(title_selector) = Selector::parse("title") {
            if let Some(title_element) = document.select(&title_selector).next() {
                let title = title_element.text().collect::<String>().trim().to_string();
                if !title.is_empty() {
                    output.push_str(&format!("=== {} ===\n\n", title));
                }
            }
        }

        // Extract main content (body or main content areas)
        let content_selectors = [
            "body",
            "main",
            "article",
            ".content",
            "#content",
            ".main",
            "#main"
        ];

        let mut content_found = false;
        for selector_str in &content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = self.extract_text(&element);
                    if !text.trim().is_empty() {
                        output.push_str(&text);
                        content_found = true;
                        break;
                    }
                }
            }
        }

        // Fallback: extract all text if no structured content found
        if !content_found {
            output.push_str(&self.extract_text(&document.root_element()));
        }

        // Clean up the output
        self.cleanup_output(output)
    }

    fn extract_text(&self, element: &scraper::ElementRef) -> String {
        let mut text = String::new();
        let in_link = false;
        let mut link_text = String::new();

        for node in element.text() {
            let trimmed = node.trim();
            if !trimmed.is_empty() {
                if in_link {
                    link_text.push_str(trimmed);
                    link_text.push(' ');
                } else {
                    text.push_str(trimmed);
                    text.push(' ');
                }
            }
        }

        // Handle links
        if !link_text.is_empty() {
            text.push_str(&format!("[LINK: {}]", link_text.trim()));
        }

        text
    }

    fn cleanup_output(&self, mut output: String) -> String {
        // Remove excessive whitespace
        output = output.split_whitespace().collect::<Vec<&str>>().join(" ");

        // Handle line breaks and paragraphs
        output = output.replace(". ", ".\n\n");
        output = output.replace("? ", "?\n\n");
        output = output.replace("! ", "!\n\n");

        // Wrap long lines
        let mut wrapped = String::new();
        for line in output.lines() {
            if line.len() > self.max_width {
                let words: Vec<&str> = line.split_whitespace().collect();
                let mut current_line = String::new();

                for word in words {
                    if current_line.len() + word.len() + 1 > self.max_width {
                        if !current_line.is_empty() {
                            wrapped.push_str(&current_line);
                            wrapped.push('\n');
                            current_line = word.to_string();
                        } else {
                            wrapped.push_str(word);
                            wrapped.push('\n');
                        }
                    } else {
                        if !current_line.is_empty() {
                            current_line.push(' ');
                        }
                        current_line.push_str(word);
                    }
                }

                if !current_line.is_empty() {
                    wrapped.push_str(&current_line);
                    wrapped.push('\n');
                }
            } else {
                wrapped.push_str(line);
                wrapped.push('\n');
            }
        }

        wrapped.trim().to_string()
    }

    pub fn render_links(&self, html: &str) -> Vec<(String, String)> {
        let document = Html::parse_document(html);
        let mut links = Vec::new();

        if let Ok(link_selector) = Selector::parse("a[href]") {
            for element in document.select(&link_selector) {
                if let Some(href) = element.value().attr("href") {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && !href.is_empty() {
                        links.push((text, href.to_string()));
                    }
                }
            }
        }

        links
    }
}