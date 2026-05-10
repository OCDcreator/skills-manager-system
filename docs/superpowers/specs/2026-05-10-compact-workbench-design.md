# Compact Workbench Responsive Design

## 1. Feature Summary

This phase introduces a unified compact workbench mode for `skills-manager-system` so the application remains effective on smaller desktop windows instead of merely squeezing the existing wide layout into less space.

The target is not “mobile styling” and not a one-off patch for one crowded screen. The goal is a coherent responsive model for the existing desktop workbench: preserve core state and primary actions in view, reduce layout pressure at medium widths, and move details or secondary controls into progressive disclosure before the UI becomes cramped.

The minimum width that must remain genuinely usable is **900px**.

## 2. Primary User Action

The single most important action is:

- open the app on a smaller desktop window
- understand the current page state immediately
- act on the main workflow without fighting the layout

In practice that means:

- navigation remains discoverable
- primary status and primary controls remain visible
- dense content remains scannable
- details and auxiliary controls yield space first

## 3. Design Direction

### Color strategy

Use the existing **Restrained** workbench palette.

- Keep the current warm dark foundation.
- Do not introduce a special “responsive mode” color language.
- Use hierarchy, spacing, grouping, and disclosure to solve crowding, not extra accent color.

### Theme scene sentence

A technical user is working in a smaller desktop window during a focused engineering session, still expects a serious workbench, and wants the interface to stay efficient without collapsing into noise or hiding the main task behind floating clutter.

### Anchor references

- existing `Skills Manager System` product and design context
- compact desktop tools such as `Raycast` and `Linear`
- operational control panels that preserve density through hierarchy, not through oversized cards

### Winning direction

The chosen direction is **global compact workbench mode**.

That means:

- responsiveness is defined at the shell level first
- `900px` triggers a real information-architecture shift, not just tighter spacing
- pages follow one shared compact logic instead of each inventing their own breakpoint behavior
- core workflow remains on-screen while detail, sidecars, floating aids, and helper copy step back

### Explicit user-approved tradeoff

At `900px`, the product should use a **balanced** strategy:

- core state and primary actions stay visible together when possible
- details, secondary controls, and large auxiliary regions may collapse, stack, or move behind intentional toggles
- readability must improve, but not by stripping the page down to a sparse single-purpose wizard

## 4. Scope

- **Fidelity:** implementation-ready product UI direction
- **Breadth:** app shell plus the first responsive rules for `Skills`, `Agents`, and `Scenes`, with baseline compact expectations for `Git`, `Projects`, `Sources`, and `Settings`
- **Behavior:** responsive structure, disclosure rules, and layout priority, not a visual rebrand
- **Target form factor:** smaller desktop windows, especially the `900px` to `1279px` range
- **Out of scope:** touch-first mobile redesign, feature removal, new business logic, or a new global state system for responsiveness

## 5. Responsive Model

The application should use three explicit layout bands.

### A. Wide workbench: `>= 1280px`

This is the current full desktop intent.

- full left navigation rail may remain
- dual-column layouts may remain
- sticky detail panels may remain where they add value
- floating support affordances may remain if they do not obscure content

This band is not the main design problem for this feature, but it remains the reference layout.

### B. Compact workbench: `900px` to `1279px`

This is the primary feature band and must be treated as its own product mode.

- the shell should stop spending desktop-scale width on persistent side structure
- pages should preserve workflow skeleton and main actions
- details should become collapsible, stacked, or on-demand
- tall, high-chrome cards should give way to tighter rows, grouped lists, or lower-height panels
- fixed or floating side aids should yield space before primary content does

Important rule:

- `900px` is not a compressed copy of the wide layout
- `900px` is a compact information architecture

### C. Narrow stacked: `< 900px`

This band accepts stronger stacking behavior.

- single-column composition is allowed and usually preferred
- top-level navigation may become a horizontal rail
- core page state and actions should still appear near the top
- only deliberate regions such as code, diff, path values, or nav rails may rely on horizontal scrolling

### D. Breakpoint migration strategy

The current shell already switches from the mobile header to the left sidebar at `768px`. This feature must replace that single handoff with a four-band shell map:

- **`>= 1280px`:** keep the existing full left rail behavior, including the current wide navigation treatment
- **`900px` to `1279px`:** use a **compact left icon rail** as the single approved compact-shell pattern
- **`768px` to `899px`:** keep the header-plus-horizontal-nav shell pattern, tightened for smaller desktop or tablet-like widths
- **`< 768px`:** continue using the same header-plus-horizontal-nav pattern with the smallest spacing budget

Important implementation rule:

- the existing `768px` desktop-sidebar breakpoint should migrate upward to `900px`
- the new `1280px` breakpoint expands the compact icon rail into the existing full rail
- the spec does **not** leave `900px` to `1279px` open as “icon rail or top nav”; the chosen design is the compact left icon rail

## 6. Global Layout Strategy

### A. Shell ownership

`AppShell` and the shared foundation layer must own the responsive model.

They should define:

- when the sidebar remains full-width
- when navigation becomes compact
- when content padding contracts
- when page chrome yields vertical space

Individual pages may choose how to reorder their internal regions, but they should not each define their own unrelated shell breakpoints.

### B. Space priorities

When width becomes constrained, the UI should yield space in this order:

1. decorative or generous spacing
2. helper copy and secondary descriptions
3. persistent detail panels
4. floating or sidecar assistance
5. duplicated controls

Core workflow content should yield last:

- current page identity
- main list or editor
- primary status
- primary action path

### C. Progressive disclosure

Compact mode should prefer:

- fold-down detail panels
- current-item expansion
- stacked sections
- explicit “show more” or “open details” controls

Compact mode should avoid:

- leaving a wide empty gutter for a panel that now contains little value
- overlapping floating affordances that cover the main task
- forcing the user into whole-page horizontal scrolling

### D. Typography and density

Do not solve crowding by simply shrinking text across the board.

- keep core body text readable
- preserve uppercase label behavior sparingly
- reduce vertical waste through structure, not by pushing typography below comfortable scanning size

## 7. Page-Level Layout Rules

### A. AppShell

The shell should provide three navigation states:

- **wide:** full left rail with brand, subtitle, and text labels
- **compact:** reduced-width **icon-only left rail** that keeps page switching obvious without consuming a large fixed column
- **narrow:** top brand plus horizontally scrollable nav rail

Rules:

- the current `16.5rem` fixed desktop sidebar should not continue unchanged down to `768px`
- the compact rail should become the default shell between `900px` and `1279px`
- the compact rail should rely on icons, hover titles, and active-state clarity instead of trying to squeeze bilingual text labels into a narrow column
- error banners must still be visible without stealing excessive vertical space
- the first content region should land closer to the top on compact widths
- focus outlines for mobile or compact navigation must remain unclipped

Compact component scheme:

- `>= 1280px`: existing full rail
- `900px` to `1279px`: icon-only rail, narrower than the full desktop rail, with the brand mark preserved but subtitle removed from persistent layout
- `< 900px`: header plus horizontally scrollable nav row

Auxiliary shell rules:

- `ProjectAssistantLauncher` must retreat with the shell, not float over the densest part of page content
- `UnsavedChangesDialog` remains modal, but its width must stay viewport-bounded on narrow widths and must not assume a wide desktop canvas

### B. Skills page

The page should stop behaving like “large cards plus a permanent detail column” in compact mode.

Compact priorities:

- search
- source and status filtering
- source grouping
- skill name
- enabled or disabled state
- recoverable path context

Compact concessions:

- the detail document panel becomes on-demand instead of permanently occupying a wide right column
- skill items become tighter rows or lower-height cards
- description preview is reduced in prominence before name, status, and path are sacrificed

At `900px` to `1279px`, a user should be able to scan many skills quickly without losing access to the selected skill’s detail when they explicitly ask for it.

Compact component scheme:

- `>= 1280px`: keep the current two-column pattern with a persistent detail region
- `900px` to `1279px`: replace the persistent right detail column with a **page-local right-side detail drawer** opened from the selected skill
- `< 900px`: selected skill detail becomes a **stacked full-width detail section** below the filters and list, not an always-open side column

The compact drawer should:

- open from the page workspace edge rather than as a blocking modal
- preserve access to the main list
- have a clear close action
- allow long markdown content to scroll internally

### C. Agents page

This page should preserve the “overview plus current editing workflow” while giving back space from auxiliary structure.

Compact priorities:

- sync summary and apply actions
- current agent selection and configuration editor
- target state preview that supports the current agent decision

Compact concessions:

- the fixed right floating agent rail should disappear in compact mode
- the page should not reserve right padding for a hidden floating control
- all-agent fully expanded editing is no longer the default compact presentation
- sidecar target lists may stack beneath the current agent card or collapse behind a current-agent focus state

The compact page should still feel like a workbench, but not like several desktop surfaces forced to coexist in one narrow viewport.

Compact component scheme:

- `>= 1280px`: keep the current overview plus floating-rail model
- `900px` to `1279px`: replace the floating rail with a **page-top horizontally scrollable agent tab bar**
- `< 900px`: keep the same tab-bar model, but allow stronger stacking and one-agent-at-a-time focus

Compact focus behavior:

- the summary panel remains near the top
- the selected agent becomes the primary editing surface
- sidecar target lists should stack below the selected agent card instead of insisting on desktop-style side-by-side occupancy
- non-selected agents may collapse to lighter summary rows or closed panels as long as switching remains obvious

The replacement agent tab bar must be:

- always visible near the top of the page
- horizontally scrollable when needed
- clear about the current active agent
- usable without the floating rail ever reappearing in compact mode

### D. Scenes page

The scenes screen should become structurally adaptive instead of relying on wide horizontal composition.

Compact priorities:

- scene list
- scene title and description
- scene skill count or high-level status
- clear entry into configuration for the active scene

Compact concessions:

- the create form becomes a responsive grid or stacked form instead of one fixed horizontal row
- detailed skill configuration and ordering are secondary to the overview list and may stay behind the active scene expansion
- helper text should tighten before inputs become cramped

Compact component scheme:

- `>= 1280px`: keep the current wider card and configuration rhythm
- `900px` to `1279px`: use a **two-column adaptive create form** where the action button may wrap onto its own row when space tightens
- `< 900px`: use a **single-column stacked create form**

Scene configuration behavior:

- only the currently configured scene should expose its dense configuration region by default in compact mode
- non-active scenes should stay summary-first
- long skill ordering or configuration blocks should appear inside the active scene expansion, not as always-open page clutter

### E. Git page

Minimum compact expectations:

- the page should collapse toward a single dominant working column before it preserves side-by-side diff or log structures
- status summary and primary Git actions stay near the top
- file list, diff viewer, log, and operation log may stack vertically in compact mode
- no compact Git layout may depend on a fixed-width secondary pane remaining visible

### F. Projects page

Minimum compact expectations:

- the saved-project list and the active project workbench must remain readable at `900px`
- wide statement panels or side-by-side assignment surfaces may stack
- the current project being edited should dominate the workspace; non-active projects can remain lighter summary surfaces
- per-agent project details may expand within the active project region rather than insisting on a wide two-column workstation

### G. Sources page

Minimum compact expectations:

- the source list remains the dominant structure
- add-source form and source detail regions may stack
- import, repair, and update controls remain reachable without side-by-side dependency
- no source-management action should require a wide persistent secondary panel

### H. Settings page

Minimum compact expectations:

- the repo path form and related controls stack into a single readable column
- long path values may wrap, truncate, or scroll within the field region, but not force whole-page overflow
- settings sections must preserve clear grouping even when all controls are in one column

### I. Shared page rule for remaining views

For any view not receiving a bespoke redesign in this phase:

- prefer one dominant column in compact mode
- do not depend on fixed-width companion panes
- keep primary actions above secondary metadata
- allow internal section stacking before text or controls become cramped

## 8. Interaction Model

### Shell flow

1. User narrows the window.
2. The shell transitions from wide to compact behavior at the shared breakpoint.
3. Navigation remains visible, but consumes less persistent width.
4. Page content gets the newly recovered width first.

Transition rule:

- breakpoint transitions should be **immediate**, not animated layout choreography
- this should match the current product’s calm, utility-first behavior and avoid visual jitter during desktop window resizing

### Page flow

1. User lands on a crowded data page.
2. Core status and main controls remain near the top.
3. The main list or editor remains the dominant surface.
4. Details or secondary regions are reachable through expansion, selection, or stacking rather than constant side occupancy.

### Important responsive invariant

- the page must never hide the main workflow simply to preserve a familiar wide-screen composition

## 9. State and Edge Cases

### Default compact state

- page identity is visible
- main action path is visible
- main content fills most of the width
- secondary regions are quieter or deferred

### Long bilingual labels

- labels may wrap where appropriate
- path or code-like content may truncate or scroll only within a controlled region
- no page should depend on unusually short English strings to stay usable
- compact left navigation should avoid bilingual width pressure by using icon-only persistent items in the `900px` to `1279px` band
- narrow horizontal nav rails may stay no-wrap and horizontally scroll
- chips and filter controls should wrap to additional lines before text is reduced below normal body legibility

### Empty and loading states

- empty-state panels should not become oversized blocks that dominate compact pages
- loading indicators should preserve the page rhythm and not introduce large dead areas

### Error banners

- errors remain obvious
- banners should use compact vertical padding in constrained widths
- errors should not permanently bury the main controls below a tall message block

### Very long lists

- long lists should favor internal scrolling regions or tighter repeated items
- list containers should not combine large fixed heights with oversized per-item cards in compact mode

### Floating controls

- a floating affordance that overlaps meaningful content in compact mode is a responsive failure

### Project assistant launcher

- `ProjectAssistantLauncher` may remain floating, but in compact mode it must use a safe inset and footprint that does not cover the main page action area
- if the default bottom-right position collides with compact page controls, the launcher should move or shrink before content is sacrificed

### Unsaved changes dialog

- `UnsavedChangesDialog` should remain centered and modal
- its content width should remain bounded by viewport width with comfortable side margins
- action buttons may wrap, but the action row must stay readable and tappable in narrow widths

## 10. Implementation Strategy

The implementation should be split into three coordinated units.

### A. Shell compact layer

Responsibility:

- define compact breakpoint behavior in `foundation.css`
- update `AppShell` structure only where necessary
- normalize spacing and navigation behavior across pages

### B. Page compaction rules

Responsibility:

- `Skills` adopts compact scanning over permanent detail occupancy
- `Agents` removes width-hungry auxiliary structure before sacrificing editor usefulness
- `Scenes` replaces fixed horizontal composition with adaptive stacked composition

### C. Responsive verification

Responsibility:

- extend existing layout contract tests
- assert the shell and key pages no longer rely on wide-only assumptions at compact widths
- protect the compact behavior from future regression

## 11. Testing Requirements

Implementation must remain aligned with the repo’s existing source-contract testing style.

Required verification areas:

- `AppShell` shell and breakpoint contracts
- `Skills` compact layout contracts
- `Agents` compact layout contracts, including floating-nav retreat
- `Scenes` compact form and section stacking contracts
- baseline compact contracts for `Git`, `Projects`, `Sources`, and `Settings` where this phase changes shared shell assumptions

Required final verification:

- targeted layout tests for the changed pages
- `npm run build`
- the repo’s current `npm run verify` gate before completion

Test style requirement:

- use the repo’s existing **source-contract** pattern, centered on DOM structure assertions, Tailwind or CSS contract assertions, and focused layout invariants
- this phase does **not** require screenshot-based visual regression before implementation can proceed

Concrete examples of acceptable responsive assertions:

- assert `foundation.css` no longer applies the `16.5rem` full sidebar grid in the `900px` compact band
- assert `AgentsView` does not keep right-side padding reserved for the floating rail when compact mode is active
- assert `SkillsView` compact layout no longer depends on the persistent `min-[1280px]` detail-column contract for mid-width operation
- assert compact navigation preserves focus ring padding and scroll safety

## 12. Non-Goals and Guardrails

- Do not redesign the product into a touch-first mobile app.
- Do not introduce a separate responsive state management framework unless a layout interaction absolutely requires local state.
- Do not replace dense workbench behavior with sparse wizard behavior.
- Do not solve crowding primarily by shrinking text.
- Do not preserve wide-screen side structures at the cost of main content usability.
- Do not let normal page content require full-page horizontal scrolling.

## 13. Acceptance Criteria

This feature is successful when:

- the app remains clearly usable at `900px`
- compact mode is governed by one shared shell model
- `Skills`, `Agents`, and `Scenes` each preserve their main workflow without feeling cramped
- detail and auxiliary surfaces retreat before core workflow surfaces do
- the responsive behavior is covered by the repo’s layout test approach

## 14. Recommended References

Implementation should lean most on:

- [PRODUCT.md](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/PRODUCT.md)
- [DESIGN.md](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/DESIGN.md)
- [AppShell.tsx](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/src/components/AppShell.tsx)
- [foundation.css](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/src/styles/foundation.css)
- [SkillsView.tsx](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/src/views/SkillsView.tsx)
- [AgentsView.tsx](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/src/views/AgentsView.tsx)
- [ScenesView.tsx](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/src/views/ScenesView.tsx)
- [app-shell-design.test.mjs](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/scripts/app-shell-design.test.mjs)
- [agent-layout.test.mjs](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/scripts/agent-layout.test.mjs)
- [scene-layout.test.mjs](/C:/Users/lt/Desktop/Write/custom-project/skills-manager-system/scripts/scene-layout.test.mjs)
