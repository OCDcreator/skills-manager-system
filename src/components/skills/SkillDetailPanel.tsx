import hljs from "highlight.js/lib/common";
import { Marked, type Tokens } from "marked";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../../context/AppContext";
import { shortCommit } from "../../lib/external-sources";
import { useRememberedScrollPosition } from "../../lib/scroll-memory";
import type { ExternalSourceWarning, SkillDocument, SkillSummary } from "../../lib/tauri";

interface SkillDetailPanelProps {
  skill: SkillSummary | null;
  document: SkillDocument | null;
  isEnabled: boolean;
}

const markdownRenderer = new Marked({
  async: false,
  breaks: true,
  gfm: true,
  renderer: {
    code({ lang, text }: Tokens.Code) {
      const rawLanguage = lang ?? "";
      const normalizedLanguage = rawLanguage.match(/^[\w-]+/)?.[0];
      const isFrontmatter = /\bfrontmatter\b/.test(rawLanguage);
      const highlighted = normalizedLanguage && hljs.getLanguage(normalizedLanguage)
        ? hljs.highlight(text, {
            ignoreIllegals: true,
            language: normalizedLanguage,
          }).value
        : hljs.highlightAuto(text).value;
      const className = normalizedLanguage
        ? `hljs language-${normalizedLanguage}`
        : "hljs";
      const preClassName = isFrontmatter ? "skill-frontmatter-pre" : "";

      return `<pre class="${preClassName}"><code class="${className}">${highlighted}</code></pre>`;
    },
  },
});

function convertFrontmatterToYamlFence(content: string) {
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---(?=\r?\n|$)/);
  if (!match) {
    return content;
  }

  const body = content.slice(match[0].length).replace(/^\r?\n/, "");
  const yamlBlock = `\`\`\`yaml frontmatter\n${match[1]}\n\`\`\``;
  return body ? `${yamlBlock}\n\n${body}` : yamlBlock;
}

function warningMessageFrom(
  warning: ExternalSourceWarning,
  skill: SkillSummary,
  variantPath: string,
  t: ReturnType<typeof useTranslation>["t"],
) {
  if (warning.code === "variant_disappeared") {
    return t("skills.warnings.variantDisappeared", {
      agent: skill.managedSource?.agentKey ?? "unknown",
      variant: variantPath,
    });
  }

  if (warning.code === "integrity_mismatch") {
    return t("skills.detail.managedSource.integrityMismatch");
  }

  return warning.message;
}

export function SkillDetailPanel({ document, isEnabled, skill }: SkillDetailPanelProps) {
  const { t } = useTranslation();
  const { externalSources } = useAppContext();
  const [wrapFrontmatter, setWrapFrontmatter] = useState(true);
  const scrollRef = useRememberedScrollPosition(`skills:detail:${skill?.id ?? "empty"}`);
  const basePanelClassName =
    "min-w-0 rounded-2xl border border-slate-800 bg-slate-900 p-6";
  const dockedPanelClassName =
    "min-[1280px]:sticky min-[1280px]:top-8 min-[1280px]:max-h-[calc(100vh-7rem)]";
  const renderedDocument = useMemo(() => {
    if (!document) {
      return null;
    }

    const previewContent = convertFrontmatterToYamlFence(document.content);
    return markdownRenderer.parse(previewContent);
  }, [document]);
  const managedImport = useMemo(() => {
    if (!skill?.managedSource) {
      return null;
    }

    return externalSources
      .flatMap((source) => source.imports)
      .find((item) => item.importId === skill.managedSource?.importId) ?? null;
  }, [externalSources, skill]);
  const managedWarnings = useMemo(() => {
    if (!skill?.managedSource) {
      return [];
    }

    const warnings = managedImport?.warnings.map((warning) =>
      warningMessageFrom(
        warning,
        skill,
        managedImport?.upstreamVariantPath ?? skill.relativePath,
        t,
      ),
    ) ?? [];
    if (skill.managedSource.integrity === "mismatch") {
      warnings.unshift(t("skills.detail.managedSource.integrityMismatch"));
    }
    return [...new Set(warnings)];
  }, [managedImport, skill, t]);

  if (!skill) {
    return (
      <aside className={`${basePanelClassName} ${dockedPanelClassName} text-sm text-slate-400`}>
        {t("skills.selectPrompt")}
      </aside>
    );
  }

  return (
    <aside className={`${basePanelClassName} ${dockedPanelClassName} flex flex-col gap-4 min-[1280px]:overflow-hidden`}>
      <div className="space-y-2">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <h3 className="truncate text-xl font-semibold text-slate-100">{skill.name}</h3>
            <p className="mt-2 text-sm text-slate-400">
              {skill.description || t("skills.noDescription")}
            </p>
          </div>
          <button
            aria-pressed={wrapFrontmatter}
            className={`shrink-0 rounded-lg border px-3 py-2 text-xs font-medium transition ${
              wrapFrontmatter
                ? "border-sky-400/60 bg-sky-400/15 text-sky-100 shadow-[0_0_20px_rgba(56,189,248,0.12)]"
                : "border-slate-700 bg-slate-950 text-slate-300 hover:border-slate-500"
            }`}
            onClick={() => setWrapFrontmatter((current) => !current)}
            title={t("tooltip.skills.frontmatterWrap")}
            type="button"
          >
            {wrapFrontmatter
              ? t("skills.detail.frontmatterWrapOn")
              : t("skills.detail.frontmatterWrapOff")}
          </button>
        </div>
        <dl className="grid grid-cols-[96px_1fr] gap-2 text-sm text-slate-300">
          <dt className="text-slate-500">{t("skills.detail.sourceLabel")}</dt>
          <dd>
            {skill.sourceType === "custom"
              ? t("skills.source.custom")
              : t("skills.source.external")}
          </dd>
          <dt className="text-slate-500">{t("skills.detail.statusLabel")}</dt>
          <dd>{isEnabled ? t("skills.status.enabled") : t("skills.status.disabled")}</dd>
          <dt className="text-slate-500">{t("skills.detail.pathLabel")}</dt>
          <dd className="break-all">{skill.relativePath}</dd>
        </dl>
        {skill.managedSource ? (
          <section className="rounded-xl border border-slate-800 bg-slate-950/70 p-4">
            <div className="flex flex-wrap items-center gap-2">
              <h4 className="text-sm font-semibold text-slate-100">
                {t("skills.detail.managedSource.title")}
              </h4>
              <span className="rounded-full bg-sky-500/15 px-2 py-1 text-xs text-sky-100">
                {t("skills.badges.managedGithubMirror")}
              </span>
              {skill.managedSource.updateAvailable ? (
                <span className="rounded-full bg-amber-500/15 px-2 py-1 text-xs text-amber-200">
                  {t("skills.detail.managedSource.updateAvailable")}
                </span>
              ) : null}
            </div>
            <dl className="mt-3 grid grid-cols-[120px_1fr] gap-2 text-sm text-slate-300">
              <dt className="text-slate-500">{t("skills.detail.managedSource.repoUrl")}</dt>
              <dd className="break-all">{skill.managedSource.repoUrl}</dd>
              <dt className="text-slate-500">{t("skills.detail.managedSource.pinnedCommit")}</dt>
              <dd>{shortCommit(skill.managedSource.pinnedCommit, skill.managedSource.pinnedCommit)}</dd>
              <dt className="text-slate-500">{t("skills.detail.managedSource.agent")}</dt>
              <dd>{skill.managedSource.agentKey}</dd>
              {managedImport ? (
                <>
                  <dt className="text-slate-500">{t("skills.detail.managedSource.variantPath")}</dt>
                  <dd className="break-all">{managedImport.upstreamVariantPath}</dd>
                </>
              ) : null}
            </dl>
            {managedWarnings.length ? (
              <div className="mt-3 rounded-xl border border-amber-700/50 bg-amber-950/30 px-3 py-2 text-xs text-amber-100">
                {managedWarnings.map((warning) => (
                  <p key={warning}>{warning}</p>
                ))}
              </div>
            ) : null}
          </section>
        ) : skill.sourceType === "external" ? (
          <div className="flex flex-wrap gap-2">
            <span className="rounded-full bg-amber-500/15 px-2 py-1 text-xs text-amber-200">
              {t("skills.badges.manualExternal")}
            </span>
          </div>
        ) : null}
      </div>

      <div
        className="skill-markdown-scroll min-h-0 min-w-0 flex-1 overflow-x-hidden overflow-y-auto rounded-xl border border-slate-800 bg-slate-950 p-4"
        ref={scrollRef}
      >
        {renderedDocument ? (
          <article
            className={`markdown-body skill-markdown-body min-w-0 break-words rounded-lg ${
              wrapFrontmatter ? "skill-frontmatter-wrap" : "skill-frontmatter-nowrap"
            }`}
            dangerouslySetInnerHTML={{ __html: renderedDocument }}
          />
        ) : (
          <p className="text-sm text-slate-400">{t("skills.detail.loadingDocument")}</p>
        )}
      </div>
    </aside>
  );
}
