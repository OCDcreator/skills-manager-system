import { useCallback, useState } from "react";
import { confirm } from "@tauri-apps/plugin-dialog";
import type { TFunction } from "i18next";
import * as api from "./tauri";
import type { AgentTargetSkillEntry } from "./tauri";

interface UseAgentTargetActionsOptions {
  repoPath: string | null;
  refreshAgents: () => Promise<void>;
  refreshSkills: () => Promise<void>;
  setError: (message: string | null) => void;
  t: TFunction;
}

function messageFrom(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}

function buildTargetActionId(agentKey: string, entryName: string, action: string) {
  return `${agentKey}:${entryName}:${action}`;
}

export function useAgentTargetActions({
  repoPath,
  refreshAgents,
  refreshSkills,
  setError,
  t,
}: UseAgentTargetActionsOptions) {
  const [actionId, setActionId] = useState<string | null>(null);
  const [actionNotice, setActionNotice] = useState<string | null>(null);

  const clearActionNotice = useCallback(() => {
    setActionNotice(null);
  }, []);

  const takeOverTargetSkill = useCallback(
    async (agentKey: string, agentName: string, entry: AgentTargetSkillEntry) => {
      const accepted = await confirm(
        t("agents.globalSkills.confirmTakeOverBody", {
          agent: agentName,
          name: entry.displayName,
        }),
        {
          title: t("agents.globalSkills.confirmTakeOverTitle"),
          kind: "warning",
        },
      );
      if (!accepted) return;

      setActionId(buildTargetActionId(agentKey, entry.entryName, "take-over"));
      setActionNotice(null);
      try {
        await api.takeOverAgentTargetSkill(agentKey, entry.entryName);
        await refreshAgents();
        setError(null);
        setActionNotice(t("agents.globalSkills.noticeTakeOver", { name: entry.displayName }));
      } catch (error) {
        setError(messageFrom(error));
      } finally {
        setActionId(null);
      }
    },
    [refreshAgents, setError, t],
  );

  const deleteTargetSkill = useCallback(
    async (agentKey: string, agentName: string, entry: AgentTargetSkillEntry) => {
      const accepted = await confirm(
        t(
          entry.managed
            ? "agents.globalSkills.confirmDeleteManagedBody"
            : "agents.globalSkills.confirmDeleteUnmanagedBody",
          {
            agent: agentName,
            name: entry.displayName,
          },
        ),
        {
          title: t("agents.globalSkills.confirmDeleteTitle"),
          kind: "warning",
        },
      );
      if (!accepted) return;

      setActionId(buildTargetActionId(agentKey, entry.entryName, "delete"));
      setActionNotice(null);
      try {
        await api.deleteAgentTargetSkill(agentKey, entry.entryName);
        await refreshAgents();
        setError(null);
        setActionNotice(t("agents.globalSkills.noticeDelete", { name: entry.displayName }));
      } catch (error) {
        setError(messageFrom(error));
      } finally {
        setActionId(null);
      }
    },
    [refreshAgents, setError, t],
  );

  const importTargetSkill = useCallback(
    async (
      agentKey: string,
      agentName: string,
      entry: AgentTargetSkillEntry,
      deleteSourceAfterImport: boolean,
    ) => {
      if (!repoPath) return;

      const accepted = await confirm(
        t(
          deleteSourceAfterImport
            ? "agents.globalSkills.confirmImportDeleteBody"
            : "agents.globalSkills.confirmImportBody",
          {
            agent: agentName,
            name: entry.displayName,
            repoPath,
          },
        ),
        {
          title: t(
            deleteSourceAfterImport
              ? "agents.globalSkills.confirmImportDeleteTitle"
              : "agents.globalSkills.confirmImportTitle",
          ),
          kind: "warning",
        },
      );
      if (!accepted) return;

      setActionId(
        buildTargetActionId(
          agentKey,
          entry.entryName,
          deleteSourceAfterImport ? "import-delete" : "import",
        ),
      );
      setActionNotice(null);
      try {
        const result = await api.importAgentTargetSkill(
          agentKey,
          entry.entryName,
          deleteSourceAfterImport,
        );
        await Promise.all([refreshAgents(), refreshSkills()]);
        setError(null);
        setActionNotice(
          t(
            deleteSourceAfterImport
              ? "agents.globalSkills.noticeImportDelete"
              : "agents.globalSkills.noticeImport",
            {
              name: entry.displayName,
              path: result.relativePath,
            },
          ),
        );
      } catch (error) {
        setError(messageFrom(error));
      } finally {
        setActionId(null);
      }
    },
    [refreshAgents, refreshSkills, repoPath, setError, t],
  );

  return {
    actionId,
    actionNotice,
    clearActionNotice,
    deleteTargetSkill,
    importTargetSkill,
    takeOverTargetSkill,
  };
}
