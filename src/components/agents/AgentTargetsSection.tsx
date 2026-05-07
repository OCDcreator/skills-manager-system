import { useTranslation } from "react-i18next";
import {
  draftFromAgent,
  isAgentDraftDirty,
  resolveAgentSelectionPreview,
  type AgentConfigDraft,
} from "../../lib/agent-selection";
import type { SceneConfigSnapshot, SceneEntry } from "../../lib/scenes";
import type {
  AgentInventoryItem,
  ExternalSourceSnapshotItem,
  ExternalVariantKey,
  SkillSummary,
} from "../../lib/tauri";
import { AgentExternalVariantPanel } from "./AgentExternalVariantPanel";
import { AgentGlobalSkillList } from "./AgentGlobalSkillList";
import { AgentTargetCard } from "./AgentTargetCard";

interface AgentTargetsSectionProps {
  actionKey: string | null;
  agents: AgentInventoryItem[];
  canImport: boolean;
  batchDeleteTargetSkills: (
    agentKey: string,
    agentDisplayName: string,
    entries: AgentInventoryItem["targetSkillEntries"],
  ) => Promise<void>;
  batchImportTargetSkills: (
    agentKey: string,
    agentDisplayName: string,
    entries: AgentInventoryItem["targetSkillEntries"],
    deleteSourceAfterImport: boolean,
  ) => Promise<void>;
  batchTakeOverTargetSkills: (
    agentKey: string,
    agentDisplayName: string,
    entries: AgentInventoryItem["targetSkillEntries"],
  ) => Promise<void>;
  deleteTargetSkill: (
    agentKey: string,
    agentDisplayName: string,
    entry: AgentInventoryItem["targetSkillEntries"][number],
  ) => Promise<void>;
  disabledSkillIds: string[];
  drafts: Record<string, AgentConfigDraft>;
  externalSources: ExternalSourceSnapshotItem[];
  importTargetSkill: (
    agentKey: string,
    agentDisplayName: string,
    entry: AgentInventoryItem["targetSkillEntries"][number],
    deleteSourceAfterImport: boolean,
  ) => Promise<void>;
  isLoadingExternalSources: boolean;
  savingAgentKey: string | null;
  sceneConfig: SceneConfigSnapshot | null;
  scenes: SceneEntry[];
  skills: SkillSummary[];
  takeOverTargetSkill: (
    agentKey: string,
    agentDisplayName: string,
    entry: AgentInventoryItem["targetSkillEntries"][number],
  ) => Promise<void>;
  updatingAgentKey: string | null;
  updatingExternalImportId: string | null;
  updatingExternalSourceId: string | null;
  isSavingAll: boolean;
  onDraftChange: (agentKey: string, nextDraft: AgentConfigDraft) => void;
  onImportVariant: (
    sourceId: string,
    agentKey: ExternalVariantKey,
    variantPath: string,
  ) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
  onSaveAgent: (agentKey: string) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
  repoPath: string | null;
}

export function AgentTargetsSection({
  actionKey,
  agents,
  canImport,
  batchDeleteTargetSkills,
  batchImportTargetSkills,
  batchTakeOverTargetSkills,
  deleteTargetSkill,
  disabledSkillIds,
  drafts,
  externalSources,
  importTargetSkill,
  isLoadingExternalSources,
  savingAgentKey,
  sceneConfig,
  scenes,
  skills,
  takeOverTargetSkill,
  updatingAgentKey,
  updatingExternalImportId,
  updatingExternalSourceId,
  isSavingAll,
  onDraftChange,
  onImportVariant,
  onRepairImport,
  onSaveAgent,
  onUpdateImport,
  repoPath,
}: AgentTargetsSectionProps) {
  const { t } = useTranslation();

  return (
    <div className="grid gap-4">
      {agents.map((agent) => {
        const draft = drafts[agent.key] ?? draftFromAgent(agent);
        const preview = resolveAgentSelectionPreview(
          draft,
          skills,
          disabledSkillIds,
          sceneConfig?.scenes ?? {},
          agent.targetSkillEntries,
        );

        return (
          <div id={`agent-sync-target-${agent.key}`} className="scroll-mt-8" key={agent.key}>
            <div className="grid gap-4 min-[1380px]:grid-cols-[minmax(0,1fr)_clamp(22rem,28vw,34rem)]">
              <AgentTargetCard
                agent={agent}
                disabledSkillIds={disabledSkillIds}
                draft={draft}
                isDirty={isAgentDraftDirty(agent, draft)}
                isUpdating={
                  updatingAgentKey === agent.key ||
                  savingAgentKey === agent.key ||
                  isSavingAll
                }
                onDraftChange={(nextDraft) => onDraftChange(agent.key, nextDraft)}
                onSave={() => onSaveAgent(agent.key)}
                preview={preview}
                scenes={scenes}
                skills={skills}
              />
              <div className="min-h-0 min-[1380px]:relative min-[1380px]:overflow-hidden">
                <div className="min-[1380px]:absolute min-[1380px]:inset-0">
                  <AgentGlobalSkillList
                    actionKey={actionKey}
                    agent={agent}
                    canImport={canImport}
                    onBatchDelete={(entries) =>
                      void batchDeleteTargetSkills(agent.key, agent.displayName, entries)
                    }
                    onBatchImport={(entries, deleteSourceAfterImport) =>
                      void batchImportTargetSkills(
                        agent.key,
                        agent.displayName,
                        entries,
                        deleteSourceAfterImport,
                      )
                    }
                    onBatchTakeOver={(entries) =>
                      void batchTakeOverTargetSkills(agent.key, agent.displayName, entries)
                    }
                    onDelete={(entry) =>
                      void deleteTargetSkill(agent.key, agent.displayName, entry)
                    }
                    onImport={(entry, deleteSourceAfterImport) =>
                      void importTargetSkill(
                        agent.key,
                        agent.displayName,
                        entry,
                        deleteSourceAfterImport,
                      )
                    }
                    onTakeOver={(entry) =>
                      void takeOverTargetSkill(agent.key, agent.displayName, entry)
                    }
                  />
                </div>
              </div>
            </div>
            <div className="mt-4">
              {isLoadingExternalSources ? (
                <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5 text-sm text-slate-400">
                  {t("sources.loading")}
                </div>
              ) : (
                <AgentExternalVariantPanel
                  agent={agent}
                  onImportVariant={onImportVariant}
                  onRepairImport={onRepairImport}
                  onUpdateImport={onUpdateImport}
                  repoPath={repoPath}
                  sources={externalSources}
                  updatingExternalImportId={updatingExternalImportId}
                  updatingExternalSourceId={updatingExternalSourceId}
                />
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
