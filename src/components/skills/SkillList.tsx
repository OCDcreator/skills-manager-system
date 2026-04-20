import { useTranslation } from "react-i18next";
import type { SkillSummary } from "../../lib/tauri";
import { truncateDescription } from "../../lib/skills/filters";

interface SkillListProps {
  title: string;
  skills: SkillSummary[];
  selectedSkillId: string | null;
  onSelect: (skill: SkillSummary) => void;
}

export function SkillList({ onSelect, selectedSkillId, skills, title }: SkillListProps) {
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
        {skills.map((skill) => (
          <button
            key={skill.id}
            className={`w-full rounded-xl border p-4 text-left ${
              selectedSkillId === skill.id
                ? "border-sky-400 bg-sky-400/10"
                : "border-slate-800 bg-slate-950"
            }`}
            onClick={() => onSelect(skill)}
            type="button"
          >
            <div className="flex items-center justify-between gap-3">
              <strong className="text-sm font-semibold text-slate-100">{skill.name}</strong>
              <span className="rounded-full bg-slate-800 px-2 py-1 text-xs text-slate-300">
                {skill.sourceType === "custom"
                  ? t("skills.source.custom")
                  : t("skills.source.external")}
              </span>
            </div>
            <p className="mt-2 text-sm text-slate-400">
              {skill.description
                ? truncateDescription(skill.description)
                : t("skills.noDescription")}
            </p>
            <p className="mt-2 text-xs text-slate-500">{skill.relativePath}</p>
          </button>
        ))}
      </div>
    </section>
  );
}
