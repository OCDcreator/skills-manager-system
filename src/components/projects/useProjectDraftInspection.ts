import { useEffect, useMemo, useRef, useState } from "react";
import {
  inspectProjectAssignmentPath,
  type ProjectPathInspection,
} from "../../lib/projects";

export function useProjectDraftInspection(projectPath: string, agentKeys: string[]) {
  const [inspection, setInspection] = useState<ProjectPathInspection | null>(null);
  const [isInspecting, setIsInspecting] = useState(false);
  const [inspectionError, setInspectionError] = useState<string | null>(null);
  const requestIdRef = useRef(0);
  const agentKeySignature = useMemo(() => agentKeys.join("\n"), [agentKeys]);

  useEffect(() => {
    const trimmedPath = projectPath.trim();
    if (!trimmedPath) {
      setInspection(null);
      setInspectionError(null);
      setIsInspecting(false);
      return;
    }

    const requestId = ++requestIdRef.current;
    const timeoutId = window.setTimeout(() => {
      setIsInspecting(true);
      void inspectProjectAssignmentPath(trimmedPath, agentKeys)
        .then((result) => {
          if (requestId !== requestIdRef.current) {
            return;
          }
          setInspection(result);
          setInspectionError(null);
        })
        .catch((error) => {
          if (requestId !== requestIdRef.current) {
            return;
          }
          setInspection(null);
          setInspectionError(error instanceof Error ? error.message : String(error));
        })
        .finally(() => {
          if (requestId === requestIdRef.current) {
            setIsInspecting(false);
          }
        });
    }, 250);

    return () => window.clearTimeout(timeoutId);
  }, [agentKeys, agentKeySignature, projectPath]);

  return { inspection, isInspecting, inspectionError };
}
