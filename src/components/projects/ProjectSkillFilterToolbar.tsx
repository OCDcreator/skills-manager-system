import { useState } from "react";
import { Search } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { ProjectSkillSelectionFilter } from "../../lib/project-filters";
import type {
  ExternalGroupFilter,
  ExternalGroupSummary,
} from "../../lib/scene-skill-filters";
import type { SkillPathFilter, SkillPathSummary } from "../../lib/skills/filters";

interface ProjectSkillFilterToolbarProps {
  externalGroupFilter: ExternalGroupFilter;
  externalGroupSummaries: ExternalGroupSummary[];
  skillPathFilter: SkillPathFilter;
  skillPathSummaries: SkillPathSummary[];
  skillQuery: string;
  skillSelectionFilter: ProjectSkillSelectionFilter;
  onExternalGroupFilterChange: (value: ExternalGroupFilter) => void;
  onSkillPathFilterChange: (value: SkillPathFilter) => void;
  onSkillQueryChange: (value: string) => void;
  onSkillSelectionFilterChange: (value: ProjectSkillSelectionFilter) => void;
}

export function ProjectSkillFilterToolbar(props: ProjectSkillFilterToolbarProps) {
  const { t } = useTranslation();
  const [isExternalGroupOpen, setIsExternalGroupOpen] = useState(false);
  const selectedExternalGroup =
    props.externalGroupSummaries.find(
      (summary) => summary.key === props.externalGroupFilter,
    ) ?? props.externalGroupSummaries[0];
  const filterPillClass = (active: boolean) =>
    `rounded-full border px-2 py-1 transition ${
      active
        ? "border-sky-500/60 bg-sky-500/15 text-sky-100"
        : "border-slate-700 bg-slate-900 text-slate-300 hover:bg-slate-800"
    }`;

  const changeSkillPathFilter = (filter: SkillPathFilter) => {
    props.onSkillPathFilterChange(filter);
    setIsExternalGroupOpen(filter === "external");
    if (filter !== "external") {
      props.onExternalGroupFilterChange("all");
    }
  };

  return (
    <div className="border-b border-slate-800 px-4 py-3">
      <label className="flex h-9 min-w-[13rem] flex-1 items-center gap-2 rounded-lg border border-slate-700 bg-slate-900 px-3">
        <Search className="h-3.5 w-3.5 shrink-0 text-slate-500" />
        <input
          className="min-w-0 flex-1 bg-transparent text-xs text-slate-200 outline-none placeholder:text-slate-600"
          onChange={(event) => props.onSkillQueryChange(event.target.value)}
          placeholder={t("projects.editor.searchSkills")}
          value={props.skillQuery}
        />
      </label>
      <div className="relative mt-3 flex flex-wrap items-center gap-2 text-[11px] text-slate-400">
        {props.skillPathSummaries.map((summary) => (
          <button
            className={filterPillClass(props.skillPathFilter === summary.key)}
            key={summary.key}
            onClick={() => changeSkillPathFilter(summary.key)}
            type="button"
          >
            {summary.key === "all"
              ? t("projects.editor.allPaths", { count: summary.count })
              : `${summary.key} · ${summary.count}`}
          </button>
        ))}
        {props.skillPathFilter === "external" ? (
          <span className="relative inline-flex">
            <button
              className={filterPillClass(isExternalGroupOpen)}
              onClick={() => setIsExternalGroupOpen((current) => !current)}
              type="button"
            >
              {selectedExternalGroup
                ? `${selectedExternalGroup.label} · ${selectedExternalGroup.count}`
                : t("projects.editor.externalGroups.empty")}
            </button>
            {isExternalGroupOpen ? (
              <div className="absolute bottom-full left-0 z-50 mb-2 max-h-64 min-w-72 overflow-hidden rounded-xl border border-slate-700 bg-slate-950 shadow-2xl shadow-slate-950/70">
                <div className="skill-markdown-scroll max-h-64 overflow-y-auto p-2">
                  {props.externalGroupSummaries.map((summary) => (
                    <button
                      className={`flex w-full items-center justify-between gap-3 rounded-lg px-3 py-2 text-left text-xs ${
                        props.externalGroupFilter === summary.key
                          ? "bg-sky-500/15 text-sky-100"
                          : "text-slate-300 hover:bg-slate-900"
                      }`}
                      key={summary.key}
                      onClick={() => {
                        props.onExternalGroupFilterChange(summary.key);
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
        {(["all", "selected", "unselected"] as const).map((status) => (
          <button
            className={filterPillClass(props.skillSelectionFilter === status)}
            key={status}
            onClick={() => props.onSkillSelectionFilterChange(status)}
            type="button"
          >
            {status === "all"
              ? t("projects.editor.allSkills")
              : status === "selected"
                ? t("projects.editor.selectedSkills")
                : t("projects.editor.unselectedSkills")}
          </button>
        ))}
      </div>
    </div>
  );
}
