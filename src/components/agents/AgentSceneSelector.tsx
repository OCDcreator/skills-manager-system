import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { toggleId, type AgentConfigDraft } from "../../lib/agent-selection";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { SceneEntry } from "../../lib/scenes";

interface AgentSceneSelectorProps {
  draft: AgentConfigDraft;
  scenes: SceneEntry[];
  onDraftChange: (draft: AgentConfigDraft) => void;
}

export function AgentSceneSelector({
  draft,
  scenes,
  onDraftChange,
}: AgentSceneSelectorProps) {
  const { t } = useTranslation();
  const selected = useMemo(() => new Set(draft.selectedSceneIds), [draft.selectedSceneIds]);
  const scrollRef = useRememberedScrollPosition(`agents:scene-selector:${draft.key}`);

  return (
    <div className="space-y-2 rounded-xl border border-slate-800 bg-slate-950/50 p-3">
      <div className="flex items-center justify-between gap-3">
        <label className="text-xs font-semibold uppercase tracking-wide text-slate-500">
          {t("agents.card.scenes")}
        </label>
        <span className="text-[11px] text-slate-500">
          {t("agents.card.selectedCount", { count: draft.selectedSceneIds.length })}
        </span>
      </div>
      <div
        className="skill-markdown-scroll max-h-40 space-y-1 overflow-y-auto pr-1"
        ref={scrollRef}
      >
        {scenes.length === 0 ? (
          <div className="text-xs text-slate-500">{t("agents.card.noScenes")}</div>
        ) : (
          scenes.map((scene) => (
            <label
              className="flex cursor-pointer items-start gap-2 rounded-md px-2 py-1.5 text-xs text-slate-300"
              key={scene.id}
            >
              <input
                checked={selected.has(scene.id)}
                className="mt-0.5 rounded border-slate-600"
                onChange={() =>
                  onDraftChange({
                    ...draft,
                    selectedSceneIds: toggleId(draft.selectedSceneIds, scene.id),
                  })
                }
                type="checkbox"
              />
              <span className="min-w-0 flex-1">
                <span className="block truncate">{scene.name}</span>
                <span className="block truncate text-[11px] text-slate-500">
                  {scene.description || scene.id}
                </span>
              </span>
            </label>
          ))
        )}
      </div>
    </div>
  );
}
