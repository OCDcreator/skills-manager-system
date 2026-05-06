import { getOrderedEnabledSceneSkills } from "./scene-skill-order";
import type { SceneEntry } from "./scenes";
import type { SkillSummary } from "./tauri";

export type SceneSkillStatusFilter = "all" | "enabled" | "disabled";
export type SceneSkillSourceFilter = "all" | "custom" | "external";
export type ExternalGroupFilter = "all" | string;

export interface ExternalGroupSummary {
  key: ExternalGroupFilter;
  label: string;
  count: number;
}

export function getOrderedEnabledSkillSummaries(
  scene: SceneEntry,
  skills: SkillSummary[],
) {
  const skillById = new Map(skills.map((skill) => [skill.id, skill]));
  return getOrderedEnabledSceneSkills(scene, skills)
    .map((skill) => skillById.get(skill.id))
    .filter((skill): skill is SkillSummary => Boolean(skill));
}

export function filterSceneSkills({
  externalGroupFilter,
  search,
  skills,
  sourceFilter,
  statusFilter,
  wantedStatus,
}: {
  externalGroupFilter: ExternalGroupFilter;
  search: string;
  skills: SkillSummary[];
  sourceFilter: SceneSkillSourceFilter;
  statusFilter: SceneSkillStatusFilter;
  wantedStatus: Exclude<SceneSkillStatusFilter, "all">;
}) {
  const lowered = search.trim().toLowerCase();
  if (statusFilter !== "all" && statusFilter !== wantedStatus) {
    return [];
  }

  return skills.filter((skill) => {
    if (sourceFilter !== "all" && skill.sourceType !== sourceFilter) {
      return false;
    }
    if (
      sourceFilter === "external" &&
      externalGroupFilter !== "all" &&
      getExternalGroupKey(skill) !== externalGroupFilter
    ) {
      return false;
    }
    if (!lowered) {
      return true;
    }
    return [skill.name, skill.description, skill.relativePath]
      .join(" ")
      .toLowerCase()
      .includes(lowered);
  });
}

export function buildSceneSkillSourceCounts(skills: SkillSummary[]) {
  return {
    all: skills.length,
    custom: skills.filter((skill) => skill.sourceType === "custom").length,
    external: skills.filter((skill) => skill.sourceType === "external").length,
  };
}

export function buildExternalGroupSummaries(
  skills: SkillSummary[],
  allExternalLabel: string,
): ExternalGroupSummary[] {
  const counts = new Map<ExternalGroupFilter, { count: number; label: string }>();
  const externalSkills = skills.filter((skill) => skill.sourceType === "external");
  for (const skill of externalSkills) {
    const key = getExternalGroupKey(skill);
    const current = counts.get(key);
    counts.set(key, {
      count: (current?.count ?? 0) + 1,
      label: current?.label ?? getExternalGroupLabel(skill),
    });
  }

  return [
    { key: "all", label: allExternalLabel, count: externalSkills.length },
    ...[...counts.entries()]
      .map(([key, value]) => ({ key, ...value }))
      .sort((left, right) => left.label.localeCompare(right.label)),
  ];
}

export function getExternalGroupKey(skill: SkillSummary): ExternalGroupFilter {
  if (skill.managedSource) {
    return `managed:${skill.managedSource.importId}`;
  }
  const segments = skill.relativePath.split(/[\\/]/).filter(Boolean);
  return segments.slice(0, 2).join("/") || skill.relativePath;
}

function getExternalGroupLabel(skill: SkillSummary) {
  if (skill.managedSource) {
    return `${repoLabel(skill.managedSource.repoUrl)} · ${skill.managedSource.agentKey}`;
  }
  const segments = skill.relativePath.split(/[\\/]/).filter(Boolean);
  return segments[1] ?? skill.relativePath;
}

function repoLabel(repoUrl: string) {
  const trimmed = repoUrl.replace(/\.git$/, "").replace(/[\\/]$/, "");
  const match = trimmed.match(/[:/]([^/:/]+\/[^/:/]+)$/);
  return match?.[1] ?? trimmed;
}
