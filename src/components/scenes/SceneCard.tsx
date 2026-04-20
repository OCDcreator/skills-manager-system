import {
  CheckCircle,
  CheckSquare,
  Circle,
  Copy,
  Pencil,
  Power,
  Settings2,
  Square,
  Trash2,
} from "lucide-react";
import type { SceneEntry } from "../../lib/scenes";

export interface SceneCardProps {
  scene: SceneEntry;
  isActive: boolean;
  isEditing: boolean;
  isApplying: boolean;
  isConfiguring: boolean;
  skills: { id: string; name: string }[];
  agents: { key: string; displayName: string }[];
  editName: string;
  editDesc: string;
  t: (key: string, options?: Record<string, unknown>) => string;
  onStartEdit: () => void;
  onCancelEdit: () => void;
  onSaveEdit: () => void;
  onDelete: () => void;
  onApply: () => void;
  onDuplicate: () => void;
  onEditNameChange: (v: string) => void;
  onEditDescChange: (v: string) => void;
  onToggleConfigure: () => void;
  onToggleSkill: (skillId: string) => void;
  onToggleAgent: (agentKey: string) => void;
}

export function SceneCard({
  scene,
  skills,
  agents,
  isActive,
  isEditing,
  isApplying,
  isConfiguring,
  editName,
  editDesc,
  t,
  onStartEdit,
  onCancelEdit,
  onSaveEdit,
  onDelete,
  onApply,
  onDuplicate,
  onEditNameChange,
  onEditDescChange,
  onToggleConfigure,
  onToggleSkill,
  onToggleAgent,
}: SceneCardProps) {
  const allSkillsEnabled = scene.disabledSkillIds.length === 0;
  const enabledSkillCount = skills.length - scene.disabledSkillIds.length;

  return (
    <div
      className={`rounded-2xl border p-5 ${
        isActive
          ? "border-sky-700 bg-sky-950/20"
          : "border-slate-800 bg-slate-900"
      }`}
    >
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-center gap-3">
          {isActive ? (
            <CheckCircle className="h-5 w-5 text-sky-400" />
          ) : (
            <Circle className="h-5 w-5 text-slate-600" />
          )}
          <div>
            {isEditing ? (
              <div className="flex items-center gap-2">
                <input
                  className="rounded border border-slate-700 bg-slate-800 px-2 py-1 text-sm text-slate-200"
                  value={editName}
                  onChange={(e) => onEditNameChange(e.target.value)}
                />
                <input
                  className="flex-1 rounded border border-slate-700 bg-slate-800 px-2 py-1 text-sm text-slate-200"
                  placeholder={t("scenes.card.description")}
                  value={editDesc}
                  onChange={(e) => onEditDescChange(e.target.value)}
                />
                <button
                  className="rounded bg-sky-600 px-3 py-1 text-xs text-white"
                  onClick={onSaveEdit}
                >
                  {t("scenes.card.save")}
                </button>
                <button
                  className="rounded bg-slate-700 px-3 py-1 text-xs text-slate-300"
                  onClick={onCancelEdit}
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
        {!isEditing && (
          <div className="flex items-center gap-1">
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              title={t("scenes.card.edit")}
              onClick={onStartEdit}
            >
              <Pencil className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              title={t("scenes.card.configure")}
              onClick={onToggleConfigure}
            >
              <Settings2 className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-slate-800 hover:text-slate-200"
              title={t("scenes.card.duplicate")}
              onClick={onDuplicate}
            >
              <Copy className="h-4 w-4" />
            </button>
            <button
              className="rounded p-1.5 text-slate-400 hover:bg-rose-900/40 hover:text-rose-300"
              title={t("scenes.card.delete")}
              onClick={onDelete}
            >
              <Trash2 className="h-4 w-4" />
            </button>
          </div>
        )}
      </div>

      <div className="mt-3 flex items-center gap-4 text-xs text-slate-400">
        <span>
          {allSkillsEnabled
            ? t("scenes.card.allSkills", { count: skills.length })
            : t("scenes.card.skills", { count: enabledSkillCount })}
        </span>
        <span>{t("scenes.card.agents", { count: scene.enabledAgentKeys.length })}</span>
      </div>

      {isConfiguring ? (
        <div className="mt-4 grid gap-4 md:grid-cols-2">
          <div className="bg-slate-800/50 rounded-lg p-3">
            <div className="mb-3 text-xs font-medium text-slate-200">
              {t("scenes.card.skillsPanel", {
                enabled: enabledSkillCount,
                total: skills.length,
              })}
            </div>
            <div className="max-h-48 space-y-2 overflow-y-auto pr-1">
              {skills.map((skill) => {
                const enabled = !scene.disabledSkillIds.includes(skill.id);
                return (
                  <label
                    key={skill.id}
                    className="flex cursor-pointer items-center gap-2 text-xs text-slate-300"
                  >
                    <button
                      className="text-slate-400 hover:text-sky-400"
                      onClick={() => onToggleSkill(skill.id)}
                      aria-label={skill.name}
                      type="button"
                    >
                      {enabled ? (
                        <CheckSquare className="h-4 w-4" />
                      ) : (
                        <Square className="h-4 w-4" />
                      )}
                    </button>
                    <span>{skill.name}</span>
                  </label>
                );
              })}
            </div>
          </div>

          <div className="bg-slate-800/50 rounded-lg p-3">
            <div className="mb-3 text-xs font-medium text-slate-200">
              {t("scenes.card.agentsPanel", {
                count: scene.enabledAgentKeys.length,
              })}
            </div>
            <div className="max-h-48 space-y-2 overflow-y-auto pr-1">
              {agents.map((agent) => {
                const enabled = scene.enabledAgentKeys.includes(agent.key);
                return (
                  <label
                    key={agent.key}
                    className="flex cursor-pointer items-center gap-2 text-xs text-slate-300"
                  >
                    <button
                      className="text-slate-400 hover:text-sky-400"
                      onClick={() => onToggleAgent(agent.key)}
                      aria-label={agent.displayName}
                      type="button"
                    >
                      {enabled ? (
                        <CheckSquare className="h-4 w-4" />
                      ) : (
                        <Square className="h-4 w-4" />
                      )}
                    </button>
                    <span>{agent.displayName}</span>
                  </label>
                );
              })}
            </div>
          </div>
        </div>
      ) : null}

      <div className="mt-3">
        <button
          className="flex items-center gap-2 rounded-lg bg-slate-800 px-4 py-2 text-sm text-slate-200 hover:bg-slate-700 disabled:opacity-50"
          disabled={isApplying}
          onClick={onApply}
        >
          <Power className="h-4 w-4" />
          {isApplying ? t("scenes.card.applying") : t("scenes.card.apply")}
        </button>
      </div>
    </div>
  );
}
