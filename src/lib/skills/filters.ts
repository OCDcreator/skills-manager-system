import type { SkillSummary } from "../tauri";

export type SourceFilter = "all" | "custom" | "external";

export interface SourceSummary {
  key: SourceFilter;
  count: number;
}

export function buildSourceSummaries(skills: SkillSummary[]): SourceSummary[] {
  const custom = skills.filter((skill) => skill.sourceType === "custom").length;
  const external = skills.filter((skill) => skill.sourceType === "external").length;

  return [
    { key: "all", count: skills.length },
    { key: "custom", count: custom },
    { key: "external", count: external },
  ];
}

export function filterSkills(
  skills: SkillSummary[],
  search: string,
  sourceFilter: SourceFilter,
) {
  const lowered = search.trim().toLowerCase();

  return skills.filter((skill) => {
    if (sourceFilter !== "all" && skill.sourceType !== sourceFilter) {
      return false;
    }

    if (!lowered) {
      return true;
    }

    return (
      skill.name.toLowerCase().includes(lowered) ||
      skill.description.toLowerCase().includes(lowered) ||
      skill.relativePath.toLowerCase().includes(lowered)
    );
  });
}

export function groupSkills(skills: SkillSummary[]) {
  return {
    custom: skills.filter((skill) => skill.sourceType === "custom"),
    external: skills.filter((skill) => skill.sourceType === "external"),
  };
}

export function truncateDescription(description: string, maxLength = 100) {
  if (description.length <= maxLength) {
    return description;
  }

  return `${description.slice(0, maxLength - 1)}…`;
}
