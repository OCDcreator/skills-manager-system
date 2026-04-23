import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem } from "../../lib/tauri";

interface AgentGlobalSkillListProps {
  agent: AgentInventoryItem;
}

export function AgentGlobalSkillList({ agent }: AgentGlobalSkillListProps) {
  const { t } = useTranslation();
  const managedCount = useMemo(
    () => agent.targetSkillEntries.filter((entry) => entry.managed).length,
    [agent.targetSkillEntries],
  );
  const unmanagedCount = agent.targetSkillEntries.length - managedCount;

  return (
    <section className="rounded-xl border border-slate-800 bg-slate-950/50 p-3">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h4 className="text-xs font-semibold uppercase tracking-wide text-slate-500">
            {t("agents.globalSkills.title")}
          </h4>
          <p className="mt-1 text-xs leading-5 text-slate-500">
            {t("agents.globalSkills.description")}
          </p>
        </div>
        <div className="flex flex-wrap gap-2 text-[11px]">
          <span className="rounded-full bg-emerald-500/10 px-2 py-1 text-emerald-200">
            {t("agents.globalSkills.managedCount", { count: managedCount })}
          </span>
          <span className="rounded-full bg-amber-500/10 px-2 py-1 text-amber-200">
            {t("agents.globalSkills.unmanagedCount", { count: unmanagedCount })}
          </span>
        </div>
      </div>

      {!agent.effectiveSkillsDir ? (
        <p className="mt-3 text-xs text-slate-500">{t("agents.globalSkills.noTarget")}</p>
      ) : agent.targetSkillScanError ? (
        <p className="mt-3 rounded-lg border border-amber-900/60 bg-amber-950/30 px-3 py-2 text-xs text-amber-200">
          {agent.targetSkillScanError}
        </p>
      ) : agent.targetSkillEntries.length === 0 ? (
        <p className="mt-3 text-xs text-slate-500">{t("agents.globalSkills.empty")}</p>
      ) : (
        <div className="skill-markdown-scroll mt-3 max-h-44 space-y-1 overflow-y-auto pr-1">
          {agent.targetSkillEntries.map((entry) => (
            <div
              className="flex items-start gap-2 rounded-lg border border-slate-800/70 bg-slate-950/70 px-2 py-2 text-xs text-slate-300"
              key={entry.entryName}
            >
              <span className="min-w-0 flex-1">
                <span className="block truncate font-medium text-slate-200">
                  {entry.displayName}
                </span>
                <span className="block truncate text-[11px] text-slate-500">
                  {entry.relativePath || entry.entryName}
                </span>
                <span className="block truncate text-[10px] text-slate-600">
                  {entry.absolutePath}
                </span>
              </span>
              <span
                className={`shrink-0 rounded px-1.5 py-0.5 text-[10px] ${
                  entry.managed
                    ? "bg-emerald-500/10 text-emerald-200"
                    : "bg-amber-500/10 text-amber-200"
                }`}
              >
                {entry.managed
                  ? t("agents.globalSkills.managed")
                  : t("agents.globalSkills.unmanaged")}
              </span>
              {!entry.hasSkillDocument ? (
                <span className="shrink-0 rounded bg-slate-800 px-1.5 py-0.5 text-[10px] text-slate-400">
                  {t("agents.globalSkills.noSkillDocument")}
                </span>
              ) : null}
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
