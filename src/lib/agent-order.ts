import type { AgentInventoryItem, AgentKey } from "./tauri";

function orderIndex(agentOrder: AgentKey[]) {
  return new Map(agentOrder.map((key, index) => [key, index]));
}

export function resolveSortedAgentInventory<
  T extends Pick<AgentInventoryItem, "key" | "enabled">,
>(agents: T[], agentOrder: AgentKey[]) {
  const indexMap = orderIndex(agentOrder);

  return [...agents].sort((left, right) => {
    if (left.enabled !== right.enabled) {
      return left.enabled ? -1 : 1;
    }

    const leftIndex = indexMap.get(left.key as AgentKey);
    const rightIndex = indexMap.get(right.key as AgentKey);

    if (leftIndex != null && rightIndex != null) {
      return leftIndex - rightIndex;
    }
    if (leftIndex != null) return -1;
    if (rightIndex != null) return 1;
    return 0;
  });
}

export function applyDraggedAgentOrder(
  currentOrder: AgentKey[],
  draggedKey: AgentKey,
  targetKey: AgentKey,
) {
  const next = currentOrder.filter((key) => key !== draggedKey);
  const targetIndex = next.indexOf(targetKey);

  if (targetIndex === -1) {
    next.push(draggedKey);
    return next;
  }

  next.splice(targetIndex, 0, draggedKey);
  return next;
}

export function mergeAgentOrderWithInventory(
  agents: Pick<AgentInventoryItem, "key">[],
  persistedOrder: AgentKey[],
) {
  const knownKeys = new Set(agents.map((agent) => agent.key));
  const preserved = persistedOrder.filter((key) => knownKeys.has(key));
  const missing = agents.map((agent) => agent.key).filter((key) => !preserved.includes(key));
  return [...preserved, ...missing] as AgentKey[];
}
