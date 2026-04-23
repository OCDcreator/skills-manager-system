import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem, AgentTargetSkillEntry } from "../../lib/tauri";

interface AgentGlobalSkillListProps {
  agent: AgentInventoryItem;
  actionKey: string | null;
  canImport: boolean;
  onDelete: (entry: AgentTargetSkillEntry) => void;
  onImport: (entry: AgentTargetSkillEntry, deleteSourceAfterImport: boolean) => void;
  onTakeOver: (entry: AgentTargetSkillEntry) => void;
}

export function AgentGlobalSkillList({
  agent,
  actionKey,
  canImport,
  onDelete,
  onImport,
  onTakeOver,
}: AgentGlobalSkillListProps) {
  const { t } = useTranslation();
  const managedCount = useMemo(
    () => agent.targetSkillEntries.filter((entry) => entry.managed).length,
    [agent.targetSkillEntries],
  );
  const unmanagedCount = agent.targetSkillEntries.length - managedCount;
  const entryActionKey = (entry: AgentTargetSkillEntry, action: string) =>
    `${agent.key}:${entry.entryName}:${action}`;

  return (
    <section className="flex h-full min-h-0 overflow-hidden flex-col rounded-2xl border border-slate-800 bg-slate-950/50 p-4">
      <div className="shrink-0 flex flex-wrap items-start justify-between gap-3">
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
        <div className="skill-markdown-scroll mt-3 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1">
          {agent.targetSkillEntries.map((entry) => (
            <div
              className="rounded-lg border border-slate-800/70 bg-slate-950/70 px-3 py-3 text-xs text-slate-300"
              key={entry.entryName}
            >
              <div className="flex items-start gap-2">
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
                      ? entry.preserveExisting
                        ? "bg-violet-500/10 text-violet-200"
                        : "bg-emerald-500/10 text-emerald-200"
                      : "bg-amber-500/10 text-amber-200"
                  }`}
                >
                  {entry.managed
                    ? entry.preserveExisting
                      ? t("agents.globalSkills.takenOver")
                      : t("agents.globalSkills.managed")
                    : t("agents.globalSkills.unmanaged")}
                </span>
              </div>
              <div className="mt-2 flex flex-wrap gap-2">
                {!entry.hasSkillDocument ? (
                  <span className="rounded bg-slate-800 px-1.5 py-0.5 text-[10px] text-slate-400">
                    {t("agents.globalSkills.noSkillDocument")}
                  </span>
                ) : null}
                {!entry.managed ? (
                  <button
                    className="rounded-lg border border-violet-700/70 px-2 py-1 text-[11px] text-violet-200 disabled:opacity-60"
                    disabled={actionKey === entryActionKey(entry, "take-over")}
                    onClick={() => onTakeOver(entry)}
                    title={t("tooltip.agents.takeOverSkill")}
                    type="button"
                  >
                    {actionKey === entryActionKey(entry, "take-over")
                      ? t("agents.globalSkills.working")
                      : t("agents.globalSkills.takeOver")}
                  </button>
                ) : null}
                {!entry.managed && entry.hasSkillDocument && canImport ? (
                  <>
                    <button
                      className="rounded-lg border border-sky-700/70 px-2 py-1 text-[11px] text-sky-200 disabled:opacity-60"
                      disabled={actionKey === entryActionKey(entry, "import")}
                      onClick={() => onImport(entry, false)}
                      title={t("tooltip.agents.importTargetSkill")}
                      type="button"
                    >
                      {actionKey === entryActionKey(entry, "import")
                        ? t("agents.globalSkills.working")
                        : t("agents.globalSkills.import")}
                    </button>
                    <button
                      className="rounded-lg border border-sky-700/70 px-2 py-1 text-[11px] text-sky-200 disabled:opacity-60"
                      disabled={actionKey === entryActionKey(entry, "import-delete")}
                      onClick={() => onImport(entry, true)}
                      title={t("tooltip.agents.importDeleteTargetSkill")}
                      type="button"
                    >
                      {actionKey === entryActionKey(entry, "import-delete")
                        ? t("agents.globalSkills.working")
                        : t("agents.globalSkills.importAndDelete")}
                    </button>
                  </>
                ) : null}
                <button
                  className="rounded-lg border border-rose-800/70 px-2 py-1 text-[11px] text-rose-200 disabled:opacity-60"
                  disabled={actionKey === entryActionKey(entry, "delete")}
                  onClick={() => onDelete(entry)}
                  title={t("tooltip.agents.deleteTargetSkill")}
                  type="button"
                >
                  {actionKey === entryActionKey(entry, "delete")
                    ? t("agents.globalSkills.working")
                    : t("agents.globalSkills.delete")}
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
