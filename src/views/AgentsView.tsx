import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { AgentApplyResults } from "../components/agents/AgentApplyResults";
import { AgentSyncSummary } from "../components/agents/AgentSyncSummary";
import { AgentTargetCard } from "../components/agents/AgentTargetCard";
import { useAppContext } from "../context/AppContext";

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

  return (
    <div className="space-y-6">
      <AgentSyncSummary
        canApply={canApply}
        enabledAgentCount={enabledAgentCount}
        enabledSkillCount={enabledSkillCount}
        isApplying={isApplyingAgentSync}
        onApply={applyAgentSync}
        repoPath={repoPath}
      />

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
