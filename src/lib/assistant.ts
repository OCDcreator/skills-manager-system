import { invoke } from "@tauri-apps/api/core";

export interface AssistantContextStatus {
  projectRoot: string;
  scopeLabel: string;
  indexedDocumentCount: number;
  indexedChunkCount: number;
  lastIndexedAt: string;
  warnings: string[];
}

export interface AssistantSource {
  path: string;
  title: string;
  score: number;
}

export interface AssistantRetrievedChunk {
  path: string;
  title: string;
  heading: string | null;
  excerpt: string;
  score: number;
}

export interface AssistantAnswerResponse {
  answer: string;
  contextSummary: string;
  sources: AssistantSource[];
  retrievedChunks: AssistantRetrievedChunk[];
}

export const getAssistantContextStatus = () =>
  invoke<AssistantContextStatus>("get_assistant_context_status");

export const askProjectAssistant = (question: string) =>
  invoke<AssistantAnswerResponse>("ask_project_assistant", { question });
