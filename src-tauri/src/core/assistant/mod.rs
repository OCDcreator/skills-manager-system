pub mod chunking;
pub mod context;
pub mod response;
pub mod retrieval;

use anyhow::{bail, Result};

pub use context::{
    load_context_from_root, load_workspace_context, AssistantContextBundle,
    AssistantContextDocument, AssistantContextStatus,
};
pub use response::{build_answer, AssistantAnswerResponse, AssistantSource};
pub use retrieval::{retrieve_relevant_chunks, AssistantRetrievedChunk};

pub fn ask_workspace_question(question: &str) -> Result<AssistantAnswerResponse> {
    if question.trim().is_empty() {
        bail!("Question is required");
    }

    let context = load_workspace_context()?;
    let chunks = retrieve_relevant_chunks(question, &context.documents);
    Ok(build_answer(question, &context, chunks))
}
