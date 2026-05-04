import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { AgentInventoryItem, AgentTargetSkillEntry } from "../../lib/tauri";

const multiLinePathClass =
  "block overflow-hidden break-all [display:-webkit-box] [-webkit-box-orient:vertical] [-webkit-line-clamp:2]";

interface AgentGlobalSkillListProps {
  agent: AgentInventoryItem;
  actionKey: string | null;
  canImport: boolean;
  onBatchDelete: (entries: AgentTargetSkillEntry[]) => void;
  onBatchImport: (entries: AgentTargetSkillEntry[], deleteSourceAfterImport: boolean) => void;
  onBatchTakeOver: (entries: AgentTargetSkillEntry[]) => void;
  onDelete: (entry: AgentTargetSkillEntry) => void;
  onImport: (entry: AgentTargetSkillEntry, deleteSourceAfterImport: boolean) => void;
  onTakeOver: (entry: AgentTargetSkillEntry) => void;
}

type TargetFilter = "all" | "managed" | "unmanaged";

export function AgentGlobalSkillList({
  agent,
  actionKey,
  canImport,
  onBatchDelete,
  onBatchImport,
  onBatchTakeOver,
  onDelete,
  onImport,
  onTakeOver,
}: AgentGlobalSkillListProps) {
  const { t } = useTranslation();
  const [activeFilter, setActiveFilter] = useState<TargetFilter>("all");
  const [isSelectionMode, setIsSelectionMode] = useState(false);
  const [selectedEntryNames, setSelectedEntryNames] = useState<string[]>([]);
  const managedCount = useMemo(() => agent.targetSkillEntries.filter((entry) => entry.managed).length, [
    agent.targetSkillEntries,
  ]);
  const unmanagedCount = agent.targetSkillEntries.length - managedCount;
  const filteredEntries = useMemo(
    () =>
      agent.targetSkillEntries.filter((entry) => {
        if (activeFilter === "managed") return entry.managed;
        if (activeFilter === "unmanaged") return !entry.managed;
        return true;
      }),
    [activeFilter, agent.targetSkillEntries],
  );
  const selectedEntryNameSet = useMemo(() => new Set(selectedEntryNames), [selectedEntryNames]);
  const selectedEntries = agent.targetSkillEntries.filter((entry) =>
    selectedEntryNameSet.has(entry.entryName),
  );
  const selectedUnmanagedEntries = selectedEntries.filter((entry) => !entry.managed);
  const selectedImportableEntries = selectedUnmanagedEntries.filter(
    (entry) => entry.hasSkillDocument && canImport,
  );
  const isBatchWorking = actionKey?.startsWith(`${agent.key}:batch:`) ?? false;
  const scrollRef = useRememberedScrollPosition(`agents:global-skills:${agent.key}`);
  const entryActionKey = (entry: AgentTargetSkillEntry, action: string) =>
    `${agent.key}:${entry.entryName}:${action}`;
  const filterButtonClass = (filter: TargetFilter, tone: string) =>
    `rounded-full border px-2 py-1 transition ${
      activeFilter === filter
        ? tone
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;
  const toggleSelectionMode = () => {
    setIsSelectionMode((current) => !current);
    setSelectedEntryNames([]);
  };
  const toggleFilter = (filter: TargetFilter) => {
    setActiveFilter((current) => (current === filter ? "all" : filter));
    setSelectedEntryNames([]);
  };
  const toggleEntrySelection = (entryName: string) => {
    setSelectedEntryNames((current) =>
      current.includes(entryName)
        ? current.filter((candidate) => candidate !== entryName)
        : [...current, entryName],
    );
  };
  const runBatchAction = (
    action: "delete" | "take-over" | "import" | "import-delete",
  ) => {
    if (action === "delete") {
      onBatchDelete(selectedEntries);
    } else if (action === "take-over") {
      onBatchTakeOver(selectedUnmanagedEntries);
    } else {
      onBatchImport(selectedImportableEntries, action === "import-delete");
    }
    setSelectedEntryNames([]);
  };

  useEffect(() => {
    const availableNames = new Set(agent.targetSkillEntries.map((entry) => entry.entryName));
    setSelectedEntryNames((current) => {
      const next = current.filter((entryName) => availableNames.has(entryName));
      return next.length === current.length ? current : next;
    });
  }, [agent.targetSkillEntries]);

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
          <button
            className={filterButtonClass("managed", "border-emerald-500/50 bg-emerald-500/15 text-emerald-100")}
            onClick={() => toggleFilter("managed")}
            type="button"
          >
            {t("agents.globalSkills.managedCount", { count: managedCount })}
          </button>
          <button
            className={filterButtonClass("unmanaged", "border-amber-500/50 bg-amber-500/15 text-amber-100")}
            onClick={() => toggleFilter("unmanaged")}
            type="button"
          >
            {t("agents.globalSkills.unmanagedCount", { count: unmanagedCount })}
          </button>
          <button
            className="rounded-full border border-slate-700 px-2 py-1 text-slate-200 hover:bg-slate-800"
            onClick={toggleSelectionMode}
            type="button"
          >
            {isSelectionMode
              ? t("agents.globalSkills.cancelMultiSelect")
              : t("agents.globalSkills.multiSelect")}
          </button>
          {isSelectionMode ? (
            <>
              <span className="rounded-full bg-slate-800 px-2 py-1 text-slate-300">
                {t("agents.globalSkills.batchSelectedCount", {
                  count: selectedEntryNames.length,
                })}
              </span>
              <button
                className="rounded-full border border-rose-800/70 px-2 py-1 text-rose-200 disabled:opacity-50"
                disabled={selectedEntries.length === 0 || isBatchWorking}
                onClick={() => runBatchAction("delete")}
                type="button"
              >
                {t("agents.globalSkills.batchDelete")}
              </button>
              <button
                className="rounded-full border border-violet-700/70 px-2 py-1 text-violet-200 disabled:opacity-50"
                disabled={selectedUnmanagedEntries.length === 0 || isBatchWorking}
                onClick={() => runBatchAction("take-over")}
                type="button"
              >
                {t("agents.globalSkills.batchTakeOver")}
              </button>
              <button
                className="rounded-full border border-sky-700/70 px-2 py-1 text-sky-200 disabled:opacity-50"
                disabled={selectedImportableEntries.length === 0 || isBatchWorking}
                onClick={() => runBatchAction("import")}
                type="button"
              >
                {t("agents.globalSkills.batchImport")}
              </button>
              <button
                className="rounded-full border border-sky-700/70 px-2 py-1 text-sky-200 disabled:opacity-50"
                disabled={selectedImportableEntries.length === 0 || isBatchWorking}
                onClick={() => runBatchAction("import-delete")}
                type="button"
              >
                {t("agents.globalSkills.batchImportDelete")}
              </button>
            </>
          ) : null}
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
      ) : filteredEntries.length === 0 ? (
        <p className="mt-3 text-xs text-slate-500">{t("agents.globalSkills.noMatchingEntries")}</p>
      ) : (
        <div
          className="skill-markdown-scroll mt-3 min-h-0 flex-1 space-y-1 overflow-y-auto pr-1"
          ref={scrollRef}
        >
          {filteredEntries.map((entry) => {
            const secondaryLabel = entry.relativePath || entry.entryName;
            const showSecondaryLabel = secondaryLabel !== entry.displayName;

            return (
              <div
                className="rounded-lg border border-slate-800/70 bg-slate-950/70 px-3 py-3 text-xs text-slate-300"
                key={entry.entryName}
              >
                <div className="flex items-start gap-2">
                  {isSelectionMode ? (
                    <input
                      aria-label={entry.displayName}
                      checked={selectedEntryNameSet.has(entry.entryName)}
                      className="mt-0.5 h-3.5 w-3.5 rounded border-slate-700 bg-slate-950"
                      onChange={() => toggleEntrySelection(entry.entryName)}
                      type="checkbox"
                    />
                  ) : null}
                  <span className="min-w-0 flex-1">
                    <span className="block truncate font-medium text-slate-200">
                      {entry.displayName}
                    </span>
                    {showSecondaryLabel ? (
                      <span className="block truncate text-[11px] text-slate-500">
                        {secondaryLabel}
                      </span>
                    ) : null}
                    <span className="block truncate text-[10px] text-slate-600">
                      {entry.absolutePath}
                    </span>
                    {entry.entryKind === "symlink" && entry.symlinkTargetPath ? (
                      <span className={`${multiLinePathClass} text-[10px] text-cyan-400/80`}>
                        {t("agents.globalSkills.symlinkTarget", {
                          path: entry.symlinkTargetPath,
                        })}
                      </span>
                    ) : null}
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
            );
          })}
        </div>
      )}
    </section>
  );
}
