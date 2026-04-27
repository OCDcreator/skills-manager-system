import { useCallback, useMemo, useState } from "react";
import {
  mergeAgentOrderWithInventory,
  resolveSortedAgentInventory,
} from "../lib/agent-order";
import * as api from "../lib/tauri";
import type { AgentInventoryItem } from "../lib/tauri";

interface AgentOrderStateOptions {
  agentInventory: AgentInventoryItem[];
  setErrorMessage: (message: string | null) => void;
  errorMessageFrom: (error: unknown) => string;
}

export function useAgentOrderState({
  agentInventory,
  setErrorMessage,
  errorMessageFrom,
}: AgentOrderStateOptions) {
  const [agentOrder, setAgentOrder] = useState<api.AgentKey[]>([]);

  const refreshAgentOrder = useCallback(async () => {
    try {
      const nextOrder = await api.getAgentOrder();
      setAgentOrder(nextOrder);
      setErrorMessage(null);
    } catch (error) {
      setErrorMessage(errorMessageFrom(error));
    }
  }, [errorMessageFrom, setErrorMessage]);

  const saveAgentOrder = useCallback(async (nextOrder: api.AgentKey[]) => {
    try {
      const savedOrder = await api.setAgentOrder(nextOrder);
      setAgentOrder(savedOrder);
      setErrorMessage(null);
    } catch (error) {
      const message = errorMessageFrom(error);
      setErrorMessage(message);
      throw error instanceof Error ? error : new Error(message);
    }
  }, [errorMessageFrom, setErrorMessage]);

  const sortedAgentInventory = useMemo(
    () =>
      resolveSortedAgentInventory(
        agentInventory,
        mergeAgentOrderWithInventory(agentInventory, agentOrder),
      ),
    [agentInventory, agentOrder],
  );

  return {
    agentOrder,
    sortedAgentInventory,
    refreshAgentOrder,
    saveAgentOrder,
  };
}
