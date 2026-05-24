import { useTranslation } from "react-i18next";
import type { AgentInventoryItem } from "../../lib/tauri";

interface AgentCompactTabsProps {
  activeAgentKey: string | null;
  agents: Pick<AgentInventoryItem, "key" | "displayName">[];
  onSelect: (agentKey: string) => void;
}

export function AgentCompactTabs({
  activeAgentKey,
  agents,
  onSelect,
}: AgentCompactTabsProps) {
  const { t } = useTranslation();

  return (
    <nav
      className="min-w-0 max-w-full overflow-hidden min-[1280px]:hidden"
      aria-label={t("agents.compactTabs.label")}
    >
      <div className="skill-markdown-scroll flex max-w-full gap-2 overflow-x-auto pb-1">
        {agents.map((agent) => (
          <button
            key={agent.key}
            className={`whitespace-nowrap rounded-full border px-3 py-2 text-sm ${
              activeAgentKey === agent.key
                ? "border-sky-400/70 bg-sky-400/10 text-sky-100"
                : "border-slate-700 bg-slate-900 text-slate-300"
            }`}
            onClick={() => onSelect(agent.key)}
            type="button"
          >
            {agent.displayName}
          </button>
        ))}
      </div>
    </nav>
  );
}
