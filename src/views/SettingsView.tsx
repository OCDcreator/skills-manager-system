import { useTranslation } from "react-i18next";
import { RepoPathForm } from "../components/RepoPathForm";

export function SettingsView() {
  const { t } = useTranslation();

  return (
    <section className="space-y-6 rounded-2xl border border-slate-800 bg-slate-900 p-6">
      <div>
        <h2 className="text-xl font-semibold">{t("settings.title")}</h2>
        <p className="mt-2 text-sm text-slate-400">{t("settings.description")}</p>
      </div>
      <RepoPathForm />
    </section>
  );
}
