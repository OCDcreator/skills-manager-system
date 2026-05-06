import { useMemo, useState, type DragEvent } from "react";
import { CheckSquare, GripVertical, Search, Square } from "lucide-react";
import {
  getSceneEnabledSkillCount,
  isSceneSkillEnabled,
} from "../../lib/scene-skill-order";
import {
  buildExternalGroupSummaries,
  buildSceneSkillSourceCounts,
  filterSceneSkills,
  getExternalGroupKey,
  getOrderedEnabledSkillSummaries,
  type ExternalGroupFilter,
  type SceneSkillSourceFilter,
  type SceneSkillStatusFilter,
} from "../../lib/scene-skill-filters";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { SceneEntry } from "../../lib/scenes";
import type { SkillSummary } from "../../lib/tauri";

interface SceneSkillChooserProps {
  scene: SceneEntry;
  skills: SkillSummary[];
  t: (key: string, options?: Record<string, unknown>) => string;
  onToggleSkill: (skillId: string) => void;
  onReorderSkill: (draggedSkillId: string, targetSkillId: string) => void;
}

export function SceneSkillChooser({
  scene,
  skills,
  t,
  onToggleSkill,
  onReorderSkill,
}: SceneSkillChooserProps) {
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<SceneSkillStatusFilter>("all");
  const [sourceFilter, setSourceFilter] = useState<SceneSkillSourceFilter>("all");
  const [externalGroupFilter, setExternalGroupFilter] = useState<ExternalGroupFilter>("all");
  const [isExternalGroupOpen, setIsExternalGroupOpen] = useState(false);
  const [draggedSkillId, setDraggedSkillId] = useState<string | null>(null);
  const [dropTargetSkillId, setDropTargetSkillId] = useState<string | null>(null);
  const skillsScrollRef = useRememberedScrollPosition(`scenes:card:${scene.id}:skills`);
  const enabledSkillCount = getSceneEnabledSkillCount(scene, skills);
  const orderedEnabled = useMemo(
    () => getOrderedEnabledSkillSummaries(scene, skills),
    [scene, skills],
  );
  const disabledSkills = useMemo(
    () => skills.filter((skill) => !isSceneSkillEnabled(scene, skill.id)),
    [scene, skills],
  );
  const externalGroupSummaries = useMemo(
    () => buildExternalGroupSummaries(skills, t("scenes.card.externalGroups.all")),
    [skills, t],
  );
  const selectedExternalGroup =
    externalGroupSummaries.find((summary) => summary.key === externalGroupFilter) ??
    externalGroupSummaries[0];
  const visibleEnabled = useMemo(
    () =>
      filterSceneSkills({
        externalGroupFilter,
        search,
        skills: orderedEnabled,
        sourceFilter,
        statusFilter,
        wantedStatus: "enabled",
      }),
    [externalGroupFilter, orderedEnabled, search, sourceFilter, statusFilter],
  );
  const visibleDisabled = useMemo(
    () =>
      filterSceneSkills({
        externalGroupFilter,
        search,
        skills: disabledSkills,
        sourceFilter,
        statusFilter,
        wantedStatus: "disabled",
      }),
    [disabledSkills, externalGroupFilter, search, sourceFilter, statusFilter],
  );
  const visibleCount = visibleEnabled.length + visibleDisabled.length;
  const sourceCounts = useMemo(() => buildSceneSkillSourceCounts(skills), [skills]);
  const statusCounts = {
    all: skills.length,
    disabled: skills.length - enabledSkillCount,
    enabled: enabledSkillCount,
  };

  const resetDragState = () => {
    setDraggedSkillId(null);
    setDropTargetSkillId(null);
  };

  const handleDragStart = (event: DragEvent<HTMLDivElement>, skillId: string) => {
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", skillId);
    setDraggedSkillId(skillId);
    setDropTargetSkillId(skillId);
  };

  const handleDrop = (event: DragEvent<HTMLDivElement>, targetSkillId: string) => {
    const sourceSkillId = event.dataTransfer.getData("text/plain") || draggedSkillId;
    if (!sourceSkillId) return;
    if (sourceSkillId !== targetSkillId) {
      onReorderSkill(sourceSkillId, targetSkillId);
    }
    resetDragState();
  };

  const sourceButtonClass = (filter: SceneSkillSourceFilter) =>
    pillClass(sourceFilter === filter);
  const statusButtonClass = (filter: SceneSkillStatusFilter) =>
    pillClass(statusFilter === filter);

  const changeSourceFilter = (filter: SceneSkillSourceFilter) => {
    setSourceFilter(filter);
    setIsExternalGroupOpen(filter === "external");
    if (filter !== "external") {
      setExternalGroupFilter("all");
    }
  };

  return (
    <div className="relative overflow-visible rounded-lg border border-slate-800/80 bg-slate-950/35">
      <div className="space-y-2 border-b border-slate-800/80 px-3 py-2">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div className="text-xs font-medium text-slate-200">
            {t("scenes.card.skillsPanel", {
              enabled: enabledSkillCount,
              total: skills.length,
            })}
            <span className="ml-2 text-[11px] text-slate-500">
              {t("scenes.card.visibleSkills", { count: visibleCount })}
            </span>
          </div>
          <span className="text-[11px] text-slate-500">
            {t("scenes.card.dragHint")}
          </span>
        </div>

        <label className="flex items-center gap-2 rounded-lg border border-slate-800 bg-slate-950 px-3 py-2">
          <Search className="h-3.5 w-3.5 shrink-0 text-slate-500" />
          <input
            className="min-w-0 flex-1 bg-transparent text-xs text-slate-100 outline-none placeholder:text-slate-600"
            onChange={(event) => setSearch(event.target.value)}
            placeholder={t("scenes.card.searchSkills")}
            value={search}
          />
        </label>

        <div className="flex flex-wrap items-center gap-2 text-[11px] text-slate-400">
          {(["all", "enabled", "disabled"] as const).map((filter) => (
            <button
              className={statusButtonClass(filter)}
              key={filter}
              onClick={() => setStatusFilter(filter)}
              type="button"
            >
              {t(`scenes.card.status.${filter}`, { count: statusCounts[filter] })}
            </button>
          ))}
        </div>

        <div className="relative flex flex-wrap items-center gap-2 text-[11px] text-slate-400">
          {(["all", "custom", "external"] as const).map((filter) => (
            <button
              className={sourceButtonClass(filter)}
              key={filter}
              onClick={() => changeSourceFilter(filter)}
              type="button"
            >
              {t(`scenes.card.source.${filter}`, { count: sourceCounts[filter] })}
            </button>
          ))}
          {sourceFilter === "external" ? (
            <span className="relative inline-flex">
              <button
                className={pillClass(isExternalGroupOpen)}
                onClick={() => setIsExternalGroupOpen((current) => !current)}
                type="button"
              >
                {selectedExternalGroup
                  ? `${selectedExternalGroup.label} · ${selectedExternalGroup.count}`
                  : t("scenes.card.externalGroups.empty")}
              </button>
              {isExternalGroupOpen ? (
                <div className="absolute bottom-full left-0 z-50 mb-2 max-h-64 min-w-72 overflow-hidden rounded-xl border border-slate-700 bg-slate-950 shadow-2xl shadow-slate-950/70">
                  <div className="skill-markdown-scroll max-h-64 overflow-y-auto p-2">
                    {externalGroupSummaries.map((summary) => (
                      <button
                        className={`flex w-full items-center justify-between gap-3 rounded-lg px-3 py-2 text-left text-xs ${
                          externalGroupFilter === summary.key
                            ? "bg-sky-500/15 text-sky-100"
                            : "text-slate-300 hover:bg-slate-900"
                        }`}
                        key={summary.key}
                        onClick={() => {
                          setExternalGroupFilter(summary.key);
                          setIsExternalGroupOpen(false);
                        }}
                        type="button"
                      >
                        <span className="min-w-0 flex-1 truncate">{summary.label}</span>
                        <span className="text-slate-500">{summary.count}</span>
                      </button>
                    ))}
                  </div>
                </div>
              ) : null}
            </span>
          ) : null}
        </div>
      </div>

      <div
        className="skill-markdown-scroll grid max-h-[18rem] grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-x-3 gap-y-1 overflow-y-auto px-3 py-2 pr-4"
        ref={skillsScrollRef}
      >
        {visibleCount === 0 ? (
          <div className="col-span-full rounded-lg border border-dashed border-slate-800 px-3 py-4 text-xs text-slate-500">
            {t("scenes.card.noMatchingSkills")}
          </div>
        ) : null}
        {visibleEnabled.map((skill) => (
          <div
            className={`flex h-8 min-w-0 items-center gap-2 rounded-md px-2 text-xs text-slate-300 ${
              dropTargetSkillId === skill.id && draggedSkillId !== skill.id
                ? "bg-sky-950/40 ring-1 ring-sky-700"
                : ""
            } ${draggedSkillId === skill.id ? "opacity-60" : ""}`}
            draggable
            key={skill.id}
            onDragEnd={resetDragState}
            onDragEnter={() => setDropTargetSkillId(skill.id)}
            onDragOver={(event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "move";
              setDropTargetSkillId(skill.id);
            }}
            onDragStart={(event) => handleDragStart(event, skill.id)}
            onDrop={(event) => {
              event.preventDefault();
              handleDrop(event, skill.id);
            }}
          >
            <button
              aria-label={skill.name}
              className="shrink-0 text-slate-400 hover:text-sky-400"
              onClick={() => onToggleSkill(skill.id)}
              title={`${t("tooltip.scenes.toggleSkill")}: ${skill.name}`}
              type="button"
            >
              <CheckSquare className="h-4 w-4" />
            </button>
            <span className="shrink-0" title={t("tooltip.scenes.gripDrag")}>
              <GripVertical className="h-3.5 w-3.5 cursor-grab text-slate-500 active:cursor-grabbing" />
            </span>
            <span className="min-w-0 flex-1 truncate" title={skill.relativePath}>
              {skill.name}
            </span>
          </div>
        ))}
        {visibleDisabled.map((skill) => (
          <div
            className="flex h-8 min-w-0 items-center gap-2 rounded-md px-2 text-xs text-slate-500"
            key={skill.id}
          >
            <button
              aria-label={skill.name}
              className="shrink-0 text-slate-400 hover:text-sky-400"
              onClick={() => onToggleSkill(skill.id)}
              title={`${t("tooltip.scenes.toggleSkill")}: ${skill.name}`}
              type="button"
            >
              <Square className="h-4 w-4" />
            </button>
            <span className="min-w-0 flex-1 truncate" title={skill.relativePath}>
              {skill.name}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function pillClass(active: boolean) {
  return `rounded-full border px-2 py-1 transition ${
    active
      ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
      : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
  }`;
}
