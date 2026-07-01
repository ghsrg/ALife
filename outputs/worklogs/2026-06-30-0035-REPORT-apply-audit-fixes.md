---
tags:
  - alife
  - worklog/report
---

# REPORT: apply audit fixes

Date: 2026-06-30 00:35

## Scope

Applied the agreed fixes for the first documentation inconsistency audit.

## Done

- Fixed Genome status: Direct Regulatory Graph is now the base Genome model, not a pending choice and not a final monolith.
- Removed MVP language from Canon documentation and converted phase wording to neutral "base model" / "future development" wording.
- Kept future implementation framing only in `docs/ROADMAP.md` as a non-Canon planning reference.
- Clarified Energy axiom:
  - active action without sufficient Energy does not execute;
  - mandatory costs are separate from planned actions;
  - if Energy is insufficient for all planned actions in a Tick, planned actions do not execute;
  - iteration order must not create random priority.
- Removed stale ADR placeholder sections and `ADR-000X` marker knowledge.
- Kept ADR only as governance/process and empty journal documentation.
- Clarified Space:
  - Space is a constraint similar to a resource, but not `ResourceType`;
  - added `Space` to glossary;
  - added `docs/world/space.md` to `docs/README.md` structure.
- Replaced neural-cell terminology with neural-like / signal-plastic wording where relevant.
- Clarified Energy Buffer during division as partition of one cell's local state, not Energy transport between independent cells.
- Clarified Heat/temperature:
  - cells/objects have local temperature;
  - Heat is transfer/effect;
  - base model transfers Heat only through contact or Joint;
  - global Heat field is future model only.
- Clarified Tick vs Scheduler:
  - Tick is world time;
  - Scheduler is engine execution optimization and may not match conceptual Tick phases one-to-one.
- Clarified Organism:
  - world does not know `Organism` as active entity;
  - observer/debug/research layers may derive `OrganismView`.
- Kept status registry centralized in `docs/ROADMAP.md`; removed local `# Статус` sections from research files.

## Verification

Ran control searches for:

- `MVP`
- `ADR-000X`
- local `# Статус` / `## Статус`
- `нейронн`
- `partial execution` / `часткове виконання`
- stale Energy partial-consumption examples
- stale Space-as-Resource wording
- stale `перша реалізація` wording

Remaining expected matches:

- `docs/ROADMAP.md` contains the only documentation status registry.
- `docs/README.md`, `docs/STYLE_GUIDE.md`, `docs/GLOSSARY.md`, and `docs/decisions/README.md` still mention ADR as process/journal terminology, not accepted placeholder decisions.

## Notes

Broad wording cleanup touched many Markdown files because stale MVP/first-implementation language was spread across Canon, Research, Config, and Engine docs.

Link audit was intentionally not performed here; it remains part of the later Obsidian/linking audit.
