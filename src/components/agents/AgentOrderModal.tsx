import { ArrowDownUp, GripVertical, X } from "lucide-react";
import { useMemo, useRef, useState, type PointerEvent } from "react";
import { useTranslation } from "react-i18next";
import {
  applyDraggedAgentOrder,
  mergeAgentOrderWithInventory,
  resolveSortedAgentInventory,
} from "../../lib/agent-order";
import type { AgentInventoryItem, AgentKey } from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";

interface AgentOrderModalProps {
  agents: AgentInventoryItem[];
  initialOrder: AgentKey[];
  isSaving: boolean;
  onClose: () => void;
  onSave: (agentOrder: AgentKey[]) => Promise<void>;
}

export function AgentOrderModal({
  agents,
  initialOrder,
  isSaving,
  onClose,
  onSave,
}: AgentOrderModalProps) {
  const { t } = useTranslation();
  const [draftOrder, setDraftOrder] = useState<AgentKey[]>(
    mergeAgentOrderWithInventory(agents, initialOrder),
  );
  const [draggedKey, setDraggedKey] = useState<AgentKey | null>(null);
  const [dropTargetKey, setDropTargetKey] = useState<AgentKey | null>(null);
  const draggedKeyRef = useRef<AgentKey | null>(null);
  const dragPointerIdRef = useRef<number | null>(null);

  const orderedAgents = useMemo(
    () => resolveSortedAgentInventory(agents, draftOrder),
    [agents, draftOrder],
  );

  const sections = useMemo(
    () => [
      {
        key: "enabled",
        label: t("agents.orderModal.enabledSection"),
        items: orderedAgents.filter((agent) => agent.enabled),
      },
      {
        key: "disabled",
        label: t("agents.orderModal.disabledSection"),
        items: orderedAgents.filter((agent) => !agent.enabled),
      },
    ],
    [orderedAgents, t],
  );

  const resetDragState = () => {
    draggedKeyRef.current = null;
    dragPointerIdRef.current = null;
    setDraggedKey(null);
    setDropTargetKey(null);
  };

  const moveDraggedAgentTo = (targetKey: AgentKey) => {
    const activeDraggedKey = draggedKeyRef.current;
    if (!activeDraggedKey || activeDraggedKey === targetKey) {
      setDropTargetKey(targetKey);
      return;
    }

    setDraftOrder((current) =>
      applyDraggedAgentOrder(current, activeDraggedKey, targetKey),
    );
    setDropTargetKey(targetKey);
  };

  const resolvePointerTargetKey = (event: PointerEvent<HTMLElement>) => {
    const element = document.elementFromPoint(event.clientX, event.clientY);
    const row = element?.closest<HTMLElement>("[data-agent-order-key]");
    return row?.dataset.agentOrderKey as AgentKey | undefined;
  };

  const handlePointerDragStart = (
    event: PointerEvent<HTMLButtonElement>,
    agentKey: AgentKey,
  ) => {
    if (event.button !== 0) return;

    event.preventDefault();
    event.currentTarget.setPointerCapture(event.pointerId);
    draggedKeyRef.current = agentKey;
    dragPointerIdRef.current = event.pointerId;
    setDraggedKey(agentKey);
    setDropTargetKey(agentKey);
  };

  const handlePointerDragMove = (event: PointerEvent<HTMLButtonElement>) => {
    if (dragPointerIdRef.current !== event.pointerId) return;

    event.preventDefault();
    const targetKey = resolvePointerTargetKey(event);
    if (targetKey) {
      moveDraggedAgentTo(targetKey);
    }
  };

  const handlePointerDragEnd = (event: PointerEvent<HTMLButtonElement>) => {
    if (dragPointerIdRef.current !== event.pointerId) return;

    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
    resetDragState();
  };

  return (
    <div className="fixed inset-0 z-[70] flex items-center justify-center bg-slate-950/75 p-4 backdrop-blur-sm">
      <div className="flex max-h-[calc(100vh-2rem)] w-full max-w-xl flex-col overflow-hidden rounded-2xl border border-slate-800 bg-slate-900 shadow-2xl shadow-slate-950/60">
        <div className="flex items-start justify-between gap-4 border-b border-slate-800 px-5 py-4">
          <div>
            <div className="flex items-center gap-2 text-sky-300">
              <ArrowDownUp className="h-4 w-4" />
              <h2 className="text-base font-semibold text-slate-100">
                {t("agents.orderModal.title")}
              </h2>
            </div>
            <p className="mt-2 text-sm text-slate-400">
              {t("agents.orderModal.description")}
            </p>
          </div>
          <button
            className="rounded-lg p-2 text-slate-400 hover:bg-slate-800 hover:text-slate-100"
            onClick={onClose}
            type="button"
          >
            <X className="h-4 w-4" />
          </button>
        </div>

        <div className="skill-markdown-scroll min-h-0 flex-1 space-y-5 overflow-y-auto px-5 py-4">
          {sections.map((section) => (
            <section key={section.key}>
              <div className="mb-2 flex items-center justify-between gap-2">
                <div className="text-xs font-medium uppercase tracking-[0.18em] text-slate-500">
                  {section.label}
                </div>
                <div className="text-[11px] text-slate-500">
                  {t("agents.orderModal.dragHint")}
                </div>
              </div>

              <div className="space-y-2">
                {section.items.map((agent) => {
                  const isDropTarget =
                    dropTargetKey === agent.key && draggedKey !== agent.key;

                  return (
                    <div
                      className={`flex items-center gap-3 rounded-xl border px-3 py-3 text-sm transition ${
                        isDropTarget
                          ? "border-sky-600 bg-sky-950/30 text-slate-100"
                          : "border-slate-800 bg-slate-950/60 text-slate-200"
                      } ${draggedKey === agent.key ? "opacity-60" : ""}`}
                      data-agent-order-key={agent.key}
                      key={agent.key}
                    >
                      <button
                        aria-label={`${t("agents.orderModal.dragHint")}: ${agent.displayName}`}
                        className="touch-none rounded p-1 text-slate-500 hover:bg-slate-800 hover:text-sky-300 active:cursor-grabbing"
                        onPointerCancel={handlePointerDragEnd}
                        onPointerDown={(event) =>
                          handlePointerDragStart(event, agent.key)
                        }
                        onPointerMove={handlePointerDragMove}
                        onPointerUp={handlePointerDragEnd}
                        title={`${t("agents.orderModal.dragHint")}: ${agent.displayName}`}
                        type="button"
                      >
                        <GripVertical className="h-4 w-4 cursor-grab" />
                      </button>
                      <div className="grid h-9 w-9 place-items-center rounded-full border border-slate-700 bg-slate-900">
                        <AgentBrandIcon agentKey={agent.key} className="size-[65%]" />
                      </div>
                      <div className="min-w-0 flex-1">
                        <div className="truncate font-medium text-slate-100">
                          {agent.displayName}
                        </div>
                        <div className="text-xs text-slate-500">
                          {agent.enabled
                            ? t("agents.orderModal.enabledBadge")
                            : t("agents.orderModal.disabledBadge")}
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            </section>
          ))}
        </div>

        <div className="flex justify-end gap-2 border-t border-slate-800 px-5 py-4">
          <button
            className="rounded-lg border border-slate-700 px-4 py-2 text-sm text-slate-200"
            onClick={onClose}
            type="button"
          >
            {t("agents.orderModal.cancel")}
          </button>
          <button
            className="rounded-lg bg-sky-400 px-4 py-2 text-sm font-semibold text-slate-950 disabled:opacity-60"
            disabled={isSaving}
            onClick={() => void onSave(draftOrder)}
            type="button"
          >
            {isSaving ? t("agents.orderModal.saving") : t("agents.orderModal.save")}
          </button>
        </div>
      </div>
    </div>
  );
}
