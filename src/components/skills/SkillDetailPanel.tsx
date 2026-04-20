import { useTranslation } from "react-i18next";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { SkillDocument, SkillSummary } from "../../lib/tauri";

interface SkillDetailPanelProps {
  skill: SkillSummary | null;
  document: SkillDocument | null;
  isEnabled: boolean;
}

export function SkillDetailPanel({ document, isEnabled, skill }: SkillDetailPanelProps) {
  const { t } = useTranslation();

  if (!skill) {
    return (
      <aside className="rounded-2xl border border-slate-800 bg-slate-900 p-6 text-sm text-slate-400">
        {t("skills.selectPrompt")}
      </aside>
    );
  }

  return (
    <aside className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-6">
      <div className="space-y-2">
        <h3 className="text-xl font-semibold text-slate-100">{skill.name}</h3>
        <p className="text-sm text-slate-400">
          {skill.description || t("skills.noDescription")}
        </p>
        <dl className="grid grid-cols-[96px_1fr] gap-2 text-sm text-slate-300">
          <dt className="text-slate-500">{t("skills.detail.sourceLabel")}</dt>
          <dd>
            {skill.sourceType === "custom"
              ? t("skills.source.custom")
              : t("skills.source.external")}
          </dd>
          <dt className="text-slate-500">{t("skills.detail.statusLabel")}</dt>
          <dd>{isEnabled ? t("skills.status.enabled") : t("skills.status.disabled")}</dd>
          <dt className="text-slate-500">{t("skills.detail.pathLabel")}</dt>
          <dd className="break-all">{skill.relativePath}</dd>
        </dl>
      </div>

      <div className="rounded-xl border border-slate-800 bg-slate-950 p-4">
        {document ? (
          <article className="prose prose-invert max-w-none prose-pre:bg-slate-900 prose-code:text-sky-200">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{document.content}</ReactMarkdown>
          </article>
        ) : (
          <p className="text-sm text-slate-400">{t("skills.detail.loadingDocument")}</p>
        )}
      </div>
    </aside>
  );
}
