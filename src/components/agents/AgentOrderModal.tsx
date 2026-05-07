import { ArrowDownUp, GripVertical, X } from "lucide-react";
import { useMemo, useRef, useState, type PointerEvent } from "react";
import { useTranslation } from "react-i18next";
import {
  applyDraggedAgentOrder,
  mergeAgentOrderWithInventory,
  resolveSortedAgentInventory,
} from "../../lib/agent-order";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { AgentInventoryItem, AgentKey } from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";

interface AgentOrderModalProps {
  agents: AgentInventoryItem[];
  initialOrder: AgentKey[];
  isSaving: boolean;
  onClose: () => void;
  onSave: (agentOrder: AgentKey[]) => Promise<void>;
  onShowEnabledOnlyInSideNavChange: (enabled: boolean) => void;
  showEnabledOnlyInSideNav: boolean;
}

export function AgentOrderModal({
  agents,
  initialOrder,
  isSaving,
  onClose,
  onSave,
  onShowEnabledOnlyInSideNavChange,
  showEnabledOnlyInSideNav,
}: AgentOrderModalProps) {
  const { t } = useTranslation();
  const mergedOrder = useMemo(
    () => mergeAgentOrderWithInventory(agents, initialOrder),
    [agents, initialOrder],
  );
  const [draftOrder, setDraftOrder] = useState<AgentKey[]>(mergedOrder);
  const [draggedKey, setDraggedKey] = useState<AgentKey | null>(null);
  const [dropTargetKey, setDropTargetKey] = useState<AgentKey | null>(null);
  const draggedKeyRef = useRef<AgentKey | null>(null);
  const dragPointerIdRef = useRef<number | null>(null);
  const scrollRef = useRememberedScrollPosition("agents:order-modal");

  const orderedAgents = useMemo(
    () => resolveSortedAgentInventory(agents, draftOrder),
    [agents, draftOrder],
  );

  const sections = useMemo(
    () => [
      {
        key: "enabled",
        label: t("agents.orderModal.enabledSection"),
        helper: t("agents.orderModal.enabledSectionHint"),
        items: orderedAgents.filter((agent) => agent.enabled),
      },
      {
        key: "disabled",
        label: t("agents.orderModal.disabledSection"),
        helper: t("agents.orderModal.disabledSectionHint"),
        items: orderedAgents.filter((agent) => !agent.enabled),
      },
    ],
    [orderedAgents, t],
  );
  const enabledByKey = useMemo(
    () => new Map(agents.map((agent) => [agent.key, agent.enabled])),
    [agents],
  );
  const isDirty =
    draftOrder.length !== mergedOrder.length ||
    draftOrder.some((key, index) => key !== mergedOrder[index]);

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
    if (enabledByKey.get(activeDraggedKey) !== enabledByKey.get(targetKey)) {
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
    if (
      targetKey &&
      enabledByKey.get(draggedKeyRef.current ?? targetKey) === enabledByKey.get(targetKey)
    ) {
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
      <div className="flex max-h-[min(44rem,calc(100vh-2rem))] w-full max-w-[58rem] flex-col overflow-hidden rounded-[1.75rem] border border-slate-700/80 bg-[radial-gradient(circle_at_top,rgba(59,130,246,0.14),transparent_26%),linear-gradient(180deg,rgba(15,23,42,0.985),rgba(8,15,30,0.985))] shadow-2xl shadow-slate-950/70">
        <div className="border-b border-slate-800/90 px-6 py-5">
          <div className="flex items-start justify-between gap-4">
            <div className="min-w-0 flex-1">
              <div className="flex flex-wrap items-center gap-3 text-sky-300">
                <div className="grid h-9 w-9 place-items-center rounded-full border border-sky-400/20 bg-sky-400/10 text-sky-200 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]">
                  <ArrowDownUp className="h-4 w-4" />
                </div>
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-3">
                    <h2 className="text-lg font-semibold tracking-tight text-slate-50">
                      {t("agents.orderModal.title")}
                    </h2>
                    {isDirty ? (
                      <span className="inline-flex items-center gap-2 rounded-full border border-sky-400/20 bg-sky-400/10 px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.18em] text-sky-200">
                        <span className="h-1.5 w-1.5 rounded-full bg-sky-300" />
                        {t("agents.orderModal.dirtyBadge")}
                      </span>
                    ) : null}
                  </div>
                  <p className="mt-2 max-w-2xl text-sm leading-6 text-slate-400">
                    {t("agents.orderModal.description")}
                  </p>
                </div>
              </div>
            </div>
            <button
              className="rounded-full border border-slate-700/80 p-2 text-slate-400 transition hover:border-slate-500 hover:bg-slate-800/80 hover:text-slate-100"
              onClick={onClose}
              type="button"
            >
              <X className="h-4 w-4" />
            </button>
          </div>

          <label className="mt-5 flex items-start gap-4 rounded-[1.35rem] border border-slate-700/70 bg-slate-950/55 px-4 py-4 text-left shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]">
            <span className="mt-0.5 grid h-5 w-5 flex-none place-items-center rounded-md border border-slate-500/80 bg-slate-900/90 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)]">
              <input
                checked={showEnabledOnlyInSideNav}
                className="h-4 w-4 rounded border-transparent bg-transparent text-sky-400 focus:ring-sky-400"
                onChange={(event) =>
                  onShowEnabledOnlyInSideNavChange(event.target.checked)
                }
                type="checkbox"
              />
            </span>
            <span className="min-w-0 flex-1">
              <span className="block text-sm font-semibold text-slate-100">
                {t("agents.orderModal.sideNavEnabledOnly")}
              </span>
              <span className="mt-1 block text-xs leading-5 text-slate-500">
                {t("agents.orderModal.sideNavEnabledOnlyHint")}
              </span>
            </span>
          </label>
        </div>

        <div
          className="skill-markdown-scroll min-h-0 flex-1 space-y-6 overflow-y-auto px-6 py-5"
          ref={scrollRef}
        >
          {sections.map((section) => (
            <section
              className="rounded-[1.45rem] border border-slate-800/90 bg-slate-950/45 p-4 shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]"
              key={section.key}
            >
              <div className="mb-3 flex items-center justify-between gap-3">
                <div>
                  <div className="text-[11px] font-semibold uppercase tracking-[0.22em] text-slate-500">
                    {section.label}
                  </div>
                  <div className="mt-1 text-xs text-slate-600">
                    {section.helper}
                  </div>
                </div>
                <div className="rounded-full border border-slate-700/80 bg-slate-900/80 px-3 py-1 text-[11px] font-medium text-slate-500">
                  {t("agents.orderModal.dragHint")}
                </div>
              </div>

              <div className="space-y-2.5">
                {section.items.map((agent) => {
                  const isDropTarget =
                    dropTargetKey === agent.key && draggedKey !== agent.key;
                  const isDragged = draggedKey === agent.key;

                  return (
                    <div
                      className={`flex items-center gap-4 rounded-[1.2rem] border px-4 py-3.5 text-sm transition ${
                        isDropTarget
                          ? "border-sky-500/60 bg-sky-950/30 text-slate-100 shadow-[inset_0_0_0_1px_rgba(125,211,252,0.18)]"
                          : isDragged
                            ? "border-sky-400/30 bg-slate-950/90 text-slate-100 opacity-75"
                            : "border-slate-800/90 bg-slate-950/70 text-slate-200 hover:border-slate-700 hover:bg-slate-950/90"
                      }`}
                      data-agent-order-key={agent.key}
                      key={agent.key}
                    >
                      <button
                        aria-label={`${t("agents.orderModal.dragHint")}: ${agent.displayName}`}
                        className="touch-none rounded-xl border border-transparent p-2 text-slate-500 transition hover:border-slate-700 hover:bg-slate-900 hover:text-sky-300 active:cursor-grabbing"
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
                      <div className="grid h-11 w-11 flex-none place-items-center rounded-full border border-slate-700 bg-slate-900 shadow-[inset_0_1px_0_rgba(255,255,255,0.05)]">
                        <AgentBrandIcon agentKey={agent.key} className="size-[62%]" />
                      </div>
                      <div className="min-w-0 flex-1">
                        <div className="truncate text-base font-semibold tracking-tight text-slate-50">
                          {agent.displayName}
                        </div>
                        <div className="mt-1 text-xs font-medium text-slate-500">
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

        <div className="flex items-end justify-between gap-4 border-t border-slate-800/90 bg-slate-950/70 px-6 py-4">
          <div className="min-w-0 flex-1">
            {isDirty ? (
              <div className="flex items-center gap-2 text-xs text-sky-200">
                <span className="h-1.5 w-1.5 rounded-full bg-sky-300" />
                <span>{t("agents.orderModal.dirtyHint")}</span>
              </div>
            ) : (
              <div className="text-xs text-slate-500">
                {t("agents.orderModal.cleanHint")}
              </div>
            )}
          </div>
          <div className="flex justify-end gap-3">
            <button
              className="rounded-2xl border border-slate-700/90 bg-slate-900/60 px-4 py-2.5 text-sm font-medium text-slate-200 transition hover:border-slate-500 hover:text-slate-50"
              onClick={onClose}
              type="button"
            >
              {t("agents.orderModal.cancel")}
            </button>
            <button
              className="rounded-2xl bg-sky-400 px-5 py-2.5 text-sm font-semibold text-slate-950 shadow-[0_12px_32px_rgba(56,189,248,0.22)] transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:opacity-60"
              disabled={isSaving || !isDirty}
              onClick={() => void onSave(draftOrder)}
              type="button"
            >
              {isSaving ? t("agents.orderModal.saving") : t("agents.orderModal.save")}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
