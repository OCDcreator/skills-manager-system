import { useEffect, useRef } from "react";
import { RotateCcw, Square } from "lucide-react";
import { useTranslation } from "react-i18next";
import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "@xterm/xterm";
import {
  drainTerminalOutput,
  resizeTerminalSession,
  stopTerminalSession,
  writeTerminalInput,
  type TerminalLaunchInput,
  type TerminalSessionSnapshot,
} from "../../lib/terminal";

interface AssistantTerminalSessionProps {
  launchInput: TerminalLaunchInput;
  session: TerminalSessionSnapshot;
  onRestart: () => void;
  onSessionChange: (session: TerminalSessionSnapshot | null) => void;
}

function sameSessionSnapshot(
  left: TerminalSessionSnapshot | null,
  right: TerminalSessionSnapshot | null,
) {
  if (left === right) {
    return true;
  }
  if (!left || !right) {
    return false;
  }

  return (
    left.cliKey === right.cliKey &&
    left.workingDirectory === right.workingDirectory &&
    left.cols === right.cols &&
    left.rows === right.rows &&
    left.status === right.status &&
    left.message === right.message
  );
}

export function AssistantTerminalSession({
  launchInput,
  onRestart,
  onSessionChange,
  session,
}: AssistantTerminalSessionProps) {
  const { t } = useTranslation();
  const containerRef = useRef<HTMLDivElement | null>(null);
  const latestSessionRef = useRef<TerminalSessionSnapshot | null>(session);

  useEffect(() => {
    latestSessionRef.current = session;
  }, [session]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) {
      return;
    }

    const terminal = new Terminal({
      cursorBlink: true,
      fontSize: 13,
      theme: { background: "#020617", foreground: "#e2e8f0" },
    });
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(container);
    fitAddon.fit();

    const dataDisposable = terminal.onData((data: string) => {
      void writeTerminalInput(data);
    });

    let disposed = false;
    const pollId = window.setInterval(async () => {
      const drained = await drainTerminalOutput();
      if (disposed) {
        return;
      }
      if (drained.output) {
        terminal.write(drained.output);
      }
      if (!sameSessionSnapshot(latestSessionRef.current, drained.session)) {
        latestSessionRef.current = drained.session;
        onSessionChange(drained.session);
      }
    }, 80);

    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
      void resizeTerminalSession(terminal.cols, terminal.rows);
    });
    resizeObserver.observe(container);

    return () => {
      disposed = true;
      window.clearInterval(pollId);
      resizeObserver.disconnect();
      dataDisposable.dispose();
      terminal.dispose();
    };
  }, [launchInput, onSessionChange]);

  async function handleStop() {
    const stopped = await stopTerminalSession();
    onSessionChange(stopped);
  }

  return (
    <div className="grid h-full grid-rows-[auto_1fr] gap-4 p-5">
      <div className="flex items-center justify-between gap-3">
        <div>
          <h3 className="text-lg font-semibold text-slate-100">
            {t("assistant.activeSessionTitle")}
          </h3>
          <p className="text-sm text-slate-400">{session.workingDirectory}</p>
        </div>
        <div className="flex gap-2">
          <button
            className="rounded-full border border-slate-700 px-4 py-2 text-sm text-slate-100"
            onClick={() => void handleStop()}
            type="button"
          >
            <Square className="mr-2 inline h-4 w-4" />
            {t("assistant.stop")}
          </button>
          <button
            className="rounded-full bg-sky-400 px-4 py-2 text-sm font-medium text-slate-950"
            onClick={onRestart}
            type="button"
          >
            <RotateCcw className="mr-2 inline h-4 w-4" />
            {t("assistant.restart")}
          </button>
        </div>
      </div>

      {session.message ? (
        <p className="text-sm text-slate-400">{session.message}</p>
      ) : null}

      <div className="min-h-0 overflow-hidden rounded-[1.5rem] border border-slate-800 bg-slate-950/95 p-3">
        <div className="h-full w-full" ref={containerRef} />
      </div>
    </div>
  );
}
