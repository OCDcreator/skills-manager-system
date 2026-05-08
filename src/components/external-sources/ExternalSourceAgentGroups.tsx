import type { Dispatch, SetStateAction } from "react";
import { useMemo } from "react";
import { CircleHelp } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  agentLabel,
  equivalentContentGroupCount,
  EXTERNAL_IMPORT_TARGETS,
  groupExternalVariantsByAgent,
  variantsWithSameContent,
} from "../../lib/external-sources";
import type {
  AgentKey,
  ExternalSourceSnapshotItem,
  ExternalVariantKey,
  ExternalVariantSnapshot,
} from "../../lib/tauri";
import { AgentBrandIcon } from "../agents/AgentBrandIcon";
import { ExternalImportList } from "./ExternalImportList";

interface ExternalSourceAgentGroupsProps {
  imports: ExternalSourceSnapshotItem["imports"];
  isBusy: boolean;
  recordId: string;
  repoPath: string | null;
  selectedTargets: Record<string, AgentKey | "">;
  setSelectedTargets: Dispatch<SetStateAction<Record<string, AgentKey | "">>>;
  updatingExternalImportId: string | null;
  variants: ExternalSourceSnapshotItem["variants"];
  onImportVariant: (
    sourceId: string,
    agentKey: ExternalVariantKey,
    variantPath: string,
  ) => Promise<void>;
  onRepairImport: (importId: string) => Promise<void>;
  onUpdateImport: (importId: string) => Promise<void>;
}

function variantPreviewList(
  variant: ExternalVariantSnapshot,
  variantKey: string,
  kind: "directories" | "files",
  t: ReturnType<typeof useTranslation>["t"],
) {
  const items = kind === "directories" ? variant.childDirectories : variant.childFiles;
  const emptyLabel =
    kind === "files" && !variant.childFiles.length && !variant.childDirectories.length
      ? t("sources.variants.rootOnly")
      : t("sources.variants.none");

  return (
    <div className="flex flex-wrap items-center gap-2">
      <span className="font-medium text-slate-400">
        {kind === "directories"
          ? t("sources.variants.childDirectories")
          : t("sources.variants.childFiles")}
      </span>
      {items.length ? (
        <>
          {items.slice(0, 5).map((item) => (
            <span
              className="rounded-full border border-slate-800 bg-slate-950/70 px-2 py-1 text-[11px] text-slate-300"
              key={`${variantKey}:${kind}:${item}`}
            >
              {item}
            </span>
          ))}
          {items.length > 5 ? (
            <span className="text-[11px] text-slate-500">+{items.length - 5}</span>
          ) : null}
        </>
      ) : (
        <span className="text-[11px] text-slate-500">{emptyLabel}</span>
      )}
    </div>
  );
}

export function ExternalSourceAgentGroups({
  imports,
  isBusy,
  recordId,
  repoPath,
  selectedTargets,
  setSelectedTargets,
  updatingExternalImportId,
  variants,
  onImportVariant,
  onRepairImport,
  onUpdateImport,
}: ExternalSourceAgentGroupsProps) {
  const { t } = useTranslation();
  const groups = useMemo(
    () => groupExternalVariantsByAgent(variants, imports),
    [imports, variants],
  );
  const equivalentGroups = useMemo(
    () => equivalentContentGroupCount(variants),
    [variants],
  );

  if (!groups.length) {
    return <p className="text-sm text-slate-400">{t("sources.variants.noneDetected")}</p>;
  }

  return (
    <section className="space-y-4 rounded-xl border border-slate-800 bg-slate-950/60 p-4">
      <div className="flex flex-col gap-2 lg:flex-row lg:items-start lg:justify-between">
        <div>
          <h4 className="text-sm font-semibold text-slate-100">
            {t("sources.variants.groupTitle")}
          </h4>
          <p className="text-xs text-slate-400">
            {t("sources.variants.groupDescription")}
          </p>
        </div>
        <div className="flex flex-wrap gap-2">
          {equivalentGroups ? (
            <span className="rounded-full border border-emerald-800/80 bg-emerald-950/40 px-3 py-1 text-xs text-emerald-200">
              {t("sources.variants.equivalentGroups", { count: equivalentGroups })}
            </span>
          ) : null}
          <span className="rounded-full border border-slate-700 bg-slate-950/70 px-3 py-1 text-xs text-slate-300">
            {t("sources.variants.count", { count: variants.length })}
          </span>
        </div>
      </div>

      <div className="skill-markdown-scroll max-h-[36rem] space-y-4 overflow-y-auto pr-1">
        {groups.map((group) => {
          const groupLabel =
            group.agentKey === "manual"
              ? t("sources.variants.manualGroup")
              : agentLabel(group.agentKey);
          const fixedTarget = group.agentKey === "manual" ? null : group.agentKey;

          return (
            <section
              className="space-y-4 rounded-xl border border-slate-800 bg-slate-950 p-4"
              key={`${recordId}:${group.agentKey}`}
            >
              <header className="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                <div className="flex items-center gap-3">
                  <div className="grid h-10 w-10 shrink-0 place-items-center rounded-xl border border-slate-700 bg-slate-900">
                    {fixedTarget ? (
                      <AgentBrandIcon agentKey={fixedTarget} className="size-[70%]" />
                    ) : (
                      <CircleHelp className="h-5 w-5 text-slate-400" />
                    )}
                  </div>
                  <div>
                    <h5 className="text-sm font-semibold text-slate-100">{groupLabel}</h5>
                    <p className="text-xs text-slate-500">
                      {t("sources.variants.groupCounts", {
                        imports: group.imports.length,
                        variants: group.variants.length,
                      })}
                    </p>
                  </div>
                </div>
              </header>

              {group.variants.length ? (
                <div className="space-y-3">
                  {group.variants.map((variant) => {
                    const variantKey = `${group.agentKey}:${variant.agentKey}:${variant.variantPath}`;
                    const sameContentVariants = variantsWithSameContent(variant, variants);
                    const sameContentPaths = sameContentVariants
                      .slice(0, 3)
                      .map((item) => item.variantPath)
                      .join(", ");
                    const hiddenSameContentCount = Math.max(sameContentVariants.length - 3, 0);
                    const selectedTarget = selectedTargets[variantKey] ?? "";
                    const targetAgent = fixedTarget ?? selectedTarget;
                    const isImported = targetAgent
                      ? group.imports.some(
                          (item) =>
                            item.agentKey === targetAgent
                            && item.upstreamVariantPath === variant.variantPath,
                        )
                      : false;
                    const canImport = Boolean(targetAgent) && !isImported && !isBusy && Boolean(repoPath);

                    return (
                      <article
                        className="flex flex-col gap-3 rounded-xl border border-slate-800 bg-slate-900/40 p-3 lg:flex-row lg:items-start lg:justify-between"
                        key={variantKey}
                      >
                        <div className="min-w-0 space-y-2">
                          <div className="flex flex-wrap items-center gap-2">
                            <span className="rounded-full border border-slate-700 px-2 py-1 text-[11px] text-slate-300">
                              {t(`sources.variants.detection.${variant.detectionClass}`)}
                            </span>
                            {variant.detectedAgentHint ? (
                              <span className="rounded-full border border-sky-800/80 bg-sky-950/40 px-2 py-1 text-[11px] text-sky-200">
                                {t("sources.variants.hint", {
                                  agent: agentLabel(variant.detectedAgentHint),
                                })}
                              </span>
                            ) : null}
                            <span className="break-all text-sm font-medium text-slate-100">
                              {variant.variantPath}
                            </span>
                          </div>
                          {variant.sourceOfTruthPath ? (
                            <p className="text-xs text-slate-500">
                              {t("sources.variants.sourceOfTruth", {
                                path: variant.sourceOfTruthPath,
                              })}
                            </p>
                          ) : null}
                          {sameContentVariants.length ? (
                            <p className="text-xs text-emerald-200">
                              {t("sources.variants.sameContent", { paths: sameContentPaths })}
                              {hiddenSameContentCount
                                ? t("sources.variants.sameContentMore", {
                                    count: hiddenSameContentCount,
                                  })
                                : null}
                            </p>
                          ) : null}
                          <div className="space-y-2 text-xs text-slate-500">
                            {variantPreviewList(variant, variantKey, "directories", t)}
                            {variantPreviewList(variant, variantKey, "files", t)}
                          </div>
                        </div>

                        <div className="flex min-w-[13rem] flex-col gap-2">
                          {fixedTarget ? null : (
                            <label className="space-y-1">
                              <span className="text-[11px] font-medium uppercase tracking-wide text-slate-500">
                                {t("sources.variants.targetLabel")}
                              </span>
                              <select
                                className="w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 outline-none transition focus:border-sky-400"
                                onChange={(event) =>
                                  setSelectedTargets((current) => ({
                                    ...current,
                                    [variantKey]: event.target.value as AgentKey | "",
                                  }))
                                }
                                value={selectedTarget}
                              >
                                <option value="">{t("sources.variants.selectTarget")}</option>
                                {EXTERNAL_IMPORT_TARGETS.map((target) => (
                                  <option key={`${variantKey}:${target}`} value={target}>
                                    {agentLabel(target)}
                                  </option>
                                ))}
                              </select>
                            </label>
                          )}
                          <button
                            className="rounded-lg bg-sky-400 px-3 py-2 text-xs font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:bg-slate-800 disabled:text-slate-400"
                            disabled={!canImport}
                            onClick={() =>
                              targetAgent
                                ? void onImportVariant(recordId, targetAgent, variant.variantPath)
                                : undefined
                            }
                            type="button"
                          >
                            {!repoPath
                              ? t("sources.variants.repoRequired")
                              : !targetAgent
                                ? t("sources.variants.targetRequired")
                                : isImported
                                  ? t("sources.variants.imported")
                                  : isBusy
                                    ? t("sources.variants.importing")
                                    : t("sources.variants.importAs", {
                                        agent: agentLabel(targetAgent),
                                      })}
                          </button>
                        </div>
                      </article>
                    );
                  })}
                </div>
              ) : (
                <p className="text-sm text-slate-400">{t("sources.variants.noneDetected")}</p>
              )}

              <div className="space-y-3 border-t border-slate-800 pt-4">
                <h6 className="text-xs font-semibold uppercase tracking-wide text-slate-400">
                  {t("sources.variants.groupImportsTitle")}
                </h6>
                <ExternalImportList
                  imports={group.imports}
                  onRepairImport={onRepairImport}
                  onUpdateImport={onUpdateImport}
                  updatingImportId={updatingExternalImportId}
                />
              </div>
            </section>
          );
        })}
      </div>
    </section>
  );
}
