import { useCallback, useEffect, useRef } from "react";

interface ScrollPosition {
  left: number;
  top: number;
}

const STORAGE_PREFIX = "skills-manager-system.scroll:";
const MAX_RESTORE_FRAMES = 12;

function readScrollPosition(storageKey: string) {
  try {
    const rawValue = window.localStorage.getItem(storageKey);
    if (!rawValue) {
      return null;
    }

    const parsedValue = JSON.parse(rawValue) as Partial<ScrollPosition>;
    if (
      typeof parsedValue.top !== "number" ||
      typeof parsedValue.left !== "number"
    ) {
      return null;
    }

    return parsedValue as ScrollPosition;
  } catch {
    return null;
  }
}

function writeScrollPosition(storageKey: string, position: ScrollPosition) {
  try {
    window.localStorage.setItem(storageKey, JSON.stringify(position));
  } catch {
    // Best-effort persistence only.
  }
}

export function useRememberedScrollPosition(id: string) {
  const cleanupRef = useRef<(() => void) | null>(null);
  const storageKey = `${STORAGE_PREFIX}${id}`;

  const scrollRef = useCallback((node: HTMLElement | null) => {
    cleanupRef.current?.();
    cleanupRef.current = null;

    if (!node) {
      return;
    }

    const savedPosition = readScrollPosition(storageKey);
    let frameId: number | null = null;

    const persistPosition = () => {
      writeScrollPosition(storageKey, {
        left: node.scrollLeft,
        top: node.scrollTop,
      });
    };

    const restorePosition = (attempt = 0) => {
      if (!savedPosition) {
        return;
      }

      node.scrollLeft = savedPosition.left;
      node.scrollTop = savedPosition.top;

      const topSettled = Math.abs(node.scrollTop - savedPosition.top) < 2;
      const leftSettled = Math.abs(node.scrollLeft - savedPosition.left) < 2;
      if (topSettled && leftSettled) {
        return;
      }

      if (attempt >= MAX_RESTORE_FRAMES) {
        return;
      }

      frameId = window.requestAnimationFrame(() => restorePosition(attempt + 1));
    };

    node.addEventListener("scroll", persistPosition, { passive: true });
    frameId = window.requestAnimationFrame(() => restorePosition());
    cleanupRef.current = () => {
      if (frameId !== null) {
        window.cancelAnimationFrame(frameId);
      }
      node.removeEventListener("scroll", persistPosition);
      persistPosition();
    };
  }, [storageKey]);

  useEffect(
    () => () => {
      cleanupRef.current?.();
    },
    [],
  );

  return scrollRef;
}
