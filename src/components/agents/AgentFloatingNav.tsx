import { ArrowDownUp, ChevronDown, ChevronUp } from "lucide-react";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import type { AgentInventoryItem } from "../../lib/tauri";
import { AgentBrandIcon } from "./AgentBrandIcon";

interface AgentFloatingNavProps {
  agents: Pick<AgentInventoryItem, "key" | "displayName">[];
  onOpenOrderModal: () => void;
}

interface AgentFloatingNavItem {
  href?: string;
  label: string;
  kind: "jump" | "agent" | "action";
  actionKey?: "open-order-modal";
  agentKey?: string;
  direction?: "up" | "down";
}

function targetHref(agentKey: string) {
  return `#agent-sync-target-${agentKey}`;
}

function bubbleClassName(agentKey?: string) {
  switch (agentKey) {
    case "codex":
      return "border-emerald-200/45 bg-emerald-100 text-emerald-950 shadow-emerald-950/30";
    case "claude_code":
      return "border-orange-200/45 bg-orange-100 text-orange-950 shadow-orange-950/30";
    case "opencode":
      return "border-slate-200/40 bg-slate-950 text-white shadow-slate-950/45";
    case "cursor":
      return "border-slate-200/50 bg-slate-50 text-slate-950 shadow-slate-950/35";
    case "amp":
      return "border-fuchsia-200/45 bg-fuchsia-500 text-white shadow-fuchsia-950/35";
    case "kilo_code":
      return "border-slate-200/40 bg-slate-950 text-white shadow-slate-950/45";
    case "kimi":
      return "border-orange-200/45 bg-orange-100 text-orange-950 shadow-orange-950/30";
    case "roo_code":
      return "border-emerald-200/45 bg-emerald-500 text-white shadow-emerald-950/35";
    case "goose":
      return "border-lime-200/45 bg-lime-300 text-slate-950 shadow-lime-950/30";
    case "gemini_cli":
      return "border-violet-200/45 bg-violet-500 text-white shadow-violet-950/35";
    case "github_copilot":
      return "border-sky-200/45 bg-sky-400 text-slate-950 shadow-sky-950/35";
    case "windsurf":
      return "border-blue-200/45 bg-blue-500 text-white shadow-blue-950/35";
    default:
      return "border-slate-500/60 bg-slate-900/95 text-slate-300 shadow-slate-950/40";
  }
}

function motionForIndex(index: number, activeIndex: number | null) {
  if (activeIndex === null) {
    return { scale: 1, opacity: 0.9, pull: 0 };
  }

  const distance = Math.abs(activeIndex - index);

  if (distance === 0) {
    return { scale: 1.38, opacity: 1, pull: -14 };
  }
  if (distance === 1) {
    return { scale: 1.2, opacity: 0.96, pull: -8 };
  }
  if (distance === 2) {
    return { scale: 1.08, opacity: 0.9, pull: -4 };
  }
  if (distance === 3) {
    return { scale: 0.98, opacity: 0.78, pull: -1 };
  }

  return { scale: 0.88, opacity: 0.56, pull: 0 };
}

export function AgentFloatingNav({ agents, onOpenOrderModal }: AgentFloatingNavProps) {
  const { t } = useTranslation();
  const [activeIndex, setActiveIndex] = useState<number | null>(null);
  const navItems = useMemo<AgentFloatingNavItem[]>(
    () => [
      {
        label: t("agents.sideNav.order"),
        kind: "action",
        actionKey: "open-order-modal",
      },
      {
        href: "#agent-sync-overview",
        label: t("agents.sideNav.jumpTop"),
        kind: "jump",
        direction: "up",
      },
      ...agents.map((agent) => ({
        href: targetHref(agent.key),
        label: agent.displayName,
        kind: "agent" as const,
        agentKey: agent.key,
      })),
      {
        href: "#agent-sync-results",
        label: t("agents.sideNav.jumpBottom"),
        kind: "jump",
        direction: "down",
      },
    ],
    [agents, t],
  );

  return (
    <nav
      aria-label={t("agents.sideNav.label")}
      className="pointer-events-none fixed right-3 top-1/2 z-50 w-96 max-w-[calc(100vw-1.5rem)] -translate-y-1/2 bg-transparent"
    >
      {/* The rail is transparent; only the nodes and labels visibly float above the content. */}
      <div className="relative">
        <div
          aria-hidden="true"
          className="absolute bottom-7 right-7 top-7 w-px bg-gradient-to-b from-slate-800/0 via-slate-600/35 to-slate-800/0"
        />
        <ul
          className="agent-floating-nav-scroll relative max-h-[calc(100vh-2rem)] overflow-y-auto py-1 pr-1"
          onMouseLeave={() => setActiveIndex(null)}
        >
          {navItems.map((item, index) => {
            const motion = motionForIndex(index, activeIndex);
            const isActive = activeIndex === index;
            const labelBubble = (
              <span
                className={`pointer-events-none absolute right-16 top-1/2 -translate-y-1/2 overflow-hidden whitespace-nowrap rounded-full border border-slate-700/60 bg-slate-950/88 py-2 text-sm font-semibold text-slate-100 shadow-2xl shadow-slate-950/45 backdrop-blur-md transition-all duration-300 ease-out ${
                  isActive
                    ? "max-w-64 translate-x-0 px-4 opacity-100"
                    : "max-w-0 translate-x-2 px-0 opacity-0"
                }`}
              >
                {item.label}
              </span>
            );

            const bubble = (
              <span
                className={`absolute right-2 top-1/2 grid size-10 origin-center place-items-center rounded-full border shadow-xl transition-all duration-300 ease-out ${
                  item.kind === "agent"
                    ? bubbleClassName(item.agentKey)
                    : "border-slate-500/60 bg-slate-900/95 text-slate-200 shadow-slate-950/40"
                }`}
                style={{
                  transform: `translateY(-50%) scale(${motion.scale})`,
                }}
              >
                {item.kind === "agent" && item.agentKey ? (
                  <AgentBrandIcon agentKey={item.agentKey} className="size-[62%]" />
                ) : item.kind === "action" ? (
                  <ArrowDownUp className="size-[58%]" strokeWidth={2.2} />
                ) : item.direction === "up" ? (
                  <ChevronUp className="size-[58%]" strokeWidth={2.2} />
                ) : (
                  <ChevronDown className="size-[58%]" strokeWidth={2.2} />
                )}
              </span>
            );

            return (
              <li
                className="pointer-events-none relative z-10 h-14"
                key={item.kind === "action" ? item.actionKey : item.href}
              >
                {item.kind === "action" ? (
                  <button
                    className="pointer-events-auto relative ml-auto block h-14 w-14 text-right transition-all duration-300 ease-out focus-visible:outline-none"
                    onBlur={() => setActiveIndex(null)}
                    onClick={onOpenOrderModal}
                    onFocus={() => setActiveIndex(index)}
                    onMouseEnter={() => setActiveIndex(index)}
                    style={{
                      opacity: motion.opacity,
                      transform: `translateX(${motion.pull}px)`,
                    }}
                    title={item.label}
                    type="button"
                  >
                    {labelBubble}
                    {bubble}
                  </button>
                ) : (
                  <a
                    className="pointer-events-auto relative ml-auto block h-14 w-14 text-right transition-all duration-300 ease-out focus-visible:outline-none"
                    href={item.href}
                    onBlur={() => setActiveIndex(null)}
                    onFocus={() => setActiveIndex(index)}
                    onMouseEnter={() => setActiveIndex(index)}
                    style={{
                      opacity: motion.opacity,
                      transform: `translateX(${motion.pull}px)`,
                    }}
                    title={item.label}
                  >
                    {labelBubble}
                    {bubble}
                  </a>
                )}
              </li>
            );
          })}
        </ul>
      </div>
    </nav>
  );
}
