import { useCallback, useState, type Dispatch, type SetStateAction } from "react";
import { confirm } from "@tauri-apps/plugin-dialog";
import { useTranslation } from "react-i18next";
import {
  removeProjectManagedTargetSkill,
  type ProjectDraft,
} from "../../lib/project-draft";
import * as projectsApi from "../../lib/projects";
import type { AgentTargetSkillEntry } from "../../lib/tauri";

interface ProjectTargetSkillActionOptions {
  draft: ProjectDraft;
  setDraft: Dispatch<SetStateAction<ProjectDraft>>;
  setInspectionRefreshKey: Dispatch<SetStateAction<number>>;
  setLastResult: Dispatch<SetStateAction<string | null>>;
}

export function useProjectTargetSkillActions({
  draft,
  setDraft,
  setInspectionRefreshKey,
  setLastResult,
}: ProjectTargetSkillActionOptions) {
  const { t } = useTranslation();
  const [targetActionId, setTargetActionId] = useState<string | null>(null);

  const handleDeleteTargetSkill = useCallback(
    async (agentKey: string, entry: AgentTargetSkillEntry) => {
      if (entry.managed) {
        if (!entry.skillId) {
          setLastResult(t("projects.summary.missingManagedSkillId"));
          return;
        }
        setDraft((current) =>
          removeProjectManagedTargetSkill(current, agentKey, entry.skillId!),
        );
        setLastResult(
          t("projects.summary.cancelSelectionNotice", { name: entry.displayName }),
        );
        return;
      }

      const accepted = await confirm(
        t("projects.summary.confirmDeleteUnmanagedBody", {
          name: entry.displayName,
        }),
        {
          title: t("projects.summary.confirmDeleteUnmanagedTitle"),
          kind: "warning",
        },
      );
      if (!accepted) return;

      const actionId = `${agentKey}:${entry.entryName}:delete`;
      setTargetActionId(actionId);
      try {
        await projectsApi.deleteProjectTargetSkill(
          draft.projectPath.trim(),
          agentKey,
          entry.entryName,
        );
        setInspectionRefreshKey((current) => current + 1);
        setLastResult(
          t("projects.summary.deleteTargetSkillNotice", {
            name: entry.displayName,
          }),
        );
      } catch (error) {
        setLastResult(error instanceof Error ? error.message : String(error));
      } finally {
        setTargetActionId(null);
      }
    },
    [draft.projectPath, setDraft, setInspectionRefreshKey, setLastResult, t],
  );

  return { handleDeleteTargetSkill, targetActionId };
}
