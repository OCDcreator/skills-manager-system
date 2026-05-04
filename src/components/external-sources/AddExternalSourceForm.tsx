import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { AddExternalSourceInput } from "../../lib/tauri";

interface AddExternalSourceFormProps {
  isSubmitting: boolean;
  onSubmit: (input: AddExternalSourceInput) => Promise<void>;
}

export function AddExternalSourceForm({ isSubmitting, onSubmit }: AddExternalSourceFormProps) {
  const { t } = useTranslation();
  const [repoUrl, setRepoUrl] = useState("");
  const [branch, setBranch] = useState("");
  const [subpath, setSubpath] = useState("");
  const [submitFailed, setSubmitFailed] = useState(false);

  return (
    <form
      className="rounded-2xl border border-slate-800 bg-slate-900 p-6"
      onSubmit={async (event) => {
        event.preventDefault();
        const trimmedRepoUrl = repoUrl.trim();
        if (!trimmedRepoUrl) {
          return;
        }
        try {
          await onSubmit({
            repoUrl: trimmedRepoUrl,
            branch: branch.trim() || null,
            subpath: subpath.trim() || null,
          });
          setRepoUrl("");
          setBranch("");
          setSubpath("");
          setSubmitFailed(false);
        } catch {
          setSubmitFailed(true);
        }
      }}
    >
      <div className="grid gap-4 lg:grid-cols-[minmax(0,1.35fr)_minmax(10rem,0.45fr)_minmax(12rem,0.65fr)_max-content] lg:items-end">
        <label className="min-w-0 space-y-2">
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
        <label className="min-w-0 space-y-2">
          <span className="text-sm font-medium text-slate-200">
            {t("sources.form.branchLabel")}
          </span>
          <input
            className="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none transition focus:border-sky-400"
            onChange={(event) => {
              setBranch(event.target.value);
              if (submitFailed) {
                setSubmitFailed(false);
              }
            }}
            placeholder={t("sources.form.branchPlaceholder")}
            value={branch}
          />
        </label>
        <label className="min-w-0 space-y-2">
          <span className="text-sm font-medium text-slate-200">
            {t("sources.form.subpathLabel")}
          </span>
          <input
            className="w-full rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none transition focus:border-sky-400"
            onChange={(event) => {
              setSubpath(event.target.value);
              if (submitFailed) {
                setSubmitFailed(false);
              }
            }}
            placeholder={t("sources.form.subpathPlaceholder")}
            value={subpath}
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
