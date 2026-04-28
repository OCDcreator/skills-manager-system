import { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { AgentApplyResults } from "../components/agents/AgentApplyResults";
import { AgentExternalVariantPanel } from "../components/agents/AgentExternalVariantPanel";
import { AgentFloatingNav } from "../components/agents/AgentFloatingNav";
import { AgentOrderModal } from "../components/agents/AgentOrderModal";
import { AgentSyncSummary } from "../components/agents/AgentSyncSummary";
import { AgentTargetsSection } from "../components/agents/AgentTargetsSection";
import { useAppContext } from "../context/AppContext";
import {
  draftFromAgent,
  draftToConfig,
  isAgentDraftDirty,
  type AgentConfigDraft,
} from "../lib/agent-selection";
import { useAgentTargetActions } from "../lib/agent-target-actions";
import * as scenesApi from "../lib/scenes";
import type { SceneConfigSnapshot } from "../lib/scenes";
import * as api from "../lib/tauri";
import type { AgentSyncMode } from "../lib/tauri";

function messageFrom(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

function buildDraftMap(agentInventory: ReturnType<typeof useAppContext>["agentInventory"]) {
  return Object.fromEntries(agentInventory.map((agent) => [agent.key, draftFromAgent(agent)]));
}

export function AgentsView() {
  const { t } = useTranslation();
  const {
    agentInventory,
    agentOrder,
    applyAgentSync,
    disabledSkillIds,
    externalSources,
    isApplyingAgentSync,
    isLoadingAgents,
    isLoadingExternalSources,
    lastAgentApplyResult,
    importExternalVariant,
    registerNavigationGuard,
    repairExternalImport,
    refreshSkills,
    refreshAgents,
    repoPath,
    saveAgentConfiguration,
    saveAgentOrder,
    scanResult,
    sortedAgentInventory,
    updateExternalImport,
    updatingAgentKey,
    updatingExternalImportId,
    updatingExternalSourceId,
  } = useAppContext();
  const [syncMode, setSyncMode] = useState<AgentSyncMode>("copy");
  const [isSavingSyncMode, setIsSavingSyncMode] = useState(false);
  const [syncModeError, setSyncModeError] = useState<string | null>(null);
  const [sceneConfig, setSceneConfig] = useState<SceneConfigSnapshot | null>(null);
  const [drafts, setDrafts] = useState<Record<string, AgentConfigDraft>>({});
  const [savingAgentKey, setSavingAgentKey] = useState<string | null>(null);
  const [isSavingAll, setIsSavingAll] = useState(false);
  const [isOrderModalOpen, setIsOrderModalOpen] = useState(false);
  const [isSavingAgentOrder, setIsSavingAgentOrder] = useState(false);

  const disabledSkillIdSet = useMemo(() => new Set(disabledSkillIds), [disabledSkillIds]);
  const availableSkillCount = useMemo(
    () => scanResult.skills.filter((skill) => !disabledSkillIdSet.has(skill.id)).length,
    [scanResult.skills, disabledSkillIdSet],
  );
  const sceneList = useMemo(() => Object.values(sceneConfig?.scenes ?? {}), [sceneConfig]);

  useEffect(() => {
    setDrafts((current) =>
      Object.fromEntries(
        agentInventory.map((agent) => {
          const existingDraft = current[agent.key];
          if (existingDraft && isAgentDraftDirty(agent, existingDraft)) {
            return [agent.key, existingDraft];
          }
          return [agent.key, draftFromAgent(agent)];
        }),
      ),
    );
  }, [agentInventory]);

  useEffect(() => {
    let isActive = true;
    void (async () => {
      try {
        const [savedMode, snapshot] = await Promise.all([
          api.getAgentSyncMode(),
          scenesApi.getSceneConfig(),
        ]);
        if (!isActive) return;
        setSyncMode(savedMode);
        setSceneConfig(snapshot);
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

  const dirtyAgentKeys = useMemo(
    () =>
      agentInventory
        .filter((agent) => isAgentDraftDirty(agent, drafts[agent.key]))
        .map((agent) => agent.key),
    [agentInventory, drafts],
  );
  const hasDirtyDrafts = dirtyAgentKeys.length > 0;
  const enabledAgentCount = useMemo(
    () =>
      agentInventory.filter((agent) => (drafts[agent.key] ?? draftFromAgent(agent)).enabled)
        .length,
    [agentInventory, drafts],
  );
  const canApply = Boolean(repoPath) && enabledAgentCount > 0;
  const {
    actionId: targetActionId,
    actionNotice,
    clearActionNotice,
    deleteTargetSkill,
    importTargetSkill,
    takeOverTargetSkill,
  } = useAgentTargetActions({
    repoPath,
    refreshAgents,
    refreshSkills,
    setError: setSyncModeError,
    t,
  });

  const discardDrafts = useCallback(() => {
    setDrafts(buildDraftMap(agentInventory));
  }, [agentInventory]);

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

  const handleSaveAgent = useCallback(async (key: string) => {
    const draft = drafts[key];
    if (!draft) return;

    setSavingAgentKey(key);
    try {
      await saveAgentConfiguration(draftToConfig(draft));
      await applyAgentSync(syncMode, key);
      await refreshAgents();
      setSyncModeError(null);
      clearActionNotice();
    } finally {
      setSavingAgentKey(null);
    }
  }, [applyAgentSync, clearActionNotice, drafts, refreshAgents, saveAgentConfiguration, syncMode]);

  const handleSaveAll = useCallback(async () => {
    if (dirtyAgentKeys.length === 0) return;

    setIsSavingAll(true);
    try {
      for (const key of dirtyAgentKeys) {
        const draft = drafts[key];
        if (draft) {
          await saveAgentConfiguration(draftToConfig(draft));
        }
      }
      await applyAgentSync(syncMode);
      await refreshAgents();
      setSyncModeError(null);
      clearActionNotice();
    } finally {
      setIsSavingAll(false);
    }
  }, [applyAgentSync, clearActionNotice, dirtyAgentKeys, drafts, refreshAgents, saveAgentConfiguration, syncMode]);

  const handleApplyAll = useCallback(async () => {
    if (hasDirtyDrafts) {
      await handleSaveAll();
      return;
    }
    clearActionNotice();
    await applyAgentSync(syncMode);
  }, [applyAgentSync, clearActionNotice, handleSaveAll, hasDirtyDrafts, syncMode]);

  const handleSaveAgentOrder = useCallback(async (nextOrder: api.AgentKey[]) => {
    setIsSavingAgentOrder(true);
    try {
      await saveAgentOrder(nextOrder);
      setIsOrderModalOpen(false);
    } finally {
      setIsSavingAgentOrder(false);
    }
  }, [saveAgentOrder]);

  useEffect(
    () =>
      registerNavigationGuard({
        view: "agents",
        isDirty: () => hasDirtyDrafts,
        save: handleSaveAll,
        discard: discardDrafts,
      }),
    [discardDrafts, handleSaveAll, hasDirtyDrafts, registerNavigationGuard],
  );

  return (
    <div className="space-y-6 pr-12">
      <AgentFloatingNav agents={sortedAgentInventory} onOpenOrderModal={() => setIsOrderModalOpen(true)} />

      <div id="agent-sync-overview" className="scroll-mt-8">
        <AgentSyncSummary
          canApply={canApply}
          enabledAgentCount={enabledAgentCount}
          enabledSkillCount={availableSkillCount}
          isApplying={isApplyingAgentSync || isSavingAll}
          isSavingMode={isSavingSyncMode}
          onApply={handleApplyAll}
          onSyncModeChange={handleSyncModeChange}
          repoPath={repoPath}
          syncMode={syncMode}
        />
      </div>

      {hasDirtyDrafts ? (
        <div className="flex flex-wrap items-center justify-between gap-3 rounded-2xl border border-sky-900/60 bg-sky-950/30 px-4 py-3 text-sm text-sky-200">
          <span>{t("agents.unsavedBanner", { count: dirtyAgentKeys.length })}</span>
          <div className="flex gap-2">
            <button
              className="rounded-lg border border-slate-700 px-3 py-2 text-xs text-slate-200"
              disabled={isSavingAll}
              onClick={discardDrafts}
              type="button"
            >
              {t("agents.discardAll")}
            </button>
            <button
              className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-semibold text-slate-950 disabled:opacity-60"
              disabled={isSavingAll}
              onClick={() => void handleSaveAll()}
              type="button"
            >
              {isSavingAll ? t("agents.card.saving") : t("agents.saveAll")}
            </button>
          </div>
        </div>
      ) : null}

      {syncModeError ? (
        <div className="rounded-2xl border border-amber-900/60 bg-amber-950/40 px-4 py-3 text-sm text-amber-200">
          {syncModeError}
        </div>
      ) : null}

      {actionNotice ? (
        <div className="rounded-2xl border border-emerald-900/60 bg-emerald-950/30 px-4 py-3 text-sm text-emerald-200">
          {actionNotice}
        </div>
      ) : null}

      <section id="agent-sync-targets" className="scroll-mt-8 space-y-4">
        <div><h2 className="text-lg font-semibold text-slate-100">{t("agents.targets.title")}</h2><p className="mt-1 text-sm text-slate-500">{t("agents.targets.description")}</p></div>

        {isLoadingAgents ? (
          <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5 text-sm text-slate-400">
            {t("agents.targets.loading")}
          </div>
        ) : (
          <AgentTargetsSection
            actionKey={targetActionId}
            agents={sortedAgentInventory}
            canImport={Boolean(repoPath)}
            deleteTargetSkill={deleteTargetSkill}
            disabledSkillIds={disabledSkillIds}
            drafts={drafts}
            importTargetSkill={importTargetSkill}
            isSavingAll={isSavingAll}
            onDraftChange={(agentKey, nextDraft) =>
              setDrafts((current) => ({ ...current, [agentKey]: nextDraft }))
            }
            onSaveAgent={handleSaveAgent}
            savingAgentKey={savingAgentKey}
            sceneConfig={sceneConfig}
            scenes={sceneList}
            skills={scanResult.skills}
            takeOverTargetSkill={takeOverTargetSkill}
            updatingAgentKey={updatingAgentKey}
          />
        )}
      </section>

      <section className="space-y-4">
        {isLoadingExternalSources ? (
          <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5 text-sm text-slate-400">{t("sources.loading")}</div>
        ) : (
          sortedAgentInventory.map((agent) => (
            <AgentExternalVariantPanel
              agent={agent}
              key={`external-${agent.key}`}
              onImportVariant={importExternalVariant}
              onRepairImport={repairExternalImport}
              onUpdateImport={updateExternalImport}
              repoPath={repoPath}
              sources={externalSources}
              updatingExternalImportId={updatingExternalImportId}
              updatingExternalSourceId={updatingExternalSourceId}
            />
          ))
        )}
      </section>

      <div id="agent-sync-results" className="scroll-mt-8">
        <AgentApplyResults result={lastAgentApplyResult} />
      </div>

      {isOrderModalOpen ? (
        <AgentOrderModal
          agents={agentInventory}
          initialOrder={agentOrder}
          isSaving={isSavingAgentOrder}
          onClose={() => setIsOrderModalOpen(false)}
          onSave={handleSaveAgentOrder}
        />
      ) : null}
    </div>
  );
}
