import { useMemo, useState } from "react";
import {
  CheckCircle,
  CheckSquare,
  Circle,
  Copy,
  GripVertical,
  Pencil,
  Power,
  Settings2,
  Square,
  Trash2,
} from "lucide-react";
import {
  getOrderedEnabledSceneSkills,
  getSceneEnabledSkillCount,
  isSceneSkillEnabled,
} from "../../lib/scene-skill-order";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
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
  onEditNameChange: (value: string) => void;
  onEditDescChange: (value: string) => void;
  onToggleConfigure: () => void;
  onToggleSkill: (skillId: string) => void;
  onToggleAgent: (agentKey: string) => void;
  onReorderSkill: (draggedSkillId: string, targetSkillId: string) => void;
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
  onReorderSkill,
}: SceneCardProps) {
  const [draggedSkillId, setDraggedSkillId] = useState<string | null>(null);
  const [dropTargetSkillId, setDropTargetSkillId] = useState<string | null>(null);
  const allSkillsEnabled = skills.length > 0 && skills.every((skill) => isSceneSkillEnabled(scene, skill.id));
  const enabledSkillCount = getSceneEnabledSkillCount(scene, skills);
  const orderedEnabled = useMemo(
    () => getOrderedEnabledSceneSkills(scene, skills),
    [scene, skills],
  );
  const disabledSkills = useMemo(
    () => skills.filter((skill) => !isSceneSkillEnabled(scene, skill.id)),
    [scene, skills],
  );
  const skillsScrollRef = useRememberedScrollPosition(`scenes:card:${scene.id}:skills`);
  const agentsScrollRef = useRememberedScrollPosition(`scenes:card:${scene.id}:agents`);

  const resetDragState = () => {
    setDraggedSkillId(null);
    setDropTargetSkillId(null);
  };

  const handleDrop = (targetSkillId: string) => {
    if (!draggedSkillId) return;
    if (draggedSkillId !== targetSkillId) {
      onReorderSkill(draggedSkillId, targetSkillId);
    }
    resetDragState();
  };

  return (
    <div
      className={`rounded-2xl border p-5 ${
        isActive ? "border-sky-700 bg-sky-950/20" : "border-slate-800 bg-slate-900"
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
        <span>{t("scenes.card.agents", { count: scene.enabledAgentKeys.length })}</span>
      </div>

      {isConfiguring ? (
        <div className="mt-4 grid gap-4 md:grid-cols-2">
          <div className="rounded-lg bg-slate-800/50 p-3">
            <div className="mb-3 flex items-center justify-between gap-3">
              <div className="text-xs font-medium text-slate-200">
                {t("scenes.card.skillsPanel", {
                  enabled: enabledSkillCount,
                  total: skills.length,
                })}
              </div>
              <span className="text-[11px] text-slate-500">
                {t("scenes.card.dragHint")}
              </span>
            </div>
            <div className="max-h-48 space-y-2 overflow-y-auto pr-1" ref={skillsScrollRef}>
              {orderedEnabled.map((skill) => {
                const isDropTarget =
                  dropTargetSkillId === skill.id && draggedSkillId !== skill.id;
                return (
                  <div
                    className={`flex items-center gap-2 rounded-md px-1 py-1 text-xs text-slate-300 ${
                      isDropTarget ? "bg-sky-950/40 ring-1 ring-sky-700" : ""
                    } ${draggedSkillId === skill.id ? "opacity-60" : ""}`}
                    draggable
                    key={skill.id}
                    onDragEnd={resetDragState}
                    onDragEnter={() => setDropTargetSkillId(skill.id)}
                    onDragOver={(event) => {
                      event.preventDefault();
                      setDropTargetSkillId(skill.id);
                    }}
                    onDragStart={() => {
                      setDraggedSkillId(skill.id);
                      setDropTargetSkillId(skill.id);
                    }}
                    onDrop={(event) => {
                      event.preventDefault();
                      handleDrop(skill.id);
                    }}
                  >
                    <button
                      aria-label={skill.name}
                      className="text-slate-400 hover:text-sky-400"
                      onClick={() => onToggleSkill(skill.id)}
                      title={`${t("tooltip.scenes.toggleSkill")}: ${skill.name}`}
                      type="button"
                    >
                      <CheckSquare className="h-4 w-4" />
                    </button>
                    <span title={t("tooltip.scenes.gripDrag")}>
                      <GripVertical className="h-3.5 w-3.5 cursor-grab text-slate-500 active:cursor-grabbing" />
                    </span>
                    <span className="flex-1">{skill.name}</span>
                  </div>
                );
              })}
              {disabledSkills.map((skill) => (
                <div
                  className="flex items-center gap-2 text-xs text-slate-500"
                  key={skill.id}
                >
                  <button
                    aria-label={skill.name}
                    className="text-slate-400 hover:text-sky-400"
                    onClick={() => onToggleSkill(skill.id)}
                    title={`${t("tooltip.scenes.toggleSkill")}: ${skill.name}`}
                    type="button"
                  >
                    <Square className="h-4 w-4" />
                  </button>
                  <span>{skill.name}</span>
                </div>
              ))}
            </div>
          </div>

          <div className="rounded-lg bg-slate-800/50 p-3">
            <div className="mb-3 text-xs font-medium text-slate-200">
              {t("scenes.card.agentsPanel", {
                count: scene.enabledAgentKeys.length,
              })}
            </div>
            <div className="max-h-48 space-y-2 overflow-y-auto pr-1" ref={agentsScrollRef}>
              {agents.map((agent) => {
                const enabled = scene.enabledAgentKeys.includes(agent.key);
                return (
                  <label
                    className="flex cursor-pointer items-center gap-2 text-xs text-slate-300"
                    key={agent.key}
                  >
                    <button
                      aria-label={agent.displayName}
                      className="text-slate-400 hover:text-sky-400"
                      onClick={() => onToggleAgent(agent.key)}
                      title={`${t("tooltip.scenes.toggleAgent")}: ${agent.displayName}`}
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
          title={t("tooltip.scenes.apply")}
          type="button"
        >
          <Power className="h-4 w-4" />
          {isApplying ? t("scenes.card.applying") : t("scenes.card.apply")}
        </button>
      </div>
    </div>
  );
}
