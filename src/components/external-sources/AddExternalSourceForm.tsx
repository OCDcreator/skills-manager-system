import { useState } from "react";
import { useTranslation } from "react-i18next";

interface AddExternalSourceFormProps {
  isSubmitting: boolean;
  onSubmit: (repoUrl: string) => Promise<void>;
}

export function AddExternalSourceForm({ isSubmitting, onSubmit }: AddExternalSourceFormProps) {
  const { t } = useTranslation();
  const [repoUrl, setRepoUrl] = useState("");
  const [submitFailed, setSubmitFailed] = useState(false);

  return (
    <form
      className="rounded-2xl border border-slate-800 bg-slate-900 p-6"
      onSubmit={async (event) => {
        event.preventDefault();
        const trimmed = repoUrl.trim();
        if (!trimmed) {
          return;
        }
        try {
          await onSubmit(trimmed);
          setRepoUrl("");
          setSubmitFailed(false);
        } catch {
          setSubmitFailed(true);
        }
      }}
    >
      <div className="flex flex-col gap-4 lg:flex-row lg:items-end">
        <label className="flex-1 space-y-2">
          <span className="text-sm font-medium text-slate-200">
            {t("sources.form.repoUrlLabel")}
          </span>
          <input
            className="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none transition focus:border-sky-400"
            onChange={(event) => {
              setRepoUrl(event.target.value);
              if (submitFailed) {
                setSubmitFailed(false);
              }
            }}
            placeholder={t("sources.form.repoUrlPlaceholder")}
            value={repoUrl}
          />
        </label>
        <button
          className="rounded-xl bg-sky-400 px-5 py-3 text-sm font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:bg-slate-700 disabled:text-slate-400"
          disabled={isSubmitting || !repoUrl.trim()}
          type="submit"
        >
          {isSubmitting ? t("sources.form.submitting") : t("sources.form.submit")}
        </button>
      </div>
      <p className="mt-3 text-sm text-slate-400">{t("sources.form.description")}</p>
      {submitFailed ? (
        <p className="mt-2 text-sm text-rose-300">{t("sources.form.submitFailed")}</p>
      ) : null}
    </form>
  );
}
