# AddExternalSourceForm

> **Source**: `src/components/external-sources/AddExternalSourceForm.tsx`
> **Status**: [REVIEW]

## Overview

Owns the repo-URL entry form for adding one external GitHub source from the dedicated sources page.

## Public Surface

| Export | Purpose |
|---|---|
| `AddExternalSourceForm` | Controlled submit form for one repository URL plus optional branch and repo subpath. |

## Core Logic

The form keeps only local input state plus a transient submit-failed flag. It trims the entered URL before submit, passes optional branch/subpath as nullable fields, ignores empty URL values, clears all fields after a successful add, and leaves persistence plus backend validation to the parent callback. The branch and subpath inputs are intentionally optional so the existing default-branch/root-repo flow remains unchanged.

## Interactions

Used only by `ExternalSourcesView`. Error copy and labels come from `sources.form.*` i18n keys, while real add/fetch behavior comes from `AppContext.addExternalSource()`.
