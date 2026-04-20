import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../context/AppContext";

export function RepoPathForm() {
  const { t } = useTranslation();
  const { isSavingPath, repoPath, saveRepoPath } = useAppContext();
  const [draftPath, setDraftPath] = useState(repoPath ?? "");
  const [statusMessage, setStatusMessage] = useState("");

  useEffect(() => {
    setDraftPath(repoPath ?? "");
  }, [repoPath]);

  const handleBrowse = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: draftPath || undefined,
    });

    if (typeof selected === "string") {
      setDraftPath(selected);
    }
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setStatusMessage("");
    try {
      await saveRepoPath(draftPath);
      setStatusMessage(t("settings.saved"));
    } catch {
      setStatusMessage(t("settings.error"));
    }
  };

  return (
    <form className="space-y-4" onSubmit={handleSubmit}>
      <label className="block text-sm font-medium text-slate-200" htmlFor="repo-path">
        {t("settings.pathLabel")}
      </label>
      <div className="flex gap-3">
        <input
          id="repo-path"
          className="flex-1 rounded-xl border border-slate-700 bg-slate-900 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
          onChange={(event) => setDraftPath(event.target.value)}
          placeholder={t("settings.pathPlaceholder")}
          value={draftPath}
        />
        <button
          className="rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm text-slate-100"
          onClick={handleBrowse}
          type="button"
        >
          {t("settings.browse")}
        </button>
      </div>
      <button
        className="rounded-xl bg-sky-400 px-4 py-3 text-sm font-medium text-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
        disabled={isSavingPath}
        type="submit"
      >
        {t("settings.save")}
      </button>
      {statusMessage ? <p className="text-sm text-slate-400">{statusMessage}</p> : null}
    </form>
  );
}
