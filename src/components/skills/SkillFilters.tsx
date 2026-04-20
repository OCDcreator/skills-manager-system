import { useTranslation } from "react-i18next";
import type { SourceFilter, SourceSummary } from "../../lib/skills/filters";

interface SkillFiltersProps {
  search: string;
  onSearchChange: (value: string) => void;
  sourceFilter: SourceFilter;
  onSourceFilterChange: (value: SourceFilter) => void;
  summaries: SourceSummary[];
  onRefresh: () => Promise<void>;
  isRefreshing: boolean;
}

export function SkillFilters(props: SkillFiltersProps) {
  const {
    isRefreshing,
    onRefresh,
    onSearchChange,
    onSourceFilterChange,
    search,
    sourceFilter,
    summaries,
  } = props;
  const { t } = useTranslation();

  return (
    <div className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <div className="flex gap-3">
        <input
          className="flex-1 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
          onChange={(event) => onSearchChange(event.target.value)}
          placeholder={t("skills.search")}
          value={search}
        />
        <button
          className="rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm text-slate-100 disabled:opacity-60"
          disabled={isRefreshing}
          onClick={() => void onRefresh()}
          type="button"
        >
          {t("skills.refresh")}
        </button>
      </div>
      <div className="flex flex-wrap gap-2">
        {summaries.map((summary) => (
          <button
            key={summary.key}
            className={`rounded-full px-3 py-1.5 text-sm ${
              sourceFilter === summary.key
                ? "bg-sky-400 text-slate-950"
                : "bg-slate-800 text-slate-200"
            }`}
            onClick={() => onSourceFilterChange(summary.key)}
            type="button"
          >
            {t(`skills.source.${summary.key}`)} ({summary.count})
          </button>
        ))}
      </div>
    </div>
  );
}
