import { invoke as tauriInvoke } from "@tauri-apps/api/core";

type InvokeArgs = Record<string, unknown>;

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

function isTauriRuntime() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);
}

function isDevPreviewRuntime() {
  if (typeof window === "undefined") {
    return false;
  }
  return ["127.0.0.1", "localhost", "::1"].includes(window.location.hostname);
}

async function invokeDevBridge<T>(command: string, args?: InvokeArgs): Promise<T> {
  const response = await fetch("/__skills-manager-dev-invoke", {
    body: JSON.stringify({ command, args: args ?? {} }),
    headers: { "content-type": "application/json" },
    method: "POST",
  });

  const payload = await response.json().catch(() => null);
  if (!response.ok || payload?.ok === false) {
    const message =
      payload?.error?.message ??
      payload?.message ??
      `Dev invoke failed for ${command}`;
    throw new Error(message);
  }

  return payload.data as T;
}

export function invoke<T>(command: string, args?: InvokeArgs): Promise<T> {
  if (isTauriRuntime()) {
    return tauriInvoke<T>(command, args);
  }

  if (isDevPreviewRuntime()) {
    return invokeDevBridge<T>(command, args);
  }

  return Promise.reject(
    new Error("This page must run inside the Tauri app or Vite dev server."),
  );
}
