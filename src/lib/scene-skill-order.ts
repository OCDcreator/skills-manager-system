import type { SceneEntry } from "./scenes";
import type { SkillSummary } from "./tauri";

type SceneSkill = Pick<SkillSummary, "id" | "name">;

export function isSceneSkillEnabled(scene: SceneEntry, skillId: string) {
  if (scene.skillSelectionMode === "onlySelected") {
    return scene.selectedSkillIds.includes(skillId);
  }
  return !scene.disabledSkillIds.includes(skillId);
}

export function getSceneEnabledSkillCount(scene: SceneEntry, skills: SceneSkill[]) {
  return skills.filter((skill) => isSceneSkillEnabled(scene, skill.id)).length;
}

export function getOrderedEnabledSceneSkills(
  scene: SceneEntry,
  skills: SceneSkill[],
) {
  const orderIndex = new Map(scene.skillOrder.map((id, index) => [id, index]));

  return skills
    .filter((skill) => isSceneSkillEnabled(scene, skill.id))
    .slice()
    .sort((left, right) => {
      const leftIndex = orderIndex.get(left.id) ?? Number.MAX_SAFE_INTEGER;
      const rightIndex = orderIndex.get(right.id) ?? Number.MAX_SAFE_INTEGER;
      if (leftIndex !== rightIndex) {
        return leftIndex - rightIndex;
      }
      return left.name.localeCompare(right.name);
    });
}

export function reorderSceneSkillOrder(
  scene: SceneEntry,
  skills: SceneSkill[],
  draggedSkillId: string,
  targetSkillId: string,
) {
  if (draggedSkillId === targetSkillId) {
    return null;
  }

  const orderedSkills = getOrderedEnabledSceneSkills(scene, skills);
  const fromIndex = orderedSkills.findIndex((skill) => skill.id === draggedSkillId);
  const toIndex = orderedSkills.findIndex((skill) => skill.id === targetSkillId);
  if (fromIndex < 0 || toIndex < 0) {
    return null;
  }

  const nextOrder = [...orderedSkills];
  const [movedSkill] = nextOrder.splice(fromIndex, 1);
  nextOrder.splice(toIndex, 0, movedSkill);
  return nextOrder.map((skill) => skill.id);
}
