import type { SkillSummary } from "../tauri";

export type SourceFilter = "all" | "custom" | "external";
export type VisibleSource = Exclude<SourceFilter, "all">;
export type SkillStatusFilter = "all" | "enabled" | "disabled";

export interface SourceSummary {
  key: SourceFilter;
  count: number;
}

export interface StatusSummary {
  key: SkillStatusFilter;
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

export function buildStatusSummaries(
  skills: SkillSummary[],
  disabledSkillIds: ReadonlySet<string>,
): StatusSummary[] {
  const disabled = skills.filter((skill) => disabledSkillIds.has(skill.id)).length;
  const enabled = skills.length - disabled;

  return [
    { key: "all", count: skills.length },
    { key: "enabled", count: enabled },
    { key: "disabled", count: disabled },
  ];
}

export function filterSkills(
  skills: SkillSummary[],
  search: string,
  sourceFilter: SourceFilter,
  statusFilter: SkillStatusFilter,
  disabledSkillIds: ReadonlySet<string>,
) {
  const lowered = search.trim().toLowerCase();

  return skills.filter((skill) => {
    if (sourceFilter !== "all" && skill.sourceType !== sourceFilter) {
      return false;
    }

    const isDisabled = disabledSkillIds.has(skill.id);
    if (statusFilter === "enabled" && isDisabled) {
      return false;
    }

    if (statusFilter === "disabled" && !isDisabled) {
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

export function resolveVisibleSources(sourceFilter: SourceFilter): VisibleSource[] {
  if (sourceFilter === "all") {
    return ["custom", "external"];
  }

  return [sourceFilter];
}

export function truncateDescription(description: string, maxLength = 100) {
  if (description.length <= maxLength) {
    return description;
  }

  return `${description.slice(0, maxLength - 1)}…`;
}
