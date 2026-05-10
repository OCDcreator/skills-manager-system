# AddExternalSourceForm

> **Source**: `src/components/external-sources/AddExternalSourceForm.tsx`
> **Status**: [REVIEW]

## Overview

Owns the add-source form for one external repository record on the dedicated sources page.

## Public Surface

| Export | Purpose |
|---|---|
| `AddExternalSourceForm` | Controlled submit form for one repository URL plus optional branch and repo subpath. |

## Core Logic

The form keeps only local draft state plus a transient `submitFailed` flag. It trims the entered URL before submit, passes optional branch/subpath as nullable fields, ignores empty URL values, clears all fields after a successful add, and leaves persistence plus backend validation to the parent callback. If submit fails, the current draft stays intact so the user can correct or retry without retyping.

The layout follows the compact workbench baseline: below `lg` the inputs and submit button stack naturally; at `lg` the form becomes one dense grid row with repo URL, branch, subpath, and action aligned together. There is no special `900px-1279px` branch beyond that shared stacked behavior.

## Interactions

Used only by `ExternalSourcesView`. Error copy and labels come from `sources.form.*` i18n keys, while real add behavior comes from `AppContext.addExternalSource()`.
