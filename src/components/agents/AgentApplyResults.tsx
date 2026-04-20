import { useTranslation } from "react-i18next";
import type { AgentApplyResult, ApplyAgentSyncResponse } from "../../lib/tauri";

interface AgentApplyResultsProps {
  result: ApplyAgentSyncResponse | null;
}

function statusClass(result: AgentApplyResult) {
  switch (result.status) {
    case "success":
      return "bg-emerald-500/15 text-emerald-200";
    case "partial":
      return "bg-amber-500/15 text-amber-200";
    case "failed":
      return "bg-rose-500/15 text-rose-200";
    default:
      return "bg-slate-700 text-slate-200";
  }
}

export function AgentApplyResults({ result }: AgentApplyResultsProps) {
  const { t } = useTranslation();

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <header className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-slate-100">{t("agents.results.title")}</h3>
        {result ? (
          <span className="text-xs text-slate-500">
            {t("agents.results.enabledSkills", { count: result.enabledSkillCount })}
          </span>
        ) : null}
      </header>

      {!result ? (
        <p className="mt-4 text-sm text-slate-500">{t("agents.results.empty")}</p>
      ) : (
        <div className="mt-4 grid gap-3 lg:grid-cols-3">
          {result.results.map((item) => (
            <article key={item.key} className="rounded-xl border border-slate-800 bg-slate-950 p-4">
              <div className="flex items-center justify-between gap-3">
                <h4 className="font-semibold text-slate-100">{item.displayName}</h4>
                <span className={`rounded-full px-2.5 py-1 text-xs ${statusClass(item)}`}>
                  {t(`agents.status.${item.status}`)}
                </span>
              </div>
              <p className="mt-3 text-sm text-slate-300">{item.message}</p>
              <dl className="mt-3 grid grid-cols-3 gap-2 text-xs text-slate-500">
                <div>
                  <dt>{t("agents.results.written")}</dt>
                  <dd className="text-slate-200">{item.writtenCount}</dd>
                </div>
                <div>
                  <dt>{t("agents.results.removed")}</dt>
                  <dd className="text-slate-200">{item.removedCount}</dd>
                </div>
                <div>
                  <dt>{t("agents.results.conflicts")}</dt>
                  <dd className="text-slate-200">{item.conflictCount}</dd>
                </div>
              </dl>
              <p className="mt-3 break-all text-xs text-slate-600">
                {item.targetDir || t("agents.results.noTarget")}
              </p>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
