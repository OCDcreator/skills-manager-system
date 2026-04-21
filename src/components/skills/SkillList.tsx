import { useTranslation } from "react-i18next";
import type { SkillSummary } from "../../lib/tauri";
import { truncateDescription } from "../../lib/skills/filters";

interface SkillListProps {
  title: string;
  skills: SkillSummary[];
  selectedSkillId: string | null;
  disabledSkillIds: ReadonlySet<string>;
  updatingSkillId: string | null;
  onSelect: (skill: SkillSummary) => void;
  onToggleEnabled: (skillId: string, enabled: boolean) => Promise<void>;
}

export function SkillList(props: SkillListProps) {
  const {
    disabledSkillIds,
    onSelect,
    onToggleEnabled,
    selectedSkillId,
    skills,
    title,
    updatingSkillId,
  } = props;
  const { t } = useTranslation();

  return (
    <section className="space-y-3 rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <header className="flex items-center justify-between">
        <h3 className="text-sm font-semibold uppercase tracking-wide text-slate-300">{title}</h3>
        <span className="text-xs text-slate-500">{skills.length}</span>
      </header>
      <div className="space-y-3">
        {skills.length === 0 ? (
          <div className="rounded-xl border border-dashed border-slate-700 px-4 py-6 text-sm text-slate-500">
            {t("skills.empty")}
          </div>
        ) : null}
        {skills.map((skill) => {
          const isDisabled = disabledSkillIds.has(skill.id);
          const isUpdating = updatingSkillId === skill.id;

          return (
            <article
              key={skill.id}
              className={`rounded-xl border p-4 transition ${
                selectedSkillId === skill.id
                  ? "border-sky-400 bg-sky-400/10"
                  : "border-slate-800 bg-slate-950"
              } ${isDisabled ? "opacity-70" : ""}`}
            >
              <div className="flex items-start justify-between gap-3">
                <button
                  className="min-w-0 flex-1 text-left"
                  onClick={() => onSelect(skill)}
                  title={t("tooltip.skills.select")}
                  type="button"
                >
                  <div className="flex flex-wrap items-center gap-2">
                    <strong className="text-sm font-semibold text-slate-100">{skill.name}</strong>
                    <span className="rounded-full bg-slate-800 px-2 py-1 text-xs text-slate-300">
                      {skill.sourceType === "custom"
                        ? t("skills.source.custom")
                        : t("skills.source.external")}
                    </span>
                    <span
                      className={`rounded-full px-2 py-1 text-xs ${
                        isDisabled
                          ? "bg-amber-500/15 text-amber-200"
                          : "bg-emerald-500/15 text-emerald-200"
                      }`}
                    >
                      {isDisabled
                        ? t("skills.status.disabled")
                        : t("skills.status.enabled")}
                    </span>
                  </div>
                  <p className="mt-2 text-sm text-slate-400">
                    {skill.description
                      ? truncateDescription(skill.description)
                      : t("skills.noDescription")}
                  </p>
                  <p className="mt-2 text-xs text-slate-500">{skill.relativePath}</p>
                </button>

                <button
                  className="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 disabled:opacity-60"
                  disabled={isUpdating}
                  onClick={() => void onToggleEnabled(skill.id, isDisabled)}
                  title={isDisabled ? t("tooltip.skills.toggle.enable") : t("tooltip.skills.toggle.disable")}
                  type="button"
                >
                  {isDisabled ? t("skills.toggle.enable") : t("skills.toggle.disable")}
                </button>
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}
