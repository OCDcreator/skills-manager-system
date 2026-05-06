import {
  Copy,
  Pencil,
  Settings2,
  Trash2,
} from "lucide-react";
import {
  getSceneEnabledSkillCount,
  isSceneSkillEnabled,
} from "../../lib/scene-skill-order";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillSummary } from "../../lib/tauri";
import { SceneSkillChooser } from "./SceneSkillChooser";

export interface SceneCardProps {
  scene: SceneEntry;
  isEditing: boolean;
  isConfiguring: boolean;
  skills: SkillSummary[];
  editName: string;
  editDesc: string;
  t: (key: string, options?: Record<string, unknown>) => string;
  onStartEdit: () => void;
  onCancelEdit: () => void;
  onSaveEdit: () => void;
  onDelete: () => void;
  onDuplicate: () => void;
  onEditNameChange: (value: string) => void;
  onEditDescChange: (value: string) => void;
  onToggleConfigure: () => void;
  onToggleSkill: (skillId: string) => void;
  onReorderSkill: (draggedSkillId: string, targetSkillId: string) => void;
}

export function SceneCard({
  scene,
  skills,
  isEditing,
  isConfiguring,
  editName,
  editDesc,
  t,
  onStartEdit,
  onCancelEdit,
  onSaveEdit,
  onDelete,
  onDuplicate,
  onEditNameChange,
  onEditDescChange,
  onToggleConfigure,
  onToggleSkill,
  onReorderSkill,
}: SceneCardProps) {
  const allSkillsEnabled = skills.length > 0 && skills.every((skill) => isSceneSkillEnabled(scene, skill.id));
  const enabledSkillCount = getSceneEnabledSkillCount(scene, skills);

  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-center gap-3">
          <div>
            {isEditing ? (
              <div className="flex items-center gap-2">
                <input
                  className="rounded border border-slate-700 bg-slate-800 px-2 py-1 text-sm text-slate-200"
                  onChange={(event) => onEditNameChange(event.target.value)}
                  value={editName}
                />
                <input
                  className="flex-1 rounded border border-slate-700 bg-slate-800 px-2 py-1 text-sm text-slate-200"
                  onChange={(event) => onEditDescChange(event.target.value)}
                  placeholder={t("scenes.card.description")}
                  value={editDesc}
                />
                <button
                  className="rounded bg-sky-600 px-3 py-1 text-xs text-white"
                  onClick={onSaveEdit}
                  title={t("tooltip.scenes.save")}
                  type="button"
                >
                  {t("scenes.card.save")}
                </button>
                <button
                  className="rounded bg-slate-700 px-3 py-1 text-xs text-slate-300"
                  onClick={onCancelEdit}
                  title={t("tooltip.scenes.cancel")}
                  type="button"
                >
                  {t("scenes.card.cancel")}
                </button>
              </div>
            ) : (
              <>
                <h3 className="font-semibold text-slate-100">{scene.name}</h3>
                <p className="text-xs text-slate-400">
                  {scene.description || t("scenes.card.noDescription")}
                </p>
              </>
            )}
          </div>
        </div>
        {!isEditing ? (
          <div className="flex items-center gap-1">
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              onClick={onStartEdit}
              title={t("tooltip.scenes.edit")}
              type="button"
            >
              <Pencil className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              onClick={onToggleConfigure}
              title={t("tooltip.scenes.configure")}
              type="button"
            >
              <Settings2 className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              onClick={onDuplicate}
              title={t("tooltip.scenes.duplicate")}
              type="button"
            >
              <Copy className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-rose-900/40 hover:text-rose-300"
              onClick={onDelete}
              title={t("tooltip.scenes.delete")}
              type="button"
            >
              <Trash2 className="h-4 w-4" />
            </button>
          </div>
        ) : null}
      </div>

      <div className="mt-3 flex items-center gap-4 text-xs text-slate-400">
        <span>
          {allSkillsEnabled
            ? t("scenes.card.allSkills", { count: skills.length })
            : t("scenes.card.skills", { count: enabledSkillCount })}
        </span>
      </div>

      {isConfiguring ? (
        <div className="mt-4">
          <SceneSkillChooser
            onReorderSkill={onReorderSkill}
            onToggleSkill={onToggleSkill}
            scene={scene}
            skills={skills}
            t={t}
          />
        </div>
      ) : null}

      <div className="mt-3">
        <p className="text-xs text-slate-500">
          {t("scenes.card.toolkitUsage")}
        </p>
      </div>
    </div>
  );
}
