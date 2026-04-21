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
  const basePanelClassName =
    "min-w-0 rounded-2xl border border-slate-800 bg-slate-900 p-6";
  const dockedPanelClassName =
    "min-[1280px]:sticky min-[1280px]:top-8 min-[1280px]:max-h-[calc(100vh-7rem)]";

  if (!skill) {
    return (
      <aside className={`${basePanelClassName} ${dockedPanelClassName} text-sm text-slate-400`}>
        {t("skills.selectPrompt")}
      </aside>
    );
  }

  return (
    <aside className={`${basePanelClassName} ${dockedPanelClassName} flex flex-col gap-4 min-[1280px]:overflow-hidden`}>
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

      <div className="min-h-0 min-w-0 flex-1 overflow-hidden rounded-xl border border-slate-800 bg-slate-950 p-4">
        {document ? (
          <article className="prose prose-invert h-full min-w-0 max-w-none overflow-x-auto overflow-y-auto break-words pr-2 prose-headings:scroll-mt-20 prose-pre:overflow-x-auto prose-pre:bg-slate-900 prose-code:text-sky-200">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{document.content}</ReactMarkdown>
          </article>
        ) : (
          <p className="text-sm text-slate-400">{t("skills.detail.loadingDocument")}</p>
        )}
      </div>
    </aside>
  );
}
