import { FormEvent, KeyboardEvent, useState } from "react";
import { Send, Sparkles, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  askProjectAssistant,
  type AssistantAnswerResponse,
  type AssistantContextStatus,
  type AssistantSource,
} from "../../lib/assistant";
import { AssistantMarkdown } from "./AssistantMarkdown";
import { ProjectAssistantSourceRail } from "./ProjectAssistantSourceRail";

interface ProjectAssistantPanelProps {
  isLoadingStatus: boolean;
  onClose: () => void;
  status: AssistantContextStatus | null;
  statusError: string | null;
}

interface AssistantMessage {
  id: string;
  role: "assistant" | "user";
  content: string;
}

function buildWelcomeMessage(t: (key: string) => string): AssistantMessage {
  return {
    id: "assistant-welcome",
    role: "assistant",
    content: t("assistant.welcomeMessage"),
  };
}

export function ProjectAssistantPanel({
  isLoadingStatus,
  onClose,
  status,
  statusError,
}: ProjectAssistantPanelProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState("");
  const [isAsking, setIsAsking] = useState(false);
  const [askError, setAskError] = useState<string | null>(null);
  const [messages, setMessages] = useState<AssistantMessage[]>([
    buildWelcomeMessage(t),
  ]);
  const [sources, setSources] = useState<AssistantSource[]>([]);

  const canSubmit = draft.trim().length > 0 && !isAsking;

  async function submitQuestion() {
    const question = draft.trim();
    if (!question || isAsking) {
      return;
    }

    setDraft("");
    setAskError(null);
    setIsAsking(true);
    setMessages((current) => [
      ...current,
      { id: crypto.randomUUID(), role: "user", content: question },
    ]);

    try {
      const response: AssistantAnswerResponse =
        await askProjectAssistant(question);
      setMessages((current) => [
        ...current,
        {
          id: crypto.randomUUID(),
          role: "assistant",
          content: response.answer,
        },
      ]);
      setSources(response.sources);
    } catch (error: unknown) {
      setAskError(error instanceof Error ? error.message : String(error));
    } finally {
      setIsAsking(false);
    }
  }

  function handleFormSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void submitQuestion();
  }

  function handleKeyDown(event: KeyboardEvent<HTMLTextAreaElement>) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      void submitQuestion();
    }
  }

  return (
    <section className="fixed bottom-24 right-6 z-50 flex h-[min(760px,calc(100vh-8rem))] w-[min(860px,calc(100vw-3rem))] flex-col overflow-hidden rounded-[28px] border border-sky-400/30 bg-slate-950/95 shadow-2xl shadow-slate-950/60 backdrop-blur">
      <header className="flex shrink-0 items-center justify-between border-b border-slate-800 px-5 py-4">
        <div>
          <h2 className="text-base font-semibold text-slate-100">
            {t("assistant.title")}
          </h2>
          <p className="text-sm text-slate-400">
            {t("assistant.subtitle")}
          </p>
        </div>
        <button
          aria-label={t("assistant.closePanel")}
          className="rounded-full p-2 text-slate-400 hover:bg-slate-900 hover:text-slate-100"
          onClick={onClose}
          title={t("assistant.closePanel")}
        >
          <X className="h-5 w-5" />
        </button>
      </header>

      <div className="grid flex-1 gap-4 overflow-hidden p-5 md:grid-cols-[minmax(0,1fr)_220px] md:items-start">
        <div className="flex h-full min-h-0 self-stretch flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900/60">
          <div className="flex-1 space-y-3 overflow-y-auto px-4 py-4">
            {messages.map((message) =>
              message.role === "assistant" ? (
                <article
                  key={message.id}
                  className="mr-6 rounded-2xl bg-slate-900 px-4 py-3 text-slate-100"
                >
                  <AssistantMarkdown content={message.content} />
                </article>
              ) : (
                <article
                  key={message.id}
                  className="ml-6 whitespace-pre-wrap rounded-2xl bg-sky-500/20 px-4 py-3 text-sm text-sky-50"
                >
                  {message.content}
                </article>
              ),
            )}
            {askError ? (
              <p className="text-sm text-rose-300">{askError}</p>
            ) : null}
            {isAsking ? (
              <div className="mr-6 flex items-center gap-2 rounded-2xl bg-slate-900 px-4 py-3 text-sm text-slate-300">
                <Sparkles className="h-4 w-4 animate-pulse" />
                {t("assistant.answerLoading")}
              </div>
            ) : null}
          </div>

          <form
            className="shrink-0 border-t border-slate-800 p-4"
            onSubmit={handleFormSubmit}
          >
            <textarea
              className="min-h-24 w-full resize-none rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none ring-0 placeholder:text-slate-500 focus:border-sky-400"
              disabled={isAsking}
              onChange={(event) => setDraft(event.target.value)}
              onKeyDown={handleKeyDown}
              placeholder={t("assistant.inputPlaceholder")}
              value={draft}
            />
            <div className="mt-3 flex items-center justify-between">
              <p className="text-xs text-slate-500">
                {t("assistant.inputHint")}
              </p>
              <button
                aria-label={t("assistant.send")}
                className="inline-flex items-center gap-2 rounded-full bg-sky-400 px-4 py-2 text-sm font-medium text-slate-950 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400"
                disabled={!canSubmit}
                type="submit"
              >
                <Send className="h-4 w-4" />
                {t("assistant.send")}
              </button>
            </div>
          </form>
        </div>

        <ProjectAssistantSourceRail
          isLoadingStatus={isLoadingStatus}
          sources={sources}
          status={status}
          statusError={statusError}
        />
      </div>
    </section>
  );
}
