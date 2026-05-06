import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus } from "lucide-react";
import { SceneCard } from "../components/scenes/SceneCard";
import { useAppContext } from "../context/AppContext";
import { reorderSceneSkillOrder } from "../lib/scene-skill-order";
import * as scenesApi from "../lib/scenes";
import type { SceneConfigSnapshot, SceneEntry } from "../lib/scenes";

export function ScenesView() {
  const { t } = useTranslation();
  const {
    repoPath,
    scanResult,
  } = useAppContext();
  const [config, setConfig] = useState<SceneConfigSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [configuringId, setConfiguringId] = useState<string | null>(null);
  const [editName, setEditName] = useState("");
  const [editDesc, setEditDesc] = useState("");
  const [creating, setCreating] = useState(false);
  const [newId, setNewId] = useState("");
  const [newName, setNewName] = useState("");
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
    queueMicrotask(() => {
      void refresh();
    });
  }, [refresh]);

  const handleCreate = async () => {
    if (!newId.trim() || !newName.trim()) return;
    setCreating(true);
    try {
      const snapshot = await scenesApi.createScene(newId.trim(), newName.trim(), "");
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

  const handleToggleSkill = async (sceneId: string, skillId: string) => {
    const scene = config?.scenes[sceneId];
    if (!scene) return;
    try {
      if (scene.skillSelectionMode === "onlySelected") {
        const selected = new Set(scene.selectedSkillIds);
        if (selected.has(skillId)) selected.delete(skillId);
        else selected.add(skillId);
        const snapshot = await scenesApi.setSceneSkills(sceneId, [...selected].sort());
        setConfig(snapshot);
        return;
      }

      const disabled = new Set(scene.disabledSkillIds);
      if (disabled.has(skillId)) disabled.delete(skillId);
      else disabled.add(skillId);
      const snapshot = await scenesApi.setSceneSkills(sceneId, [...disabled].sort());
      setConfig(snapshot);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleDuplicate = async (scene: SceneEntry) => {
    try {
      const snapshot = await scenesApi.createScene(
        `${scene.id}-copy`,
        `${scene.name} (${t("scenes.card.duplicateSuffix")})`,
        scene.description,
      );
      setConfig(snapshot);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
  };

  const handleReorderSkill = async (
    sceneId: string,
    draggedSkillId: string,
    targetSkillId: string,
  ) => {
    const scene = config?.scenes[sceneId];
    if (!scene) return;
    const nextOrder = reorderSceneSkillOrder(
      scene,
      scanResult.skills,
      draggedSkillId,
      targetSkillId,
    );
    if (!nextOrder) return;
    try {
      const snapshot = await scenesApi.setSceneSkillOrder(sceneId, nextOrder);
      setConfig(snapshot);
    } catch (error) {
      setLastResult(error instanceof Error ? error.message : String(error));
    }
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
            title={t("tooltip.scenes.dismiss")}
            type="button"
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
            onChange={(event) => setNewId(event.target.value)}
            placeholder={t("scenes.create.idPlaceholder")}
            value={newId}
          />
        </div>
        <div className="flex-1">
          <label className="mb-1 block text-xs text-slate-400">
            {t("scenes.create.nameLabel")}
          </label>
          <input
            className="w-full rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-slate-200 placeholder:text-slate-500 focus:border-sky-400 focus:outline-none"
            onChange={(event) => setNewName(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") void handleCreate();
            }}
            placeholder={t("scenes.create.namePlaceholder")}
            value={newName}
          />
        </div>
        <button
          className="flex items-center gap-2 rounded-lg bg-sky-600 px-4 py-2 text-sm text-white hover:bg-sky-500 disabled:opacity-50"
          disabled={creating || !newId.trim() || !newName.trim()}
          onClick={() => void handleCreate()}
          title={t("tooltip.scenes.create")}
          type="button"
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
              editDesc={editDesc}
              editName={editName}
              isConfiguring={configuringId === scene.id}
              isEditing={editingId === scene.id}
              key={scene.id}
              onCancelEdit={() => setEditingId(null)}
              onDelete={() => void handleDelete(scene.id)}
              onDuplicate={() => void handleDuplicate(scene)}
              onEditDescChange={setEditDesc}
              onEditNameChange={setEditName}
              onReorderSkill={(draggedSkillId, targetSkillId) =>
                void handleReorderSkill(scene.id, draggedSkillId, targetSkillId)
              }
              onSaveEdit={() => void handleSaveEdit(scene.id)}
              onStartEdit={() => startEdit(scene)}
              onToggleConfigure={() =>
                setConfiguringId((current) => (current === scene.id ? null : scene.id))
              }
              onToggleSkill={(skillId) => void handleToggleSkill(scene.id, skillId)}
              scene={scene}
              skills={skills}
              t={t}
            />
          ))}
        </div>
      )}
    </div>
  );
}
