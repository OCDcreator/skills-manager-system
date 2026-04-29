# AddExternalSourceForm

> **Source**: `src/components/external-sources/AddExternalSourceForm.tsx`
> **Status**: [REVIEW]

## Overview

Owns the repo-URL entry form for adding one external GitHub source from the dedicated sources page.

## Public Surface

| Export | Purpose |
|---|---|
| `AddExternalSourceForm` | Controlled submit form for one repository URL. |

## Core Logic

The form keeps only local input state plus a transient submit-failed flag. It trims the entered URL before submit, ignores empty values, clears the field after a successful add, and leaves persistence plus backend validation to the parent callback.

## Interactions

Used only by `ExternalSourcesView`. Error copy and labels come from `sources.form.*` i18n keys, while real add/fetch behavior comes from `AppContext.addExternalSource()`.
