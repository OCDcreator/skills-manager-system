---
name: Skills Manager System
description: A calm desktop workbench for managing AI coding skills across agents, scenes, projects, and Git sync.
colors:
  workbench-bg: "oklch(20% 0.018 82)"
  workbench-panel: "oklch(25% 0.016 83)"
  workbench-panel-strong: "oklch(30% 0.018 82)"
  workbench-border: "oklch(42% 0.018 82)"
  workbench-text: "oklch(90% 0.018 85)"
  workbench-muted: "oklch(68% 0.018 86)"
  workbench-accent: "oklch(73% 0.13 142)"
  workbench-accent-ink: "oklch(21% 0.024 142)"
  workbench-danger: "oklch(64% 0.17 25)"
  legacy-surface: "#0f172a"
  legacy-panel: "#111827"
  legacy-border: "#334155"
  legacy-accent: "#38bdf8"
typography:
  display:
    fontFamily: "Aptos, Aptos Display, Segoe UI, SF Pro Text, Helvetica Neue, sans-serif"
    fontSize: "1.05rem"
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: "0"
  title:
    fontFamily: "Aptos, Aptos Display, Segoe UI, SF Pro Text, Helvetica Neue, sans-serif"
    fontSize: "1rem"
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "0"
  body:
    fontFamily: "Aptos, Aptos Display, Segoe UI, SF Pro Text, Helvetica Neue, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: "0"
  label:
    fontFamily: "Aptos, Aptos Display, Segoe UI, SF Pro Text, Helvetica Neue, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 600
    lineHeight: 1.35
    letterSpacing: "0.18em"
rounded:
  sm: "0.5rem"
  md: "0.75rem"
  lg: "0.9rem"
  xl: "1rem"
  panel: "1.5rem"
  statement-panel: "1.75rem"
  pill: "999px"
spacing:
  xs: "0.5rem"
  sm: "0.75rem"
  md: "1rem"
  lg: "1.5rem"
  xl: "2rem"
components:
  button-primary:
    backgroundColor: "{colors.workbench-accent}"
    textColor: "{colors.workbench-accent-ink}"
    rounded: "{rounded.lg}"
    padding: "0.75rem 1.25rem"
  button-secondary:
    backgroundColor: "{colors.workbench-panel-strong}"
    textColor: "{colors.workbench-text}"
    rounded: "{rounded.lg}"
    padding: "0.625rem 1rem"
  input-field:
    backgroundColor: "{colors.legacy-surface}"
    textColor: "{colors.workbench-text}"
    rounded: "{rounded.xl}"
    padding: "0.75rem 1rem"
  nav-item-active:
    backgroundColor: "{colors.workbench-panel-strong}"
    textColor: "{colors.workbench-text}"
    rounded: "{rounded.lg}"
    padding: "0.55rem 0.75rem"
  status-chip:
    backgroundColor: "{colors.workbench-panel-strong}"
    textColor: "{colors.workbench-text}"
    rounded: "{rounded.pill}"
    padding: "0.25rem 0.625rem"
  data-card:
    backgroundColor: "{colors.legacy-surface}"
    textColor: "{colors.workbench-text}"
    rounded: "{rounded.xl}"
    padding: "1rem"
---

# Design System: Skills Manager System

## 1. Overview

**Creative North Star: "The Calibrated Workbench"**

This interface should feel like a precise desktop workbench for a technical
operator: calm, legible, and ready for repeated use. The visual system is dark
because the app is usually used during focused engineering sessions with dense
configuration, paths, diffs, and terminal-adjacent content. It is not dark for
spectacle. The surface should keep glare low while preserving clear contrast and
fast scanning.

The system rejects SaaS landing-page drama, decorative AI-tool styling, and
settings-page sprawl. Screens may be information-rich, but they must never feel
cramped. Favor explicit state, aligned controls, durable panels, and inline
previews over ornamental cards or promotional composition.

**Key Characteristics:**

- Warm dark workbench surfaces with one restrained green primary accent.
- Compact expert density with real breathing room between functional regions.
- Flat-by-default surfaces, using border and tone before shadow.
- Direct action language and visible ownership labels.
- Clear separation between configuration, preview, and filesystem mutation.

## 2. Colors

The palette is a warm, low-chroma dark workbench with a rare green accent and a
legacy slate/sky compatibility layer still present in older components.

### Primary

- **Bench Signal Green** (`workbench-accent`): Used for the brand mark, focus
  outlines, active navigation border, and the clearest primary affordances. It
  should stay rare enough to mean "current, ready, or important."
- **Bench Signal Ink** (`workbench-accent-ink`): Used only when text sits on
  Bench Signal Green.

### Secondary

- **Legacy Sky Blue** (`legacy-accent`): Existing Tailwind-era action color used
  in older buttons, form focus states, scrollbars, assistant affordances, and
  selected-card states. Treat it as a compatibility accent, not a second brand
  voice.

### Tertiary

- **Warning Amber**: Existing Tailwind amber treatments identify risky but
  recoverable states such as duplicates, unmanaged entries, or discard actions.
- **Failure Rose**: Existing rose treatments identify destructive or failed
  states. Use it for errors and deletion risk only.

### Neutral

- **Workbench Background** (`workbench-bg`): The app canvas.
- **Workbench Panel** (`workbench-panel`): Sidebar and mobile header surfaces.
- **Workbench Panel Strong** (`workbench-panel-strong`): Hover, selected nav,
  raised headers, and emphasized panel regions.
- **Workbench Border** (`workbench-border`): Structural dividers and focus-safe
  outlines between dense regions.
- **Workbench Text** (`workbench-text`): Primary text.
- **Workbench Muted** (`workbench-muted`): Secondary text, descriptions, and
  helper labels.
- **Legacy Surface** (`legacy-surface`): Deep slate content cells, inputs,
  markdown code blocks, and skill cards.

### Named Rules

**The One Signal Rule.** Bench Signal Green is the primary system voice. Do not
let sky, cyan, amber, violet, and rose all compete on the same screen.

**The State Color Rule.** Use color to label state, not to decorate. If a color
does not tell the user what changed, what is selected, or what is risky, remove
it.

## 3. Typography

**Display Font:** Aptos / Aptos Display, falling back to Segoe UI, SF Pro Text,
Helvetica Neue, and system sans-serif.
**Body Font:** Aptos / Segoe UI compatible sans-serif stack.
**Label/Mono Font:** Cascadia Code, Fira Code, JetBrains Mono, SFMono-Regular,
Consolas, Liberation Mono, Menlo, monospace for markdown code and terminal-like
content.

**Character:** The type should feel quiet and operational. It should support
long paths, bilingual labels, dense lists, and repeated scanning without becoming
tiny or decorative.

### Hierarchy

- **Display** (700, `1.05rem`, `1.1`): App title and shell identity only.
- **Headline** (600-700, `1.125rem` to `1.25rem`, `1.25`): Page and primary
  panel headings.
- **Title** (600, `0.875rem` to `1rem`, `1.25`): Card titles, section titles,
  and list group headings.
- **Body** (400, `0.875rem`, `1.5`): Descriptions, field values, and repeated
  row content. Keep body copy short and cap long prose around 65-75ch.
- **Label** (600, `0.75rem`, uppercase with `0.18em` to `0.22em` tracking):
  Metadata labels, filter group names, and path field labels.

### Named Rules

**The Quiet Label Rule.** Uppercase labels are for orientation, not decoration.
They must be short, muted, and paired with stronger content beneath them.

**The Path Legibility Rule.** Paths and skill IDs must remain readable before
they look elegant. Use wrapping, truncation, or scroll only where the owning
surface makes the full value recoverable.

## 4. Elevation

This system is flat by default. Depth comes first from tonal surfaces, borders,
sticky regions, and internal scroll. Shadows appear only on overlays, floating
assistant controls, statement panels, or interactive project identity regions
where separation from dense content is necessary.

### Shadow Vocabulary

- **Overlay Lift** (`shadow-2xl` / `shadow-slate-950/40`): Dialogs, floating
  launchers, and assistant affordances.
- **Statement Panel Lift** (`0 18px 60px rgba(2,6,23,0.34)`): Large project
  workbench identity panels that need to read as a focused input station.
- **Inset Field Pressure** (`inset 0 1px 0 rgba(148,163,184,0.08)` or slate
  inner shadows): Markdown code blocks, custom scroll tracks, and path fields.

### Named Rules

**The Flat First Rule.** Do not add a shadow when a border, background tone, or
spacing change can separate the element.

**The Overlay Exception Rule.** Heavy shadows belong to overlays and floating
controls. They do not belong on every repeated card.

## 5. Components

### Buttons

- **Shape:** Functional curve, usually 8-16px (`0.5rem` to `1.05rem`).
- **Primary:** Use Bench Signal Green or the existing sky action color for the
  single highest-value action in a region. Padding is usually `0.75rem 1rem` or
  `0.75rem 1.25rem`.
- **Hover / Focus:** Hover changes tone or border only. Focus uses a visible
  2px outline or a clear border shift. Do not animate layout properties.
- **Secondary / Ghost:** Use panel-strong or transparent backgrounds with slate
  borders. Secondary actions should be calm and not compete with primary action.
- **Icon Buttons:** Use familiar Lucide icons with `title` labels. Keep them
  compact and reserve text labels for commands that need clarity.

### Chips

- **Style:** Pill radius (`999px`), small horizontal padding, low-alpha status
  backgrounds, and readable colored text.
- **State:** Selected chips may use accent-filled backgrounds; unselected chips
  should stay slate and quiet. Chips must label state, source, selection, or
  ownership.

### Cards / Containers

- **Corner Style:** Repeated item cards use 12-24px curves (`0.75rem` to
  `1.5rem`). Large statement panels may reach `1.75rem`.
- **Background:** Use Workbench Panel, Workbench Panel Strong, or Legacy Surface
  depending on hierarchy.
- **Shadow Strategy:** Repeated cards stay flat. Use borders and tone. Large
  workbench panels may use one structural shadow.
- **Border:** Slate borders are the default structure. Do not use colored side
  stripes.
- **Internal Padding:** `1rem` for dense repeated cards, `1.25rem` to `1.5rem`
  for primary workbench panels.

### Inputs / Fields

- **Style:** Deep surface, slate border, 12-20px curve, and compact padding.
  Standard input padding is `0.75rem 1rem`; dense inputs may use `0.5rem 0.75rem`.
- **Focus:** Border shifts to sky or Bench Signal Green. Keep outline visible
  and never hide focus for mouse polish.
- **Error / Disabled:** Error fields use rose text and border. Disabled fields
  reduce opacity but must remain legible.

### Navigation

Desktop navigation is a left rail with icon plus text. Mobile navigation becomes
a horizontal scroll rail with real outline buffer: negative outer margin,
positive internal padding, and hidden scrollbar. Active state uses Workbench
Panel Strong mixed with the primary accent, not a full saturated fill.

### Markdown / Code Viewers

Markdown surfaces are dark and code-forward. Code blocks use Legacy Surface,
slate borders, 12px radius, mono font, and thin custom scrollbars. Preserve
horizontal scrolling for code and table content.

### Project Workbench Panels

Project identity panels are the strongest visual modules. They may use a subtle
dark gradient, 28px corners, inset fields, and one structural shadow. This
pattern should remain reserved for focused project setup, not every page section.

## 6. Do's and Don'ts

### Do:

- **Do** show state before action: previews, counts, managed/unmanaged labels,
  stale references, dirty markers, and target paths should appear before apply
  buttons.
- **Do** keep configuration and mutation distinct. Editing a scene or project
  assignment should not look like applying it to disk.
- **Do** use borders, tone, grouping, and concise labels to make dense screens
  readable.
- **Do** keep primary actions rare per region, with one clear action color.
- **Do** preserve keyboard focus states, bilingual label fit, and scroll regions
  that do not clip outlines or content.

### Don't:

- **Don't** make it feel like a SaaS landing page, decorative AI product demo,
  or marketing dashboard.
- **Don't** overuse cards. Do not nest cards inside cards. Use cards for
  repeated items, modals, and genuinely framed tools.
- **Don't** make the interface crowded. Avoid settings-page sprawl where every
  control has the same visual weight.
- **Don't** use ornamental gradients, vanity metrics, hero sections, or
  promotional copy inside the app shell.
- **Don't** use colored side stripes, gradient text, decorative glassmorphism,
  or full-screen accent color unless a future task explicitly changes the design
  register.
- **Don't** animate layout properties. State changes should be quick, calm, and
  non-disruptive.
