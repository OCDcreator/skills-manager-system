use serde::Serialize;

use super::context::AssistantContextBundle;
use super::retrieval::AssistantRetrievedChunk;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantSource {
    pub path: String,
    pub title: String,
    pub score: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssistantAnswerResponse {
    pub answer: String,
    pub context_summary: String,
    pub sources: Vec<AssistantSource>,
    pub retrieved_chunks: Vec<AssistantRetrievedChunk>,
}

fn sources_from_chunks(chunks: &[AssistantRetrievedChunk]) -> Vec<AssistantSource> {
    let mut sources = Vec::new();
    for chunk in chunks {
        if sources.iter().any(|source: &AssistantSource| source.path == chunk.path) {
            continue;
        }

        sources.push(AssistantSource {
            path: chunk.path.clone(),
            title: chunk.title.clone(),
            score: chunk.score,
        });
    }
    sources
}

pub fn build_answer(
    question: &str,
    context: &AssistantContextBundle,
    chunks: Vec<AssistantRetrievedChunk>,
) -> AssistantAnswerResponse {
    if chunks.is_empty() {
        return AssistantAnswerResponse {
            answer: format!(
                "I could not find enough evidence for \"{}\" in the current project docs. \
                 Try asking about AGENTS.md rules, docs content, verification, \
                 architecture, or module documentation.",
                question.trim()
            ),
            context_summary: format!(
                "Scanned {} documents from {}",
                context.status.indexed_document_count, context.status.scope_label
            ),
            sources: Vec::new(),
            retrieved_chunks: Vec::new(),
        };
    }

    let bullets = chunks
        .iter()
        .take(3)
        .map(|chunk| format!("- {}: {}", chunk.path, chunk.excerpt.trim()))
        .collect::<Vec<_>>()
        .join("\n");

    AssistantAnswerResponse {
        answer: format!(
            "I found project documentation related to \"{}\".\n\
             {}\n\n\
             This prototype is using retrieval plus a deterministic summary, \
             not a cloud model yet.",
            question.trim(),
            bullets
        ),
        context_summary: format!(
            "Scanned {} documents and matched {} excerpts from {}",
            context.status.indexed_document_count,
            chunks.len(),
            context.status.scope_label
        ),
        sources: sources_from_chunks(&chunks),
        retrieved_chunks: chunks,
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::{
        AssistantContextBundle, AssistantContextDocument, AssistantContextStatus,
    };
    use super::super::retrieval::retrieve_relevant_chunks;
    use super::build_answer;

    #[test]
    fn assistant_answer_includes_sources_for_verify_question() {
        let context = AssistantContextBundle {
            status: AssistantContextStatus {
                project_root: ".".into(),
                scope_label:
                    "AGENTS.md + docs/**/*.md + package.json + src-tauri/Cargo.toml".into(),
                indexed_document_count: 2,
                indexed_chunk_count: 2,
                last_indexed_at: "2026-04-23T00:00:00Z".into(),
                warnings: Vec::new(),
            },
            documents: vec![AssistantContextDocument {
                path: "AGENTS.md".into(),
                title: "Skills Manager System".into(),
                content: "npm run verify\ncargo check --manifest-path src-tauri/Cargo.toml"
                    .into(),
            }],
        };

        let chunks = retrieve_relevant_chunks("verify project", &context.documents);
        let response = build_answer("verify project", &context, chunks);

        assert!(response.answer.contains("I found project documentation"));
        assert_eq!(response.sources[0].path, "AGENTS.md");
    }
}
