# Agent Order Modal UI Refinement Design

## 1. Feature Summary

This phase refines the existing Agent ordering modal into a more branded, more controlled workbench surface. The goal is not to change the ordering model itself, but to redesign the window so large agent inventories feel organized instead of long, while unsaved changes and the side-rail visibility setting become easier to understand at a glance.

The surface is still a product tool, not a concept showcase. It should feel more intentional and more memorable than the current modal, but still read as a precise desktop control panel that belongs inside `skills-manager-system`.

## 2. Primary User Action

The single most important action is:

- scan the enabled and disabled groups quickly
- reorder agents within each group with confidence
- notice that the draft has changed and still needs saving

## 3. Design Direction

### Color strategy

Use a **Restrained** palette.

- Keep the existing dark workbench foundation.
- Use a narrow cold-blue signal for active drag focus, selected emphasis, and the primary save action.
- Avoid broad glow, decorative gradients, or multiple competing accents.

### Theme scene sentence

A technical user is deep in a focused desktop workflow at night, opening a short-lived system dialog to calibrate agent order without losing momentum, and expects high control with zero visual noise.

### Anchor references

- `Linear` for precision and hierarchy
- `Raycast` for compact tool-window confidence
- local desktop workbench behavior over SaaS modal polish

### Winning direction

The chosen direction is **A · Precision Rail** from the visual companion exploration.

That means:

- sharper hierarchy than the current modal
- cleaner list geometry
- stronger separation between control bands and sortable rows
- brand feel through exact spacing, proportion, and emphasis, not spectacle

### Refinement overrides confirmed by the user

- agent rows remain **grouped into enabled and disabled sections**
- dragging is **group-local only**
- the title area adds a **light dirty marker** when draft order changes
- the modal has a **clear max height**
- long lists scroll **inside the modal body**
- scrollbars must reuse the app's **existing scrollbar skin**

## 4. Scope

- **Fidelity:** high-fi, implementation-ready UI direction
- **Breadth:** one modal window plus its relationship to the fixed right-side icon rail
- **Interactivity:** shipped-quality component behavior, not static-only mockups
- **Time intent:** finalize the refined modal spec so implementation can begin immediately after review

## 5. Layout Strategy

The modal should become a compact vertical workbench with four clear layers.

### A. Header band

The header owns orientation and status.

It contains:

- the order icon and title
- one short explanatory sentence
- the close button
- a lightweight dirty marker when the order has changed but not been saved

The header should not feel verbose. The dirty marker is not a warning banner. It is a quiet but visible “draft changed” confirmation, likely a small chip or inline status token aligned near the title block.

### B. Control band

The `show only enabled agents in the side rail` setting should sit in its own calibration band immediately below the header.

This band should:

- feel separate from sortable content
- explain that it affects the right-side floating rail, not group membership
- look like a system mode control rather than a generic checkbox row

It should be compact and visually calm, with enough structure that it reads as a persistent preference surface.

### C. Sort body

The sortable area is the main stage and should absorb almost all extra height.

Structure:

- `Enabled agents` section
- `Disabled agents` section

Each section includes:

- a small uppercase section label
- a short helper note such as `drag within this group`
- a vertical list of consistent, high-density agent rows

Rows should use:

- a dedicated drag handle zone
- brand icon circle
- display name
- state line

The active drag target or current focus row gets a sharper emphasis treatment, but should not shift dramatically or feel animated for show.

### D. Fixed footer

The footer remains pinned at the bottom of the modal.

It contains:

- a secondary `Cancel` button
- a primary `Save order` button

When the draft is dirty, the footer should reinforce that state, either through a compact status line or through subtle change in primary-action context. It should never require the user to scroll to discover the save action.

## 6. Key States

### Default

- enabled and disabled sections are immediately visible
- rows feel compact and aligned
- setting band is readable but secondary
- footer is always present

### Dirty draft

- title area shows a light dirty marker
- save action becomes clearly meaningful
- the modal communicates “you have changed the order” without turning into a warning dialog

### Long list

- modal shell has a viewport-bounded `max-height`
- only the sortable body scrolls
- header, control band, and footer stay stable
- scrollbar uses the same visual treatment already used by shared scroll regions in the app

### Dragging

- dragged row remains visually legible
- target row or drop area gets a crisp highlight
- group structure remains visible during drag
- interaction does not suggest cross-group movement

### Section with few items

- section remains structurally intact
- spacing does not collapse into an awkward empty form

### Very large inventory

- the modal must avoid becoming a tall uninterrupted column
- the sections should still feel intentionally chunked
- long-scroll fatigue is reduced by the stable header/footer and strong section framing

## 7. Interaction Model

1. User opens the modal from the floating icon rail.
2. The header explains the invariant: enabled agents render before disabled agents.
3. The user may toggle the side-rail visibility preference in the control band.
4. The user reorders rows by dragging **within the current section only**.
5. As soon as order changes, a dirty marker appears in the header.
6. The footer stays available regardless of scroll position.
7. Save persists the new order. Cancel abandons the draft.

Important rule:

- the modal does **not** let the user drag an item from enabled into disabled or vice versa
- the modal explains grouping through layout, not through long warning copy

## 8. Content Requirements

### Title region

- `Agent 顺序`
- supporting sentence that states the enabled-before-disabled rule once
- lightweight dirty label, such as `顺序已更改` or similar concise wording

### Control band

- short title for the preference
- one sentence that clarifies the right-side icon rail linkage

### Section labels

- `已启用 Agent`
- `已禁用 Agent`

Helper notes should stay short and operational. Avoid repeating full drag instructions in every row.

### Row copy

Each row needs:

- display name
- state label
- no extra noisy metadata unless required for disambiguation

### Footer

- `取消`
- `保存顺序`

Avoid theatrical button copy. The tool should sound precise.

## 9. Recommended References

Implementation should lean most on:

- `layout`: to solve long-list pressure, internal scroll, and fixed footer/header structure
- `polish`: to sharpen the modal into a branded tool surface without visual excess
- `harden`: to ensure long inventories, scroll behavior, and dirty-state clarity stay production-safe

## 10. Open Questions

No blocking product questions remain.

Implementation-level choices still open to engineering judgment:

- exact wording of the dirty marker in Chinese and English
- whether the dirty marker is chip-style or inline label-style
- whether the footer echoes dirty state with a compact status line or only through button emphasis

## Recommended Implementation Bias

Build this as a refinement of the current modal, not a replacement with a wholly new interaction model.

Keep:

- the current ordering model
- the current two-group mental model
- the current fixed-width desktop modal framing

Refine:

- visual hierarchy
- row geometry
- stable height behavior
- dirty-state communication
- side-rail preference framing
- scrollbar consistency
