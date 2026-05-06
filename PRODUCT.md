# Product

## Register

product

## Users

Skills Manager System is for a technical owner who maintains a local `my-skills`
repository and routes those skills into multiple AI coding agents across Windows
and macOS. The user is usually in the middle of real engineering work: checking
what changed, deciding which skills belong to which agent, composing reusable
toolkits, and applying filesystem sync only when the target state is clear.

The product should also support future maintainers who need to understand the
current configuration quickly without reading implementation files or running
manual scripts.

## Product Purpose

The app is a lightweight desktop workbench for skill repository operations:
Git sync, skill browsing, per-agent assignment, reusable scene toolkits,
project-level overlays, external source inspection, and local assistant
workflows.

Success means the user can see the current state, preview the consequences of a
change, and apply a scoped sync with confidence. Editing configuration should
feel separate from mutating target directories. The interface should make stale
references, unmanaged target content, missing paths, and unsaved changes visible
before they become risky filesystem operations.

## Brand Personality

Steady, clear, professional.

The voice should feel like a careful desktop workbench: concise labels, concrete
state, and direct actions. It should have enough polish to feel intentional, but
it should never compete with the user's task. The product earns trust by making
complex sync rules legible.

## Anti-references

Do not make it feel like a SaaS landing page, a decorative AI product demo, or a
marketing dashboard. Avoid oversized hero sections, promotional copy, vanity
metrics, and ornamental gradients.

Do not overuse cards. Cards are appropriate for repeated items such as agents,
skills, scenes, projects, and confirmations, but page structure should rely on
clear bands, lists, toolbars, tables, split panes, and workbench regions instead
of nesting cards inside cards.

Do not make the interface crowded. Dense information is useful here, but it must
remain scannable through grouping, alignment, calm spacing, visible state, and
progressive disclosure. Avoid settings-page sprawl where every control has the
same visual weight.

## Design Principles

1. Show state before action.
   Every sync, apply, import, deletion, or takeover flow should reveal what will
   change and what will be preserved before the user commits.

2. Keep configuration and mutation distinct.
   Browsing, selecting, and editing toolkits should not look or behave like they
   immediately write to agent or project directories.

3. Prefer workbench clarity over decoration.
   The app should feel calm and capable. Visual polish should improve scanning,
   comparison, and confidence, not announce itself.

4. Make layered ownership visible.
   Global selections, scene-derived skills, project additions, exclusions,
   unmanaged target content, and stale references should have clear source
   labels and predictable placement.

5. Stay compact without becoming cramped.
   The interface should support repeated expert use with efficient controls,
   but spacing, hierarchy, and disclosure should keep each screen readable.

## Accessibility & Inclusion

The interface should target WCAG AA contrast, keyboard-accessible controls,
visible focus states, and labels that remain understandable in both English and
Chinese. Motion should be subtle and avoid layout animation. Screens should
remain usable at desktop sizes and constrained mobile-width Tauri windows, with
horizontal overflow reserved only for deliberate scroll regions such as nav
rails or code/diff panes.
