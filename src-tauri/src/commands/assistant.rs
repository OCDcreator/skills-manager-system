use crate::core::assistant::{
    ask_workspace_question, load_workspace_context, AssistantAnswerResponse, AssistantContextStatus,
};

#[tauri::command]
pub fn get_assistant_context_status() -> Result<AssistantContextStatus, String> {
    load_workspace_context()
        .map(|bundle| bundle.status)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn ask_project_assistant(question: String) -> Result<AssistantAnswerResponse, String> {
    ask_workspace_question(&question).map_err(|error| error.to_string())
}
