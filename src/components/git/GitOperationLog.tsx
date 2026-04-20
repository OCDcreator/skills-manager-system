import { useTranslation } from "react-i18next";
import { CheckCircle, XCircle } from "lucide-react";

export interface OperationLogEntry {
  timestamp: Date;
  label: string;
  success: boolean;
  message: string;
}

interface GitOperationLogProps {
  entries: OperationLogEntry[];
}

export function GitOperationLog({ entries }: GitOperationLogProps) {
  const { t } = useTranslation();

  if (entries.length === 0) return null;

  return (
    <div className="rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <h3 className="mb-3 text-sm font-medium text-slate-300">
        {t("git.opLog.title")}
      </h3>
      <div className="max-h-48 space-y-1.5 overflow-y-auto">
        {entries.map((entry, i) => (
          <div
            key={i}
            className="flex items-start gap-2 rounded-lg bg-slate-800/50 px-3 py-2 text-xs"
          >
            {entry.success ? (
              <CheckCircle className="mt-0.5 h-3.5 w-3.5 shrink-0 text-emerald-400" />
            ) : (
              <XCircle className="mt-0.5 h-3.5 w-3.5 shrink-0 text-rose-400" />
            )}
            <div className="min-w-0 flex-1">
              <div className="flex items-center gap-2">
                <span className="font-medium text-slate-200">
                  {entry.label}
                </span>
                <span className="text-slate-500">
                  {entry.timestamp.toLocaleTimeString()}
                </span>
              </div>
              <p className="mt-0.5 truncate text-slate-400">{entry.message}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
