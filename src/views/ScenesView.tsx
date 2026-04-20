import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import * as scenesApi from "../lib/scenes";
import type { SceneConfigSnapshot, SceneEntry } from "../lib/scenes";
import { useAppContext } from "../context/AppContext";
import {
  CheckCircle,
  Circle,
  Copy,
  Pencil,
  Plus,
  Power,
  Trash2,
} from "lucide-react";

export function ScenesView() {
  const { t } = useTranslation();
  const { repoPath, scanResult, agentInventory } = useAppContext();
  const [config, setConfig] = useState<SceneConfigSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editName, setEditName] = useState("");
  const [editDesc, setEditDesc] = useState("");
  const [creating, setCreating] = useState(false);
  const [newId, setNewId] = useState("");
  const [newName, setNewName] = useState("");
  const [applying, setApplying] = useState<string | null>(null);
  const [lastResult, setLastResult] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const snapshot = await scenesApi.getSceneConfig();
      setConfig(snapshot);
    } catch {
      setConfig(null);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const handleCreate = async () => {
    if (!newId.trim() || !newName.trim()) return;
    setCreating(true);
    try {
      const snapshot = await scenesApi.createScene(
        newId.trim(),
        newName.trim(),
        "",
      );
      setConfig(snapshot);
      setNewId("");
      setNewName("");
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    } finally {
      setCreating(false);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      const snapshot = await scenesApi.deleteScene(id);
      setConfig(snapshot);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleApply = async (id: string) => {
    setApplying(id);
    try {
      const result = await scenesApi.applyScene(id);
      setLastResult(
        `Applied "${result.sceneName}": ${result.enabledAgentCount} agents, ${result.disabledSkillCount} disabled skills`,
      );
      await refresh();
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    } finally {
      setApplying(null);
    }
  };

  const handleSaveEdit = async (id: string) => {
    try {
      const snapshot = await scenesApi.updateScene(id, editName, editDesc);
      setConfig(snapshot);
      setEditingId(null);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const startEdit = (scene: SceneEntry) => {
    setEditingId(scene.id);
    setEditName(scene.name);
    setEditDesc(scene.description);
  };

  if (!repoPath) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
        <h2 className="text-xl font-semibold text-slate-100">
          {t("scenes.unconfigured")}
        </h2>
        <p className="mt-3 text-sm text-slate-400">
          {t("scenes.unconfiguredBody")}
        </p>
      </section>
    );
  }

  const sceneList = config ? Object.values(config.scenes) : [];
  const skills = scanResult.skills;
  const agents = agentInventory;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-semibold text-slate-100">
            {t("scenes.title")}
          </h2>
          <p className="mt-1 text-sm text-slate-500">
            {t("scenes.description")}
          </p>
        </div>
      </div>

      {lastResult ? (
        <div className="rounded-lg bg-sky-950/40 px-4 py-2 text-sm text-sky-300">
          {lastResult}
          <button
            className="ml-2 text-sky-500 hover:text-sky-300"
            onClick={() => setLastResult(null)}
          >
            ×
          </button>
        </div>
      ) : null}

      <div className="flex items-end gap-3 rounded-2xl border border-slate-800 bg-slate-900 p-4">
        <div className="flex-1">
          <label className="mb-1 block text-xs text-slate-400">
            {t("scenes.create.idLabel")}
          </label>
          <input
            className="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
            placeholder="work"
            value={newId}
            onChange={(e) => setNewId(e.target.value)}
          />
        </div>
        <div className="flex-1">
          <label className="mb-1 block text-xs text-slate-400">
            {t("scenes.create.nameLabel")}
          </label>
          <input
            className="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
            placeholder="Work Scene"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") void handleCreate();
            }}
          />
        </div>
        <button
          className="flex items-center gap-2 rounded-lg bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-500 disabled:opacity-50"
          disabled={creating || !newId.trim() || !newName.trim()}
          onClick={() => void handleCreate()}
        >
          <Plus className="h-4 w-4" />
          {t("scenes.create.button")}
        </button>
      </div>

      {isLoading ? (
        <div className="text-sm text-slate-400">{t("scenes.loading")}</div>
      ) : sceneList.length === 0 ? (
        <div className="text-sm text-slate-500">{t("scenes.empty")}</div>
      ) : (
        <div className="space-y-4">
          {sceneList.map((scene) => (
            <SceneCard
              key={scene.id}
              agents={agents}
              isActive={config?.activeSceneId === scene.id}
              isApplying={applying === scene.id}
              isEditing={editingId === scene.id}
              scene={scene}
              skills={skills}
              editDesc={editDesc}
              editName={editName}
              onApply={() => void handleApply(scene.id)}
              onCancelEdit={() => setEditingId(null)}
              onDelete={() => void handleDelete(scene.id)}
              onSaveEdit={() => void handleSaveEdit(scene.id)}
              onStartEdit={() => startEdit(scene)}
              onEditDescChange={setEditDesc}
              onEditNameChange={setEditName}
              t={t}
            />
          ))}
        </div>
      )}
    </div>
  );
}

interface SceneCardProps {
  scene: SceneEntry;
  isActive: boolean;
  isEditing: boolean;
  isApplying: boolean;
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
  onEditNameChange: (v: string) => void;
  onEditDescChange: (v: string) => void;
}

function SceneCard({
  scene,
  skills,
  isActive,
  isEditing,
  isApplying,
  editName,
  editDesc,
  t,
  onStartEdit,
  onCancelEdit,
  onSaveEdit,
  onDelete,
  onApply,
  onEditNameChange,
  onEditDescChange,
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
              title={t("scenes.card.duplicate")}
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
        <span>
          {t("scenes.card.agents", { count: scene.enabledAgentKeys.length })}
        </span>
      </div>

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
