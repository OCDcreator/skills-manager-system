# i18n Index

> **Source**: `src/i18n/index.ts`
> **Status**: [REVIEW]

## Overview

Initializes i18next and react-i18next for English and Chinese UI resources.

## Import Relationships

```text
Upstream: src/main.tsx
Downstream: src/i18n/en.json, src/i18n/zh.json
```

## Public Surface

| Export | Purpose |
|---|---|
| `i18n` | Default initialized i18next instance. |

## Core Logic

The module registers `initReactI18next`, loads `en` and `zh` translation resources, selects Chinese when `navigator.language` starts with `zh`, and falls back to English.

## Data Flow

Locale JSON files become the translation resources consumed by `useTranslation` throughout the UI.

## Interactions

Any new UI text must update both `src/i18n/en.json` and `src/i18n/zh.json`.

## Configuration

`escapeValue` is disabled because React escapes rendered values.

## Change Notes

If locale persistence is added, update this initialization and document how saved locale overrides browser language.
