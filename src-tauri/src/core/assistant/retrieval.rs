use std::collections::BTreeSet;

use serde::Serialize;

use super::chunking::split_into_chunks;
use super::context::AssistantContextDocument;

const MIN_SCORE: usize = 2;
const MAX_RESULTS: usize = 4;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantRetrievedChunk {
    pub path: String,
    pub title: String,
    pub heading: Option<String>,
    pub excerpt: String,
    pub score: usize,
}

fn is_cjk(character: char) -> bool {
    matches!(
        character,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{2F800}'..='\u{2FA1F}'
    )
}

fn cjk_bigrams(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().filter(|c| is_cjk(*c)).collect();
    let mut bigrams = Vec::new();
    for window in chars.windows(2) {
        bigrams.push(format!("{}{}", window[0], window[1]));
    }
    if chars.len() == 1 {
        bigrams.push(chars[0].to_string());
    }
    bigrams
}

fn tokenize(value: &str) -> BTreeSet<String> {
    let mut tokens = BTreeSet::new();

    let mut current_alphanumeric = String::new();
    let mut current_cjk = String::new();

    for character in value.chars() {
        if character.is_alphanumeric() && !is_cjk(character) {
            current_cjk.clear();
            current_alphanumeric.push(character);
        } else if is_cjk(character) {
            if !current_alphanumeric.is_empty() {
                let token = current_alphanumeric.trim().to_lowercase();
                if token.len() >= 3 {
                    tokens.insert(token);
                }
                current_alphanumeric.clear();
            }
            current_cjk.push(character);
        } else {
            if !current_alphanumeric.is_empty() {
                let token = current_alphanumeric.trim().to_lowercase();
                if token.len() >= 3 {
                    tokens.insert(token);
                }
                current_alphanumeric.clear();
            }
            if !current_cjk.is_empty() {
                for bigram in cjk_bigrams(&current_cjk) {
                    tokens.insert(bigram);
                }
                current_cjk.clear();
            }
        }
    }

    if !current_alphanumeric.is_empty() {
        let token = current_alphanumeric.trim().to_lowercase();
        if token.len() >= 3 {
            tokens.insert(token);
        }
    }
    if !current_cjk.is_empty() {
        for bigram in cjk_bigrams(&current_cjk) {
            tokens.insert(bigram);
        }
    }

    tokens
}

fn excerpt_for_match(content: &str, tokens: &BTreeSet<String>) -> String {
    for token in tokens {
        let token_lower = token.to_lowercase();
        if let Some(line) = content
            .lines()
            .find(|line| line.to_lowercase().contains(&token_lower))
        {
            return line.trim().to_string();
        }
    }
    content
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

pub fn retrieve_relevant_chunks(
    question: &str,
    documents: &[AssistantContextDocument],
) -> Vec<AssistantRetrievedChunk> {
    let tokens = tokenize(question);
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<AssistantRetrievedChunk> = Vec::new();

    for document in documents {
        let path_lower = document.path.to_lowercase();
        let title_lower = document.title.to_lowercase();
        let chunks = split_into_chunks(&document.content);

        for chunk in &chunks {
            let body_lower = chunk.body.to_lowercase();
            let heading_lower = chunk
                .heading
                .as_deref()
                .unwrap_or("")
                .to_lowercase();

            let mut score = 0;
            for token in &tokens {
                if path_lower.contains(token) {
                    score += 5;
                }
                if title_lower.contains(token) {
                    score += 4;
                }
                if heading_lower.contains(token) {
                    score += 3;
                }
                if body_lower.contains(token) {
                    score += 2;
                }
            }

            if score < MIN_SCORE {
                continue;
            }

            results.push(AssistantRetrievedChunk {
                path: document.path.clone(),
                title: document.title.clone(),
                heading: chunk.heading.clone(),
                excerpt: excerpt_for_match(&chunk.body, &tokens),
                score,
            });
        }
    }

    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.path.cmp(&right.path))
    });
    results.truncate(MAX_RESULTS);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_documents() -> Vec<AssistantContextDocument> {
        vec![
            AssistantContextDocument {
                path: "AGENTS.md".into(),
                title: "Skills Manager System".into(),
                content: "## \u{5f00}\u{53d1}\u{547d}\u{4ee4}\nnpm run verify\ncargo check --manifest-path src-tauri/Cargo.toml".into(),
            },
            AssistantContextDocument {
                path: "docs/README.md".into(),
                title: "Docs".into(),
                content: "# Docs\n\u{6a21}\u{5757}\u{6587}\u{6863}\u{8986}\u{76d6}\u{5fc5}\u{987b}\u{548c}\u{6e90}\u{7801}\u{540c}\u{6b65}".into(),
            },
        ]
    }

    #[test]
    fn assistant_retrieval_prefers_verify_related_chunks() {
        let chunks =
            retrieve_relevant_chunks("how do I verify this project", &sample_documents());

        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].path, "AGENTS.md");
        assert!(chunks[0].excerpt.contains("npm run verify"));
    }

    #[test]
    fn assistant_retrieval_returns_empty_for_irrelevant_questions() {
        let chunks =
            retrieve_relevant_chunks("what is the weather today", &sample_documents());

        assert!(chunks.is_empty());
    }

    #[test]
    fn assistant_retrieval_uses_later_heading_section_excerpt() {
        let documents = vec![AssistantContextDocument {
            path: "AGENTS.md".into(),
            title: "Project Guide".into(),
            content: [
                "# Project Guide",
                "",
                "## Introduction",
                "This project is a weather dashboard for checking forecasts.",
                "",
                "## Verification",
                "npm run verify",
                "cargo check --manifest-path src-tauri/Cargo.toml",
            ]
            .join("\n"),
        }];

        let chunks = retrieve_relevant_chunks("how to verify", &documents);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].heading.as_deref(), Some("Verification"));
        assert!(
            chunks[0].excerpt.contains("npm run verify"),
            "excerpt should come from the matching section, got: {}",
            chunks[0].excerpt
        );
        assert!(
            !chunks[0].excerpt.contains("weather"),
            "excerpt should not contain text from the unrelated introduction section"
        );
    }

    #[test]
    fn assistant_retrieval_chinese_question_matches_verify() {
        let documents = vec![AssistantContextDocument {
            path: "AGENTS.md".into(),
            title: "Skills Manager System".into(),
            content: "## \u{9a8c}\u{8bc1}\u{65b9}\u{5f0f}\nnpm run verify\n\u{63d0}\u{4ea4}\u{524d}\u{81f3}\u{5c11}\u{8fd0}\u{884c} npm run verify".into(),
        }];

        let chunks = retrieve_relevant_chunks("\u{600e}\u{4e48}\u{9a8c}\u{8bc1}\u{9879}\u{76ee}", &documents);

        assert!(
            !chunks.is_empty(),
            "Chinese question should match documents containing \u{9a8c}\u{8bc1}"
        );
        assert_eq!(chunks[0].path, "AGENTS.md");
        assert!(
            chunks[0].excerpt.contains("npm run verify"),
            "Chinese query excerpt should contain npm run verify, got: {}",
            chunks[0].excerpt
        );
    }

    #[test]
    fn tokenize_handles_cjk_and_english() {
        let tokens = tokenize("\u{600e}\u{4e48}\u{9a8c}\u{8bc1}\u{9879}\u{76ee} verify project");

        assert!(
            tokens.contains("verify"),
            "English tokens should be present"
        );
        assert!(
            tokens.contains("project"),
            "English tokens should be present"
        );
        assert!(
            tokens.contains("\u{9a8c}\u{8bc1}"),
            "CJK bigrams should include \u{9a8c}\u{8bc1}"
        );
        assert!(
            tokens.contains("\u{9879}\u{76ee}"),
            "CJK bigrams should include \u{9879}\u{76ee}"
        );
    }
}
