import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { PendingNavigation } from "../context/app-context-types";

interface UnsavedChangesDialogProps {
  pendingNavigation: PendingNavigation;
  onCancel: () => void;
  onDiscard: () => void;
  onSave: () => Promise<void>;
}

export function UnsavedChangesDialog({
  pendingNavigation,
  onCancel,
  onDiscard,
  onSave,
}: UnsavedChangesDialogProps) {
  const { t } = useTranslation();
  const [isSaving, setIsSaving] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const handleSave = async () => {
    setIsSaving(true);
    setErrorMessage(null);
    try {
      await onSave();
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 px-6">
      <section className="w-full max-w-lg rounded-2xl border border-slate-700 bg-slate-900 p-6 shadow-2xl">
        <h2 className="text-lg font-semibold text-slate-100">
          {t("unsaved.title")}
        </h2>
        <p className="mt-2 text-sm text-slate-400">
          {t("unsaved.description", {
            target: t(`nav.${pendingNavigation.targetView}`),
          })}
        </p>

        {errorMessage ? (
          <div className="mt-4 rounded-lg border border-rose-900/60 bg-rose-950/50 px-3 py-2 text-sm text-rose-200">
            {errorMessage}
          </div>
        ) : null}

        <div className="mt-6 flex flex-wrap justify-end gap-2">
          <button
            className="rounded-lg border border-slate-700 px-4 py-2 text-sm text-slate-200 hover:bg-slate-800"
            disabled={isSaving}
            onClick={onCancel}
            type="button"
          >
            {t("unsaved.stay")}
          </button>
          <button
            className="rounded-lg border border-amber-800 px-4 py-2 text-sm text-amber-200 hover:bg-amber-950/50"
            disabled={isSaving}
            onClick={onDiscard}
            type="button"
          >
            {t("unsaved.discard")}
          </button>
          <button
            className="rounded-lg bg-sky-500 px-4 py-2 text-sm font-semibold text-slate-950 disabled:opacity-60"
            disabled={isSaving}
            onClick={() => void handleSave()}
            type="button"
          >
            {isSaving ? t("unsaved.saving") : t("unsaved.save")}
          </button>
        </div>
      </section>
    </div>
  );
}
