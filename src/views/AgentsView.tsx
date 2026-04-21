import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { AgentApplyResults } from "../components/agents/AgentApplyResults";
import { AgentSyncSummary } from "../components/agents/AgentSyncSummary";
import { AgentTargetCard } from "../components/agents/AgentTargetCard";
import { useAppContext } from "../context/AppContext";
import * as api from "../lib/tauri";
import type { AgentSyncMode } from "../lib/tauri";

function messageFrom(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

export function AgentsView() {
  const { t } = useTranslation();
  const {
    agentInventory,
    applyAgentSync,
    clearAgentPathOverride,
    disabledSkillIds,
    isApplyingAgentSync,
    isLoadingAgents,
    lastAgentApplyResult,
    repoPath,
    scanResult,
    setAgentEnabled,
    setAgentPathOverride,
    updatingAgentKey,
  } = useAppContext();
  const [syncMode, setSyncMode] = useState<AgentSyncMode>("copy");
  const [isSavingSyncMode, setIsSavingSyncMode] = useState(false);
  const [syncModeError, setSyncModeError] = useState<string | null>(null);
  const disabledSkillIdSet = useMemo(
    () => new Set(disabledSkillIds),
    [disabledSkillIds],
  );
  const enabledSkillCount = useMemo(
    () => scanResult.skills.filter((skill) => !disabledSkillIdSet.has(skill.id)).length,
    [scanResult.skills, disabledSkillIdSet],
  );
  const enabledAgentCount = useMemo(
    () => agentInventory.filter((agent) => agent.enabled).length,
    [agentInventory],
  );
  const canApply = Boolean(repoPath) && enabledAgentCount > 0;

  useEffect(() => {
    let isActive = true;
    void (async () => {
      try {
        const savedMode = await api.getAgentSyncMode();
        if (!isActive) return;
        setSyncMode(savedMode);
        setSyncModeError(null);
      } catch (error) {
        if (!isActive) return;
        setSyncModeError(messageFrom(error));
      }
    })();
    return () => {
      isActive = false;
    };
  }, []);

  const handleSyncModeChange = async (nextMode: AgentSyncMode) => {
    const previousMode = syncMode;
    setSyncMode(nextMode);
    setIsSavingSyncMode(true);
    try {
      await api.setAgentSyncMode(nextMode);
      setSyncModeError(null);
    } catch (error) {
      setSyncMode(previousMode);
      setSyncModeError(messageFrom(error));
    } finally {
      setIsSavingSyncMode(false);
    }
  };

  return (
    <div className="space-y-6">
      <AgentSyncSummary
        canApply={canApply}
        enabledAgentCount={enabledAgentCount}
        enabledSkillCount={enabledSkillCount}
        isApplying={isApplyingAgentSync}
        isSavingMode={isSavingSyncMode}
        onApply={() => applyAgentSync(syncMode)}
        onSyncModeChange={handleSyncModeChange}
        repoPath={repoPath}
        syncMode={syncMode}
      />

      {syncModeError ? (
        <div className="rounded-2xl border border-amber-900/60 bg-amber-950/40 px-4 py-3 text-sm text-amber-200">
          {syncModeError}
        </div>
      ) : null}

      <section className="space-y-4">
        <div>
          <h2 className="text-lg font-semibold text-slate-100">{t("agents.targets.title")}</h2>
          <p className="mt-1 text-sm text-slate-500">{t("agents.targets.description")}</p>
        </div>

        {isLoadingAgents ? (
          <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5 text-sm text-slate-400">
            {t("agents.targets.loading")}
          </div>
        ) : (
          <div className="grid gap-4 xl:grid-cols-3">
            {agentInventory.map((agent) => (
              <AgentTargetCard
                agent={agent}
                isUpdating={updatingAgentKey === agent.key}
                key={agent.key}
                onClearPathOverride={clearAgentPathOverride}
                onSavePathOverride={setAgentPathOverride}
                onToggleEnabled={setAgentEnabled}
              />
            ))}
          </div>
        )}
      </section>

      <AgentApplyResults result={lastAgentApplyResult} />
    </div>
  );
}
