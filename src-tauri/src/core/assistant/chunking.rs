#[derive(Debug, Clone)]
pub struct DocumentChunk {
    pub heading: Option<String>,
    pub body: String,
}

pub fn split_into_chunks(content: &str) -> Vec<DocumentChunk> {
    let mut chunks: Vec<DocumentChunk> = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ")
            || trimmed.starts_with("## ")
            || trimmed.starts_with("### ")
        {
            if !current_lines.is_empty() {
                let body = current_lines.join("\n");
                if !body.trim().is_empty() {
                    chunks.push(DocumentChunk {
                        heading: current_heading.clone(),
                        body,
                    });
                }
                current_lines.clear();
            }
            current_heading = Some(
                trimmed
                    .trim_start_matches('#')
                    .trim()
                    .to_string(),
            );
        } else {
            current_lines.push(line);
        }
    }

    if !current_lines.is_empty() {
        let body = current_lines.join("\n");
        if !body.trim().is_empty() {
            chunks.push(DocumentChunk {
                heading: current_heading,
                body,
            });
        }
    }

    if chunks.is_empty() {
        let body = content.trim().to_string();
        if !body.is_empty() {
            chunks.push(DocumentChunk {
                heading: None,
                body,
            });
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunking_splits_at_markdown_headings() {
        let content = "# Title\n\nintro\n\n## Section A\nbody a\n\n## Section B\nbody b";
        let chunks = split_into_chunks(content);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].heading.as_deref(), Some("Title"));
        assert!(chunks[0].body.contains("intro"));
        assert_eq!(chunks[1].heading.as_deref(), Some("Section A"));
        assert!(chunks[1].body.contains("body a"));
        assert_eq!(chunks[2].heading.as_deref(), Some("Section B"));
    }

    #[test]
    fn chunking_falls_back_to_full_content_without_headings() {
        let content = "just some text\nno headings here";
        let chunks = split_into_chunks(content);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].heading.is_none());
        assert!(chunks[0].body.contains("just some text"));
    }

    #[test]
    fn chunking_later_section_preserves_heading_and_body() {
        let content = [
            "# Guide",
            "",
            "## Introduction",
            "weather dashboard content",
            "",
            "## Verification",
            "npm run verify",
        ]
        .join("\n");

        let chunks = split_into_chunks(&content);
        let verify_chunk = chunks
            .iter()
            .find(|c| c.heading.as_deref() == Some("Verification"))
            .expect("should have Verification section");

        assert!(verify_chunk.body.contains("npm run verify"));
        assert!(!verify_chunk.body.contains("weather"));
    }
}
